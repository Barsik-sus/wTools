//! Implementation of genetic algorithm for sudoku solving.
//! 
//! Initial population is generated by randomly filling every block in sudoku board with unique numbers.
//! 
//! Fitness is calculated as number of errors in board.
//!
//! New population is generated partially from fittest individuals( percent is determined by elite_selection_rate ),
//! partially from randomly chosen individuals( percent is determined by random_selection_rate ) and rest is generated
//! as offspring of crossover of random pair of individuals, selected by tournament method.
//! 
//! Tournament is performed by randomly selecting a group of individuals from the population( the number of individuals selected is equal to the tournament_size value).
//! Likelihood of win of the fittest participant is determined by tournament_selection_pressure.
//! 
//! Crossover is performed by combining blocks from parents' boards, split in several randomly chosen crossover points.
//! 
//! New population is modified by appling mutation to some individuals in the population. Individual's likelihood of being mutated id determined by mutation_rate value.
//! 
//! Termination: process is stopped if sudoku solution is found or if max_generation_number value is exseeded.
//! 

use std::{ collections::HashSet, fmt::Debug };

use deterministic_rand::Hrng;
use iter_tools::Itertools;
use rand::{ seq::SliceRandom, Rng };

use crate::{ sudoku::*, optimization::* };

/// Functionality of crossover genetic operator.
pub trait CrossoverOperator : Debug
{
  type Person : Individual + Clone;
  /// Produce new Individual using genetic matherial of two selected Individuals.
  fn crossover( &self, hrng : Hrng, parent1 : &Self::Person, parent2 : &Self::Person ) -> Self::Person;
}

/// Crossover is performed by combining blocks from parents' boards, split in several randomly chosen crossover points.
#[ derive( Debug ) ]
pub struct MultiplePointsBlockCrossover {}

impl CrossoverOperator for MultiplePointsBlockCrossover
{
  type Person = SudokuPerson;
  fn crossover( &self, hrng : Hrng, parent1 : &Self::Person, parent2 : &Self::Person ) -> Self::Person
  {
    let rng_ref = hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();

    let possible_values = [ 1, 2, 3, 4, 5, 6, 7, 8 ];
    let first_parent_blocks_number = possible_values.choose( &mut *rng ).unwrap();
    let mut first_parent_blocks : HashSet< BlockIndex > = HashSet::new();

    while first_parent_blocks.len() != *first_parent_blocks_number
    {
      first_parent_blocks.insert( rng.gen() );
    }

    let mut child_storage: Vec< CellVal > = vec![ 0.into(); 81 ];

    for i in parent1.board.blocks()
    {
      if first_parent_blocks.contains( &i )
      {
        let parent_block = parent1.board.block( i ).collect_vec();
        let cells = parent1.board.block_cells( i );
        for ( index, cell_index ) in cells.enumerate()
        {
          child_storage[ usize::from( cell_index ) ] = parent_block[ index ];
        }
      }
      else 
      {
        let parent_block = parent2.board.block( i ).collect_vec();
        let cells = parent2.board.block_cells( i );
        for ( index, cell_index ) in cells.enumerate()
        {
          child_storage[ usize::from( cell_index ) ] = parent_block[ index ];
        }
      }
    }
    

    let child = SudokuPerson::with_board( Board::new( child_storage ) );
    child
  }
}

/// Crossover performed by selecting blocks with best rows or columns from two Individuals.
#[ derive( Debug ) ]
pub struct BestRowsColumnsCrossover {}

impl CrossoverOperator for BestRowsColumnsCrossover
{
  type Person = < SudokuInitial as SeederOperator >::Person;
  
  fn crossover( &self, _hrng : Hrng, parent1 : &Self::Person, parent2 : &Self::Person ) -> Self::Person
  {
    let mut rows_costs = vec![ Vec::new(); 2 ];
    let mut columns_costs = vec![ Vec::new(); 2 ];
    for ( index, parent ) in [ parent1, parent2 ].iter().enumerate()
    {
      rows_costs[ index ] = parent.board
      .rows()
      .map( | row | row.collect::< HashSet< _ > >().len() )
      .collect_vec()
      .chunks( 3 )
      .map( | costs | 27 - costs.iter().fold( 0, | acc, cost | acc + cost ) )
      .collect_vec()
      ;

      columns_costs[ index ] = parent.board
      .cols()
      .map( | row | row.collect::< HashSet< _ > >().len() )
      .collect_vec()
      .chunks( 3 )
      .map( | costs | 27 - costs.iter().fold( 0, | acc, cost | acc + cost ) )
      .collect_vec()
      ;
    }

    let mut child1_storage = vec![ CellVal::from( 0 ); 81 ];
    for i in 0..3
    {
      if rows_costs[ 0 ][ i ] < rows_costs[ 1 ][ i ]
      {
        for j in 0..3
        {
          let parent_block = parent1.board.block( BlockIndex::from( ( j as u8, i as u8 ) ) ).collect_vec();
          let cells = parent1.board.block_cells( BlockIndex::from( ( j as u8, i as u8 ) ) );
          for ( index, cell_index ) in cells.enumerate()
          {
            child1_storage[ usize::from( cell_index ) ] = parent_block[ index ];
          }
        }
      }
      else
      {
        for j in 0..3
        {
          let parent_block = parent2.board.block( BlockIndex::from( ( j as u8, i as u8 ) ) ).collect_vec();
          let cells = parent2.board.block_cells( BlockIndex::from( ( j as u8, i as u8 ) ) );
          for ( index, cell_index ) in cells.enumerate()
          {
            child1_storage[ usize::from( cell_index ) ] = parent_block[ index ];
          }
        }
      }
    }

    let mut child2_storage = vec![ CellVal::from( 0 ); 81 ];
    for i in 0..3
    {
      for j in 0..3
      {
        if columns_costs[ 0 ][ j ] < columns_costs[ 1 ][ j ]
        {
          let parent_block = parent1.board.block( BlockIndex::from( ( j as u8, i as u8 ) ) ).collect_vec();
          let cells = parent1.board.block_cells( BlockIndex::from( ( j as u8, i as u8 ) ) );
          for ( index, cell_index ) in cells.enumerate()
          {
            child2_storage[ usize::from( cell_index ) ] = parent_block[ index ];
          }
        }
        else 
        {
          let parent_block = parent2.board.block( BlockIndex::from( ( j as u8, i as u8 ) ) ).collect_vec();
          let cells = parent2.board.block_cells( BlockIndex::from( ( j as u8, i as u8 ) ) );
          for ( index, cell_index ) in cells.enumerate()
          {
            child2_storage[ usize::from( cell_index ) ] = parent_block[ index ];
          }
        }
      }
    }

    let min_board = [ Board::new( child1_storage ), Board::new( child2_storage ) ]
    .into_iter()
    .min_by( | b1, b2 | b1.total_error().cmp( &b2.total_error() ) )
    .unwrap()
    ;

    SudokuPerson::with_board( min_board )   
  }
}

/// Performs selection of Individuals for genetic crossover and production of new Individual for next generation.
pub trait SelectionOperator< P : Individual > : Debug
{
  /// Select Individuals which will be used by GA crossover and mutation operators for production of new individual.
  fn select< 'a >( &self, hrng : Hrng, population : &'a Vec< P > ) -> &'a P;
}

/// Selection operator which randomly selects a group of individuals from the population( the number of individuals selected is equal to the size value) and choosing the most fit with probability defined by selection_pressure value.
#[ derive( Debug ) ]
pub struct TournamentSelection 
{
  /// Number of Individuals selected to compete in tournament.
  pub size : usize,
  /// Probabilistic measure of a individuals likelihood of being selected in the tournament.
  pub selection_pressure : f64,
}


impl SelectionOperator< <SudokuInitial as SeederOperator>::Person > for TournamentSelection
{
  fn select< 'a >( &self, hrng : Hrng, population : &'a Vec< <SudokuInitial as SeederOperator>::Person > ) -> &'a <SudokuInitial as SeederOperator>::Person
  {
    let rng_ref = hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();
    let mut candidates = Vec::new();
    for _ in 0..self.size
    {
      candidates.push( population.choose( &mut *rng ).unwrap() );
    }
    candidates.sort_by( | c1, c2 | c1.fitness().cmp( &c2.fitness() ) );

    let rand : f64 = rng.gen();
    let mut selection_pressure = self.selection_pressure;
    let mut winner = *candidates.last().unwrap();
    for i in 0..self.size
    {
      if rand < selection_pressure
      {
        winner = candidates[ i ];
        break;
      }
      selection_pressure += selection_pressure * ( 1.0 - selection_pressure );
    }
    winner
  }
}

/// Functionality of Individual(potential solution) for optimization with SA and GA.
pub trait Individual
{
  /// Objective function value that is used to measure how close Individual solution is to optimum.
  fn fitness( &self ) -> usize;
  /// Recalculate fitness value of individual.
  fn update_fitness( &mut self );
  /// Check if current solution is optimal.
  fn is_optimal( &self ) -> bool;
}

impl Individual for SudokuPerson
{
  fn is_optimal( &self ) -> bool
  {
    if self.cost == 0.into()
    {
      true
    }
    else 
    {
      false
    }
  }

  fn fitness( &self ) -> usize 
  {
    self.cost.into()
  }

  fn update_fitness( &mut self )
  {
    self.cost = self.board.total_error().into();
  }

}
    
pub trait MutationOperator : Debug
{
  type Person : Individual;
  type Context;

  fn mutate( &self, hrng : Hrng, person : &mut Self::Person, context : &Self::Context );
}

#[ derive( Debug ) ]
pub struct RandomPairInBlockMutation {}

impl MutationOperator for RandomPairInBlockMutation
{
  type Person = SudokuPerson;
  type Context = Board;

  fn mutate( &self, hrng : Hrng, person : &mut Self::Person, context : &Self::Context ) 
    {
        let mutagen : SudokuMutagen =
        loop 
        { 
          let rng_ref = hrng.rng_ref();
          let mut rng = rng_ref.lock().unwrap();
          let block : BlockIndex = rng.gen();
          drop( rng );
          if let Some( m ) = cells_pair_random_in_block( &context, block, hrng.clone() )
          {
            break m;
          }
        }.into();
      let old_cross_error = person.board.cross_error( mutagen.cell1 )
        + person.board.cross_error( mutagen.cell2 );
      
      log::trace!( "cells_swap( {:?}, {:?} )", mutagen.cell1, mutagen.cell2 );
      person.board.cells_swap( mutagen.cell1, mutagen.cell2 );
      person.cost -= old_cross_error.into();
      person.cost += person.board.cross_error( mutagen.cell1 ).into();
      person.cost += person.board.cross_error( mutagen.cell2 ).into();
    }

}

/// Fuctionality of operator responsible for creation of initial solutions generation.
pub trait SeederOperator
{
  /// Type that represents generation of solutions in optimization process.
  type Person : Individual + Clone + PartialEq + Send + Sync;
  type Context : Sync;

  /// Create the initial generation for the optimization algorithm.
  fn initial_generation( &self, hrng : Hrng, size : usize ) -> Vec< Self::Person >;

  /// Create the initial generation for the optimization algorithm.
  fn initial_temperature( &self, hrng : Hrng ) -> Temperature;

  fn context( &self ) -> &Self::Context;
}

/// Functionality of generation of solutions for optimization.
pub trait Generation
{
  /// Check if current generation contains optimal solution.
  fn is_good_enough( &self ) -> bool;
}


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
//! Termination: process is stopped if sudoku solution is found or if max_dynasties_number value is exseeded.
//! 

use std::fmt::Debug;
use deterministic_rand::{ Rng, Hrng, seq::SliceRandom };

/// Functionality of crossover genetic operator.
pub trait CrossoverOperator : Debug
{
  /// Type that represents solution that crossover is performed on.
  type Person : Individual + Clone;

  /// Produce new Individual using genetic matherial of two selected Individuals.
  fn crossover( &self, hrng : Hrng, parent1 : &Self::Person, parent2 : &Self::Person ) -> Self::Person;
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

impl Default for TournamentSelection
{
  fn default() -> Self 
  {
    Self
    {
      size : 2,
      selection_pressure : 0.85,
    }
  }
}

/// Functionality of Individual(potential solution) for optimization with SA and GA.
pub trait Individual
{
  /// Objective function value that is used to measure how close Individual solution is to optimum.
  fn fitness( &self ) -> usize;
  /// Recalculate fitness value of individual.
  fn update_fitness( &mut self, value : f64 );
  /// Check if current solution is optimal.
  fn is_optimal( &self ) -> bool;
}

/// Mutation operator, used to randomly change person's genome and intoduce more diversity into population.
pub trait MutationOperator : Debug
{
  /// Type that represents possible solution of initial problem.
  type Person : Individual;
  /// Additional Information for mutation.
  type Problem : InitialProblem;

  /// Randomly changes person's genome.
  fn mutate( &self, hrng : Hrng, person : &mut Self::Person, context : &Self::Problem );
}

/// Fuctionality of operator responsible for creation of initial solutions population.
pub trait InitialProblem
{
  /// Type that represents Individual in population of solutions in optimization process.
  type Person : Individual + Clone + PartialEq + Send + Sync + Debug;

  /// Create the initial population for the optimization algorithm.
  fn initial_population( &self, hrng : Hrng, size : usize ) -> Vec< Self::Person >
  {
    let mut population = Vec::new();
    for _ in 0..size
    {
      population.push( self.get_random_person( hrng.clone() ) );
    }
    population
  }

  /// Get random initial solution.
  fn get_random_person( &self, hrng : Hrng ) -> Self::Person;

  /// Evaluate fitness of provided solution.
  fn evaluate( &self, person : &Self::Person ) -> f64;
}

/// Indicates state of population proportions with no percentage for elites selection set.
pub struct NoElites;
/// Indicates state of population proportions with no percentage for mutating population set.
pub struct NoMutations;
/// Indicates state of population proportions with no percentage for crossover set.
pub struct NoCrossover;

/// Proportion of population modifications with crossover, mutations and elites cloning.
pub struct PopulationModificationProportions< E, M, C >
{
  elite_selection_rate : E,
  mutation_rate : M,
  crossover_rate : C,
}

impl PopulationModificationProportions< NoElites, NoMutations, NoCrossover >
{
  /// Create new uniniatialized proportions.
  pub fn new() -> PopulationModificationProportions< NoElites, NoMutations, NoCrossover >
  {
    PopulationModificationProportions
    {
      elite_selection_rate : NoElites,
      mutation_rate : NoMutations,
      crossover_rate : NoCrossover,
    }
  }

  /// Set part of population that will be replaced by crossover.
  pub fn set_crossover_rate( self, crossover : f64 ) -> PopulationModificationProportions< NoElites, NoMutations, f64 >
  {
    PopulationModificationProportions
    {
      crossover_rate : crossover,
      elite_selection_rate : self.elite_selection_rate,
      mutation_rate : self.mutation_rate,
    }
  }

  /// Set part of population tha will be mutated to create new population.
  pub fn set_mutation_rate( self, mutation : f64 ) -> PopulationModificationProportions< NoElites, f64, NoCrossover >
  {
    PopulationModificationProportions
    {
      crossover_rate : self.crossover_rate,
      elite_selection_rate : self.elite_selection_rate,
      mutation_rate : mutation,
    }
  }

  /// Set part of most fit population that will be cloned.
  pub fn set_elites_selection_rate( self, elites : f64 ) -> PopulationModificationProportions< f64, NoMutations, NoCrossover >
  {
    PopulationModificationProportions
    {
      crossover_rate : self.crossover_rate,
      elite_selection_rate : elites,
      mutation_rate : self.mutation_rate,
    }
  }
}

impl PopulationModificationProportions< f64, NoMutations, NoCrossover >
{
  /// Set part of population that will be replaced by crossover, calculate remaining mutation part.
  pub fn set_crossover_rate( self, crossover : f64 ) -> PopulationModificationProportions< f64, f64, f64 >
  {
    PopulationModificationProportions
    {
      crossover_rate : crossover,
      elite_selection_rate : self.elite_selection_rate,
      mutation_rate : 1.0 - self.elite_selection_rate - crossover,
    }
  }

  /// Set part of population tha will be mutated to create new population, calculate remaining crossover part.
  pub fn set_mutation_rate( self, mutation : f64 ) -> PopulationModificationProportions< f64, f64, f64 >
  {
    PopulationModificationProportions
    {
      crossover_rate : 1.0 - self.elite_selection_rate - mutation,
      elite_selection_rate : self.elite_selection_rate,
      mutation_rate : mutation,
    }
  }
}

impl PopulationModificationProportions< NoElites, f64, NoCrossover >
{
  /// Set part of population that will be replaced by crossover, calculate remaining elites part.
  pub fn set_crossover_rate( self, crossover : f64 ) -> PopulationModificationProportions< f64, f64, f64 >
  {
    PopulationModificationProportions
    {
      crossover_rate : crossover,
      elite_selection_rate : 1.0 - self.mutation_rate - crossover,
      mutation_rate : self.mutation_rate,
    }
  }

  /// Set part of most fit population that will be cloned, calculate remaining crossover part.
  pub fn set_elites_selection_rate( self, elites : f64 ) -> PopulationModificationProportions< f64, f64, f64 >
  {
    PopulationModificationProportions
    {
      crossover_rate : 1.0 - elites - self.mutation_rate,
      elite_selection_rate : elites,
      mutation_rate : self.mutation_rate,
    }
  }
}

impl PopulationModificationProportions< NoElites, NoMutations, f64 >
{
  /// Set part of population tha will be mutated to create new population, calculate remaining elites part.
  pub fn set_mutation_rate( self, mutation : f64 ) -> PopulationModificationProportions< f64, f64, f64 >
  {
    PopulationModificationProportions
    {
      crossover_rate : self.crossover_rate,
      elite_selection_rate : 1.0 - mutation - self.crossover_rate,
      mutation_rate : mutation,
    }
  }

  /// Set part of most fit population that will be cloned, calculate remaining mutated part.
  pub fn set_elites_selection_rate( self, elites : f64 ) -> PopulationModificationProportions< f64, f64, f64 >
  {
    PopulationModificationProportions
    {
      mutation_rate : 1.0 - elites - self.crossover_rate,
      elite_selection_rate : elites,
      crossover_rate : self.crossover_rate,
    }
  }
}

impl PopulationModificationProportions< f64, f64, f64 >
{
  /// Get population part modified by mutation.
  pub fn mutation_rate( &self ) -> f64
  {
    self.mutation_rate
  }

  /// Get population part of most fit Inidividuals that are cloned.
  pub fn elite_selection_rate( &self ) -> f64
  {
    self.elite_selection_rate
  }

  /// Get population part, modified by crossover.
  pub fn crossover_rate( &self ) -> f64
  {
    self.crossover_rate
  }
}

impl< P : Individual > SelectionOperator< P > for TournamentSelection
{
  fn select< 'a >
  ( 
    &self, hrng : Hrng, 
    population : &'a Vec< P > 
  ) -> &'a P
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
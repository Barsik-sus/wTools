use optimization_tools::*;
use simplex::*;

#[ test ]
fn constraint() 
{
  let c = Constraint::new( vec![ 1.0, 2.0 ], 4.0, Comp::Greater );
  assert_eq!( c.value, 4.0 );
}

#[ test ]
fn problem_2_vars() 
{
  let p = Problem::new
  ( 
    vec![ Variable::new( 3.0 ).min( 0.0 ), Variable::new( 2.0 ).min( 0.0 ) ], 
    vec![ Constraint::new( vec![ 2.0, 1.0 ], 9.0, Comp::Less ), Constraint::new( vec![ 1.0, 2.0 ], 9.0, Comp::Less ) ],
  );
  let c = Constraint::new( vec![ 1.0, 2.0 ], 4.0, Comp::Greater );
  assert_eq!( c.value, 4.0 );

  let solution = SimplexSolver{}.solve( p );
  assert_eq!( solution.point, vec![ 3.0, 3.0 ] )
}

#[ test ]
fn problem_3_vars() 
{
  let p = Problem::new
  ( 
    vec![ Variable::new( 0.0 ).min( 0.0 ), Variable::new( 0.0 ).min( 0.0 ), Variable::new( 1.0 ).min( 0.0 ) ], 
    vec!
    [ 
      Constraint::new( vec![ 1.0, 2.0, 0.0 ], 2.0, Comp::Less ), 
      Constraint::new( vec![ 0.0, 3.0, 1.0 ], 3.0, Comp::Less ),
      Constraint::new( vec![ 3.0, 0.0, 2.0 ], 6.0, Comp::Less ),
    ],
  );

  let solution = SimplexSolver{}.solve( p );
  assert_eq!( solution.point, vec![ 0.0, 0.0, 3.0 ] )
}

#[ test ]
fn problem_4_vars() 
{
  let p = Problem::new
  ( 
    vec!
    [ 
      Variable::new( -5.0 ).min( 0.0 ), 
      Variable::new( -10.0 ).min( 0.0 ), 
      Variable::new( -15.0 ).min( 0.0 ),
      Variable::new( -4.0 ).min( 0.0 ),
    ], 
    vec!
    [ 
      Constraint::new( vec![ 1.0, 1.0, 0.0, 0.0 ], 700.0, Comp::Less ), 
      Constraint::new( vec![ 0.0, 0.0, 1.0, 1.0 ], 800.0, Comp::Less ),
      Constraint::new( vec![ 1.0, 0.0, 1.0, 0.0 ], 600.0, Comp::Greater ),
      Constraint::new( vec![ 0.0, 1.0, 0.0, 1.0 ], 400.0, Comp::Greater ),
    ],
  );

  let solution = SimplexSolver{}.solve( p );
  assert_eq!( solution.point, vec![ 600.0, 0.0, 0.0, 400.0 ] )
}

#[ test ]
fn problem_draw() 
{
  let mut p = Problem::new
  ( 
    vec![ Variable::new( 3.0 ), Variable::new( 2.0 ) ], 
    vec![ Constraint::new( vec![ 2.0, 1.0 ], 9.0, Comp::Less ), Constraint::new( vec![ 1.0, 2.0 ], 9.0, Comp::Less ) ],
  );

  let ex_points = SimplexSolver::extreme_points( &mut p );
  let _ = drawing::draw_problem( &p, ex_points );
}

#[ cfg( feature = "lp_parse" ) ]
#[ test ]
fn problem_parse() 
{
  let p = Problem::new
  ( 
    vec![ Variable::new( 2.0 ).min( 0.0 ), Variable::new( -3.0 ).min( 0.0 ), Variable::new( 4.0 ).min( 0.0 ) ], 
    vec!
    [ 
    Constraint::new( vec![ 2.0, -3.0, 1.0 ], 3.0, Comp::Less ), 
    Constraint::new( vec![ 1.0, -1.0, 0.0 ], 4.0, Comp::Less ) 
    ],
  );
  let parsed = crate::parser::ProblemParser::parse( "2*x - 3*y + 4*z", vec![ "2*x -3*y +z <= 3", "-y + x <=4" ] );

  assert_eq!( p.var_coeffs, parsed.var_coeffs );
  assert_eq!( p.constraints, parsed.constraints );
}


use test_tools::*;

use super::TheModule;

//

fn basic_test()
{
  let src = "abc";
  let iter = TheModule::string::split()
  .src( src )
  .perform();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "", "", "a", "", "b", "", "c", "", "", ] );
}

//

fn split_with_option_preserving_empty_test()
{
  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( " " )
  .preserving_empty( true )
  .stripping( false )
  .perform();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", " ", "b", " ", "c" ] );

  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( " " )
  .preserving_empty( false )
  .stripping( false )
  .perform();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", " ", "b", " ", "c" ] );

  /* */

  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( " " )
  .preserving_empty( true )
  .stripping( true )
  .perform();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "", "b", "", "c" ] );

  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( " " )
  .preserving_empty( false )
  .stripping( true )
  .perform();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "b", "c" ] );
}

//

fn split_with_option_preserving_delimeters_test()
{
  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( " " )
  .preserving_delimeters( true )
  .stripping( false )
  .perform();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", " ", "b", " ", "c" ] );

  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( " " )
  .preserving_delimeters( false )
  .stripping( false )
  .perform();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "b", "c" ] );
}

//

fn split_with_option_stripping_test()
{
  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( " " )
  .stripping( true )
  .perform();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "", "b", "", "c" ] );

  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( " " )
  .stripping( false )
  .perform();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", " ", "b", " ", "c" ] );

  /* */

  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( "b" )
  .stripping( true )
  .perform();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "b", "c" ] );

  let src = "a b c";
  let iter = TheModule::string::split()
  .src( src )
  .delimeter( "b" )
  .preserving_delimeters( false )
  .stripping( true )
  .perform();
  assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "c" ] );
}

//

test_suite!
{
  basic,
  split_with_option_preserving_empty,
  split_with_option_preserving_delimeters,
  split_with_option_stripping,
}

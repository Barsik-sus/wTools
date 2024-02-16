#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn from_named_fields()
{
  use TheModule::prelude::*;

  #[ derive( Debug, PartialEq, TheModule::VariadicFrom ) ]
  struct StructNamedFields
  {
    a : i32,
    b : i32,
    c : i32,
    d : i32,
  }

  include!( "./only_test/variadic_from_named.rs" );
}

//

#[ test ]
fn from_tuple()
{
  use TheModule::prelude::*;

  #[ derive( Debug, PartialEq, TheModule::VariadicFrom ) ]
  struct StructTuple( i32, i32, i32, i32 );

  include!( "./only_test/variadic_from_tuple.rs" );
}

//

#[ test ]
fn sample()
{
  use TheModule::exposed::*;

  #[ derive( Debug, PartialEq, TheModule::VariadicFrom ) ]
  struct MyStruct
  {
    a : i32,
    b : i32,
  }

  let got : MyStruct = from!();
  let exp = MyStruct { a : 0, b : 0 };
  a_id!( got, exp );

  let got : MyStruct = from!( 13 );
  let exp = MyStruct { a : 13, b : 13 };
  a_id!( got, exp );

}

// qqq : add to examples and to readme

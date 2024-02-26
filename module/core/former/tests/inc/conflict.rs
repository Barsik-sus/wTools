#[ allow( unused_imports ) ]
use super::*;

#[allow(dead_code)]
type Option = ();
#[allow(dead_code)]
type Some = ();
#[allow(dead_code)]
type None = ();
#[allow(dead_code)]
type Result = ();
#[allow(dead_code)]
type Ok = ();
#[allow(dead_code)]
type Err = ();
#[allow(dead_code)]
type Box = ();
#[allow(dead_code)]
type Default = ();
#[allow(dead_code)]
type HashSet = ();
#[allow(dead_code)]
type HashMap = ();

#[derive( Debug, PartialEq, TheModule::Former )]
pub struct Struct1
{
  pub int_1 : i32,
  string_1 : String,
  vec_1 : Vec< String >,
  int_optional_1 : core::option::Option< i32 >,
  string_optional_1 : core::option::Option< String >,
  hashmap_strings_1 : std::collections::HashMap< String, String >,
  hashset_strings_1 : std::collections::HashSet< String >,
}

//

// include!( "only_test/basic_with_runtine.rs" );
include!( "only_test/basic_without_runtime.rs" );

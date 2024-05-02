//! # Example Usage
//!
//! Demonstrates how to use `HashMapSubformer` with the `HashMapLike` trait to build a `std::collections::HashMap`:
//!

#[ cfg( not( all( feature = "derive_former", not( feature = "no_std" ) ) ) ) ]
fn main() {}

#[ cfg( all( feature = "derive_former", not( feature = "no_std" ) ) ) ]
fn main()
{
  use test_tools::exposed::*;

  #[ derive( Debug, PartialEq, former::Former ) ]
  pub struct StructWithSet
  {
    #[ container( definition = former::HashSetSubformer ) ]
    set : std::collections::HashSet< &'static str >,
  }

  let instance = StructWithSet::former()
  .set()
    .insert("apple")
    .insert("banana")
    .end()
  .form();

  assert_eq!(instance, StructWithSet { set : hset![ "apple", "banana" ] });

}

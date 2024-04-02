#[ allow( unused_imports ) ]
use super::*;

// use std::collections::HashMap;
// use std::collections::HashSet;

#[ derive( Debug, PartialEq, the_module::Former ) ]
#[ debug ]
pub struct Struct1
{
  #[ subformer( the_module::VectorSubformer ) ]
  vec_1 : Vec< String >,
  #[ subformer( the_module::HashMapSubformer ) ]
  hashmap_1 : std::collections::HashMap< String, String >,
  #[ subformer( the_module::HashSetSubformer ) ]
  hashset_1 : std::collections::HashSet< String >,
}

// include!( "./only_test/containers_with_subformer.rs" );
// xxx : uncomment
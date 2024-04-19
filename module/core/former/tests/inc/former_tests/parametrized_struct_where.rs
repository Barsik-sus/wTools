#![ allow( dead_code ) ]
#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq, Default ) ]
pub struct Property< Name >
{
  name : Name,
  code : isize,
}

/// generated by new
impl< Name > Property< Name >
{
  #[ inline ]
  pub fn new< Code >( name : Name, code : Code ) -> Self
  where
    Name : core::convert::Into< Name >,
    Code : core::convert::Into< isize >,
  {
    Self { name : name.into(), code : code.into() }
  }
}

#[ derive( Debug, PartialEq, the_module::Former ) ]
// #[ derive( Debug, PartialEq, the_module::Former ) ] #[ debug ]
// #[ derive( Debug, PartialEq ) ]
pub struct Command< K >
where
  K : core::hash::Hash + std::cmp::Eq,
{
  pub name : String,
  #[ subformer( former::HashMapDefinition ) ]
  pub properties : collection_tools::HashMap< K, Property< K > >,
}

// == begin of generated

// == end of generated

include!( "./only_test/parametrized_struct.rs" );

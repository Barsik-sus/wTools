#![ allow( dead_code ) ]

use super::*;

/// Parameter description.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct Child
{
  name : String,
  data : bool,
}

/// Parent required for the template.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
// #[ derive( Debug, Default, PartialEq, the_module::Former ) ] #[ debug ]
// #[ derive( Debug, Default, PartialEq ) ]
pub struct Parent
{
  #[ scalar( name = children3 ) ]
  #[ subform_collection( name = children2 ) ]
  #[ subform_entry( name = _child ) ]
  children : Vec< Child >,
}

impl< Definition > ParentFormer< Definition >
where
  Definition : former::FormerDefinition< Storage = < Parent as former::EntityToStorage >::Storage >,
{

  #[ inline( always ) ]
  pub fn child( self, name : &str ) ->
  ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
  {
    self._children_subform_entry
    ::< ChildFormer< _ >, _, >()
    .name( name )
  }

  #[ inline( always ) ]
  pub fn children( self ) -> &'static str
  {
    r#"
    Scalar setter `children` should not be generated by default if subform is used.
    It can only be generated if req
    "#
  }

}

// == begin of generated

// == end of generated

include!( "./only_test/subform_entry_child.rs" );
include!( "./only_test/subform_collection_children2.rs" );
include!( "./only_test/scalar_children3.rs" );

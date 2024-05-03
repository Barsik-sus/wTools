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
  #[ subform( name = _child ) ]
  children : Vec< Child >,
}

impl< Definition > ParentFormer< Definition >
where
  Definition : former::FormerDefinition< Storage = < Parent as former::EntityToStorage >::Storage >,
  // Definition::Types : former::FormerDefinitionTypes< Storage = < Parent as former::EntityToStorage >::Storage >,
{

  #[ inline( always ) ]
  pub fn children( self ) -> &'static str
  {
    r#"
    Scalar setter `children` should not be generated by default if subform is used.
    It can only be generated if req
    "#
  }

  #[ inline( always ) ]
  pub fn child( self, name : &str ) ->
  ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
  {
    self._children_add
    ::< ChildFormer< _ >, _, >()
    .name( name )
  }

  // #[ inline( always ) ]
  // pub fn _child( self ) ->
  // ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
  // {
  //   self._children_add
  //   ::< < Child as former::EntityToFormer< _ > >::Former, _, >()
  // }

}

// == begin of generated

// == end of generated

include!( "./only_test/subformer_subform_child.rs" );

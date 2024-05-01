#![ allow( dead_code ) ]
use super::*;

// == command

#[ derive( Debug, PartialEq, the_module::Former ) ]
pub struct Command< K >
where
  K : core::hash::Hash + std::cmp::Eq,
{
  pub name : String,
  pub subject : K,
}

// // = command subformer - generated
//
// pub type CommandAsSubformer< K, Superformer, End > = CommandFormer
// <
//   K,
//   CommandFormerDefinition
//   <
//     K,
//     Superformer,
//     Superformer,
//     End,
//     // impl former::FormingEnd< CommandFormerDefinitionTypes< K, Superformer, Superformer > >,
//   >,
// >;
//
// // = command subformer end - generated
//
// pub trait CommandAsSubformerEnd< K, SuperFormer >
// where
//   K : core::hash::Hash + std::cmp::Eq,
//   Self : the_module::FormingEnd
//   <
//     CommandFormerDefinitionTypes< K, SuperFormer, SuperFormer >,
//   >
// {
// }
//
// impl< K, SuperFormer, T > CommandAsSubformerEnd< K, SuperFormer >
// for T
// where
//   K : core::hash::Hash + std::cmp::Eq,
//   Self : the_module::FormingEnd
//   <
//     CommandFormerDefinitionTypes< K, SuperFormer, SuperFormer >,
//   >
// {
// }

// == aggregator

#[ derive( Debug, PartialEq, the_module::Former ) ]
pub struct Aggregator< K >
where
  K : core::hash::Hash + std::cmp::Eq,
{
  pub parameter1 : String,
  #[ container( former::HashMapDefinition ) ]
  pub commands : collection_tools::HashMap< String, Command< K > >,
}

// =

impl< K, Definition > AggregatorFormer
<
  K,
  Definition,
>
where
  K : core::hash::Hash + std::cmp::Eq,
  Definition : former::FormerDefinition,
  Definition::Storage : former::StoragePreform< Preformed = Aggregator< K > >,
  Definition::Types : former::FormerDefinitionTypes< Storage = AggregatorFormerStorage< K > >, // xxx
{

  //

  #[ inline( always ) ]
  pub fn command_with_closure< IntoName >( self, name : IntoName )
  ->
  CommandAsSubformer< K, Self, impl CommandAsSubformerEnd< K, Self > >
  where
    IntoName : core::convert::Into< String >,
  {

    let on_end = | storage : CommandFormerStorage< K >, super_former : core::option::Option< Self > | -> Self
    {
      let formed =  former::StoragePreform::preform( storage );
      let mut super_former = super_former.unwrap();
      if let Some( ref mut container ) = super_former.storage.commands
      {
        former::ContainerAdd::add( container, ( formed.name.clone(), formed ) );
      }
      else
      {
        let mut container : collection_tools::HashMap< String, Command< K > > = Default::default();
        former::ContainerAdd::add( &mut container, ( formed.name.clone(), formed ) );
        super_former.storage.commands = Some( container );
      }
      super_former
    };

    let former
    : CommandFormer< _, _ >
    = CommandFormer::begin( None, Some( self ), on_end );

    former.name( name )
  }

  //

  #[ inline( always ) ]
  pub fn command_with_type< IntoName >( self, name : IntoName )
  ->
  CommandAsSubformer< K, Self, impl CommandAsSubformerEnd< K, Self > >
  where
    IntoName : core::convert::Into< String >,
  {
    let former = CommandFormer::begin( None, Some( self ), AggregatorFormerCommandEnd );
    former.name( name )
  }

}

pub struct AggregatorFormerCommandEnd;
impl< K, Definition > former::FormingEnd
<
  CommandFormerDefinitionTypes
  <
    K,
    AggregatorFormer< K, Definition >,
    AggregatorFormer< K, Definition >,
  >,
>
for AggregatorFormerCommandEnd
where
  K : core::hash::Hash + std::cmp::Eq,
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes
  <
    Storage = AggregatorFormerStorage< K >,
  >,
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    sub_storage : CommandFormerStorage< K >,
    super_former : Option< AggregatorFormer< K, Definition > >,
  )
  ->
  AggregatorFormer< K, Definition >
  {
    let preformed = former::StoragePreform::preform( sub_storage );
    let mut super_former = super_former.unwrap();
    if let Some( ref mut container ) = super_former.storage.commands
    {
      former::ContainerAdd::add( container, Into::into( preformed ) );
    }
    else
    {
      let mut container : collection_tools::HashMap< String, Command< K > > = Default::default();
      former::ContainerAdd::add( &mut container, Into::into( preformed ) );
      super_former.storage.commands = Some( container );
    }
    super_former
  }
}

//

// /// Convert an entity to an element which could be added to a container.
// pub trait IntoElement< Element >
// {
//   /// Convert an entity to an element which could be added to a container.
//   fn into_element( self ) -> Element;
// }
//
// impl< K > IntoElement< ( String, Command< K > ) >
// for Command< K >
// where
//   K : core::hash::Hash + std::cmp::Eq,
// {
//   fn into_element( self ) -> ( String, Command< K > )
//   {
//     ( self.name.clone(), self )
//   }
// }

//

impl< K > From< Command< K > >
for ( String, Command< K > )
where
  K : core::hash::Hash + std::cmp::Eq,
{
  #[ inline( always ) ]
  fn from( src : Command< K > ) -> Self
  {
    ( src.name.clone(), src )
  }
}

//

// ==

include!( "./only_test/subformer_custom.rs" );

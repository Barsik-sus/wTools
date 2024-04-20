#![ allow( dead_code ) ]
use super::*;

//
// this should work
//
// let ca = Aggregator::former()
// .parameter1( "val" )
// .command( "echo" )
//   .name( "prints all subjects and properties" )
//   .subject( "Subject", wca::Type::String, true )
//   .property( "property", "simple property", wca::Type::String, true )
//   .routine( f1 )
//   .end()
// .command( "exit" )
//   .name( "just exit" )
//   .routine( || exit() )
//   .end()
// .perform()
// ;
// ca.execute( input ).unwrap();

// == property

#[ derive( Debug, PartialEq, Default ) ]
pub struct Property< Name >
{
  name : Name,
  description : String,
  code : isize,
}

/// generated by new
impl< Name > Property< Name >
{
  #[ inline ]
  pub fn new< Description, Code >( name : Name, description : Description, code : Code ) -> Self
  where
    Name : core::convert::Into< Name >,
    Description : core::convert::Into< String >,
    Code : core::convert::Into< isize >,
  {
    Self { name : name.into(), description : description.into(), code : code.into() }
  }
}

// == command

#[ derive( Debug, PartialEq, the_module::Former ) ]
pub struct Command< K >
where
  K : core::hash::Hash + std::cmp::Eq,
{
  pub name : String,
  pub subject : String,
  // #[ subformer( the_module::HashMapSubformer ) ]
  #[ subformer( former::HashMapDefinition ) ]
  pub properties : collection_tools::HashMap< K, Property< K > >,
}

// manual
impl< K, Definition > CommandFormer< K, Definition >
where
  K : core::hash::Hash + std::cmp::Eq,
  Definition : former::FormerDefinition,
  < Definition::Types as former::FormerDefinitionTypes >::Storage : former::StoragePreform,
  Definition::Types : former::FormerDefinitionTypes< Storage = CommandFormerStorage< K > >,
{

  /// Inserts a key-value pair into the map. Make a new container if it was not made so far.
  #[ inline( always ) ]
  pub fn property< Name, Description, Code >
  ( mut self, name : Name, description : Description, code : Code ) -> Self
  where
    Name : core::convert::Into< K > + Clone,
    Description : core::convert::Into< String >,
    Code : core::convert::Into< isize >,
  {
    if self.storage.properties.is_none()
    {
      self.storage.properties = core::option::Option::Some( Default::default() );
    }
    if let core::option::Option::Some( ref mut properties ) = self.storage.properties
    {
      let property = Property
      {
        name : name.clone().into(),
        description : description.into(),
        code : code.into(),
      };
      properties.insert( name.into(), property );
    }
    self
  }

}

// == aggregator

#[ derive( Debug, PartialEq, the_module::Former ) ]
pub struct Aggregator< K >
where
  K : core::hash::Hash + std::cmp::Eq,
{
  pub parameter1 : String,
  // #[ subformer( the_module::HashMapSubformer ) ]
  #[ subformer( former::HashMapDefinition ) ]
  pub commands : collection_tools::HashMap< String, Command< K > >,
}

pub type CommandSubformer< K, Superformer > = CommandFormer
<
  K,
  CommandFormerDefinition
  <
    K,
    Superformer,
    Superformer,
    former::FormingEndClosure< CommandFormerDefinitionTypes< K, Superformer, Superformer > >,
    // impl former::FormingEnd< CommandFormerDefinitionTypes< K, Superformer, Superformer > >,
  >,
>;

// manual
// impl< K, Context, End >
// AggregatorFormer< K, Context, End >
// where
//   K : core::hash::Hash + std::cmp::Eq,
//   End : the_module::FormingEnd< Aggregator< K >, Context >,

impl< K, Definition > AggregatorFormer
<
  K,
  Definition,
>
where
  K : core::hash::Hash + std::cmp::Eq,
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = AggregatorFormerStorage< K > >,

  < Definition::Types as former::FormerDefinitionTypes >::Storage : former::StoragePreform,
  < Definition::Types as former::FormerDefinitionTypes >::Storage : former::Storage< Formed = Aggregator< K > >,

{

  #[ inline( always ) ]
  pub fn command_with_types< IntoName >( self, name : IntoName )
  ->
  // CommandSubformer< K, Self >
  CommandFormer
  <
    K,
    CommandFormerDefinition
    <
      K,
      Self,
      Self,
      impl the_module::FormingEnd< CommandFormerDefinitionTypes< K, Self, Self > >,
    >,
  >
  where
    IntoName : core::convert::Into< String >,
  {

    let former
    // : CommandSubformer< K, Self >
    // : CommandFormer
    // <
    //   K,
    //   CommandFormerDefinition
    //   <
    //     K,
    //     Self,
    //     Self,
    //     AggregatorFormerCommandEnd2,
    //   >
    // >
    = CommandFormer::_begin_precise( None, Some( self ), AggregatorFormerCommandEnd2 );

    former.name( name )

  }

  #[ inline( always ) ]
  pub fn command_with_closure< IntoName >( self, name : IntoName )
  ->
  CommandSubformer< K, Self >
  where
    IntoName : core::convert::Into< String >,
  {

    let on_end = | command : CommandFormerStorage< K >, super_former : core::option::Option< Self > | -> Self
    {
      let command =  former::StoragePreform::preform( command );
      let mut super_former = super_former.unwrap();
      if let Some( ref mut commands ) = super_former.storage.commands
      {
        former::ContainerAdd::add( commands, ( command.name.clone(), command ) );
      }
      else
      {
        let mut commands : collection_tools::HashMap< String, Command< K > > = Default::default();
        former::ContainerAdd::add( &mut commands, ( command.name.clone(), command ) );
        super_former.storage.commands = Some( commands );
      }
      super_former
    };

    let former = CommandFormer::begin( None, Some( self ), on_end );
    former.name( name )

  }

}

#[ allow( non_camel_case_types ) ]
pub struct AggregatorFormerCommandEnd2;

#[ automatically_derived ]
impl< K, Definition > former::FormingEnd
<
  CommandFormerDefinitionTypes
  <
    K,
    AggregatorFormer< K, Definition >,
    AggregatorFormer< K, Definition >,
  >,
>
for AggregatorFormerCommandEnd2
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
    command : CommandFormerStorage< K >,
    super_former : Option< AggregatorFormer< K, Definition > >,
  )
  ->
  AggregatorFormer< K, Definition >
  {

    let command =  former::StoragePreform::preform( command );
    let mut super_former = super_former.unwrap();
    if let Some( ref mut commands ) = super_former.storage.commands
    {
      former::ContainerAdd::add( commands, ( command.name.clone(), command ) );
    }
    else
    {
      let mut commands : collection_tools::HashMap< String, Command< K > > = Default::default();
      former::ContainerAdd::add( &mut commands, ( command.name.clone(), command ) );
      super_former.storage.commands = Some( commands );
    }
    super_former

  }
}

//

/// xxx : extend description
/// Convert an entity to an element which could be added to a container.
pub trait IntoElement< Target >
{
  /// Convert an entity to an element which could be added to a container.
  fn into_element( self ) -> Target;
}

impl< K > IntoElement< ( String, Command< K > ) >
for Command< K >
where
  K : core::hash::Hash + std::cmp::Eq,
{
  fn into_element( self ) -> ( String, Command< K > )
  {
    ( self.name.clone(), self )
  }
}

//

#[ allow( non_camel_case_types ) ]
pub struct ContainerAddElement;

#[ automatically_derived ]
impl< K, SuperDefinition > former::FormingEnd
<
  CommandFormerDefinitionTypes
  <
    K,
    AggregatorFormer< K, SuperDefinition >,
    AggregatorFormer< K, SuperDefinition >,
  >,
>
for ContainerAddElement
where
  K : core::hash::Hash + std::cmp::Eq,
  SuperDefinition : former::FormerDefinition,
  SuperDefinition::Types : former::FormerDefinitionTypes
  <
    Storage = AggregatorFormerStorage< K >,
  >,
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    storage : CommandFormerStorage< K >,
    super_former : Option< AggregatorFormer< K, SuperDefinition > >,
  )
  ->
  AggregatorFormer< K, SuperDefinition >
  {

    let storage =  former::StoragePreform::preform( storage );
    let mut super_former = super_former.unwrap();
    if let Some( ref mut commands ) = super_former.storage.commands
    {
      former::ContainerAdd::add
      (
        commands,
        IntoElement::< ( String, Command< K > ) >::into_element( storage ),
        // ( storage.name.clone(), storage ),
      );
    }
    else
    {
      let mut commands : collection_tools::HashMap< String, Command< K > > = Default::default();
      former::ContainerAdd::add
      (
        &mut commands,
        IntoElement::< ( String, Command< K > ) >::into_element( storage ),
        // ( storage.name.clone(), storage ),
      );
      super_former.storage.commands = Some( commands );
    }
    super_former

  }
}

// ==

// include!( "./only_test/subformer_basic.rs" );
// xxx : uncomment

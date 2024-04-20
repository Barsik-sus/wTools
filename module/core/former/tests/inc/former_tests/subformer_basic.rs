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
  < Definition::Types as former::FormerDefinitionTypes >::Storage : former::StoragePreform< Preformed = Aggregator< K > >,

{

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
  pub fn command_with_helper< IntoName >( self, name : IntoName )
  ->
  // ()
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

    ContainerAddElement
    <
      collection_tools::HashMap
      <
        String,
        Command< K >
      >,
      (
        String,
        Command< K >,
      ),
      Command< K >
    >
    :
    former::FormingEnd
    <
      CommandFormerDefinitionTypes
      <
        K,
        Self,
        Self,
      >
    >
  {

    let former
    // : CommandSubformer< K, Self >
    : CommandFormer
    <
      K,
      CommandFormerDefinition
      <
        K,
        Self,
        Self,
        ContainerAddElement
        ::< _, _, _ >
        // ::
        // <
        //   // collection_tools::HashMap< String, Command< K > >,
        //   // ( String, Command< K > ),
        //   // Command< K >,
        // >,
      >
    >
    = CommandFormer::begin
    (
      None,
      Some( self ),
      ContainerAddElement
      ::new(),
    );

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

    let command = former::StoragePreform::preform( command );
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

/// xxx : extend description
/// get container for a field out of a storage
pub trait FormerStorageExtractContainer< Target >
{
  fn container_mut( &mut self ) -> &mut Target;
}

impl< K > FormerStorageExtractContainer< collection_tools::HashMap< String, Command< K > > >
for AggregatorFormerStorage< K >
where
  K : core::hash::Hash + std::cmp::Eq,
{
  fn container_mut( &mut self ) -> &mut collection_tools::HashMap< String, Command< K > >
  {
    if let Some( ref mut commands ) = self.commands
    {
      commands
    }
    else
    {
      let commands : collection_tools::HashMap< String, Command< K > > = Default::default();
      self.commands = Some( commands );
      self.commands.as_mut().unwrap()
    }
  }
}

//

/// xxx : extend description
/// extract storage from a former
pub trait FormerExtractStorage
{
  type Storage;
  fn storage_mut( &mut self ) -> &mut Self::Storage;
}

impl< K > FormerExtractStorage
for AggregatorFormer< K >
where
  K : core::hash::Hash + std::cmp::Eq,
{
  type Storage = AggregatorFormerStorage< K >;
  fn storage_mut( &mut self ) -> &mut Self::Storage
  {
    &mut self.storage
  }
}

//

pub struct ContainerAddElement< /*SuperDefinition,*/ SuperContainer, Element, SubFormed >
( core::marker::PhantomData< fn( /*SuperDefinition,*/ SuperContainer, Element, SubFormed ) > );

impl
<
  // SuperDefinition,
  SuperContainer,
  Element,
  SubFormed,
>
ContainerAddElement
<
  // SuperDefinition,
  SuperContainer,
  Element,
  SubFormed,
>
{
  pub fn new() -> Self
  {
    Self( core::marker::PhantomData )
  }
}

impl
<
  // SuperDefinition,
  SuperFormer,
  SuperContainer,
  Element,
  SubFormed,
  SubDefinition,
>
former::FormingEnd
<
  SubDefinition,
  // CommandFormerDefinitionTypes
  // <
  //   K,
  //   AggregatorFormer< K, SuperDefinition >,
  //   AggregatorFormer< K, SuperDefinition >,
  // >,
>
for ContainerAddElement
<
  // SuperDefinition,
  SuperContainer,
  Element,
  SubFormed,
>
where

  // SuperDefinition : former::FormerDefinitionTypes,
  // SuperDefinition::Storage : FormerStorageExtractContainer< SuperContainer >,

  // SuperFormer : FormerExtractStorage< Storage = SuperDefinition::Storage >,
  SuperFormer : FormerExtractStorage<>,
  < SuperFormer as FormerExtractStorage >::Storage : FormerStorageExtractContainer< SuperContainer >,
  SuperContainer : former::ContainerAdd< Element = Element >,

  SubDefinition : former::FormerDefinitionTypes
  <
    Formed = SuperFormer,
    Context = SuperFormer,
  >,
  SubDefinition::Storage : former::StoragePreform< Preformed = SubFormed >,

  SubFormed : IntoElement< Element >,
{

  #[ inline( always ) ]
  fn call
  (
    &self,
    storage : SubDefinition::Storage,
    super_former : Option< SuperFormer >,
  )
  ->
  SuperFormer
  {

    let storage : SubFormed = former::StoragePreform::preform( storage );
    let mut super_former = super_former.unwrap();

    let container = FormerStorageExtractContainer
    ::< SuperContainer >
    ::container_mut( FormerExtractStorage::storage_mut( &mut super_former ) );

    former::ContainerAdd::add
    (
      container,
      IntoElement::< Element >::into_element( storage ),
    );

    super_former
  }

}

// ==

// include!( "./only_test/subformer_basic.rs" );
// xxx : uncomment

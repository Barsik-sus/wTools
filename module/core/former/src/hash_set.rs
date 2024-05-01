//! # HashSetLike Trait and HashSetSubformer Struct
//!
//! This part of the crate provides a flexible interface (`HashSetLike`) and a builder pattern implementation (`HashSetSubformer`) for `HashSet`-like containers. It's designed to extend the builder pattern, allowing for fluent and dynamic construction of sets within custom data structures.

use super::*;
use collection_tools::HashSet;

impl< K > Container for collection_tools::HashSet< K >
where
  K : core::cmp::Eq + core::hash::Hash,
{
  type Element = K;
  type Val = K;

  #[ inline( always ) ]
  fn element_to_val( e : Self::Element ) -> Self::Val
  {
    e
  }

}

impl< K > ContainerAdd for collection_tools::HashSet< K >
where
  K : core::cmp::Eq + core::hash::Hash,
{
  // type Element = K;
  // type Val = K;

  #[ inline( always ) ]
  fn add( &mut self, e : Self::Element ) -> bool
  {
    self.insert( e )
  }

}

impl< T > ContainerAssign for collection_tools::HashSet< T >
where
  T : core::cmp::Eq + core::hash::Hash,
{
  type Element = T;

  fn assign< Elements >( &mut self, elements : Elements ) -> usize
  where
    Elements : IntoIterator< Item = Self::Element >
  {
    let initial_len = self.len();
    self.extend( elements );
    self.len() - initial_len
  }
}

/// A trait for containers behaving like a `HashSet`, allowing insertion operations.
///
/// Implementing this trait enables the associated formed to be used with `HashSetSubformer`,
/// facilitating a builder pattern that is both intuitive and concise.
///
/// # Example Implementation
///
/// Implementing `HashSetLike` for `std::collections::HashSet`:
///

pub trait HashSetLike< K >
where
  K : core::cmp::Eq + core::hash::Hash,
{
  /// Inserts a key-value pair into the map.
  fn insert( &mut self, element : K ) -> Option< K >;
}

impl< K > HashSetLike< K > for HashSet< K >
where
  K : core::cmp::Eq + core::hash::Hash,
{
  fn insert( &mut self, element : K ) -> Option< K >
  {
    HashSet::replace( self, element )
  }
}

// = storage

impl< K > Storage
for HashSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  // type Types = HashSetDefinition< K >;
  type Formed = HashSet< K >;
}

impl< K > StoragePreform
for HashSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Preformed = HashSet< K >;
  // fn preform( self ) -> < < Self as Storage >::Definition as FormerDefinitionTypes >::Formed
  // fn preform( self ) -> Self::Formed
  fn preform( self ) -> Self::Preformed
  {
    self
  }
}

// = definition

#[ derive( Debug, Default ) ]
pub struct HashSetDefinition< K, Context = (), Formed = HashSet< K >, End = ReturnStorage >
{
  _phantom : core::marker::PhantomData< ( K, Context, Formed, End ) >,
}

impl< K, Context, Formed > FormerDefinitionTypes
for HashSetDefinition< K, Context, Formed, NoEnd >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Storage = HashSet< K >;
  type Formed = Formed;
  type Context = Context;
}

impl< K, Context, Formed, End > FormerDefinition
for HashSetDefinition< K, Context, Formed, End >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  End : FormingEnd< HashSetDefinition< K, Context, Formed, NoEnd > >,
{
  type Storage = HashSet< K >;
  type Formed = Formed;
  type Context = Context;

  type Types = HashSetDefinition< K, Context, Formed, NoEnd >;
  type End = End;
}

// = Entity To

impl< K, Definition > EntityToFormer< Definition > for HashSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  Definition : FormerDefinition< Storage = HashSet< K >, Formed = () >,
  < Definition as definition::FormerDefinition>::End : Fn( HashSet< K >, Option< Definition::Context > ),
{
  type Former = HashSetSubformer< K, Definition::Context, Definition::Formed, Definition::End >;
}

impl< K > crate::EntityToStorage
for HashSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Storage = HashSet< K >;
}

impl< K, Context, Formed, End > crate::EntityToDefinition< Context, Formed, End >
for HashSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  End : crate::FormingEnd< HashSetDefinition< K, Context, Formed, NoEnd > >,
{
  type Definition = HashSetDefinition< K, Context, Formed, End >;
}

// = subformer

/// Facilitates building `HashSetLike` containers with a fluent API.
///
/// `HashSetSubformer` leverages the `HashSetLike` trait to enable a concise and expressive way
/// of populating `HashSet`-like containers. It exemplifies the crate's builder pattern variation for sets.
///
/// # Example Usage
///
/// Using `HashSetSubformer` to populate a `HashSet` within a struct:
///
/// ```rust
/// # #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
/// # {
/// # use test_tools::exposed::*;
///
/// #[ derive( Debug, PartialEq, former::Former ) ]
/// pub struct StructWithSet
/// {
///   #[ container( former::HashSetSubformer ) ]
///   set : std::collections::HashSet< &'static str >,
/// }
///
/// let instance = StructWithSet::former()
/// .set()
///   .insert( "apple" )
///   .insert( "banana" )
///   .end()
/// .form();
///
/// assert_eq!(instance, StructWithSet { set : hset![ "apple", "banana" ] });
/// # }
/// ```

// zzz : update documentation
// write: instead of writing long version with ContainerSubformer it's possible to be more concise with help of the type alias
//
pub type HashSetSubformer< K, Context, Formed, End > =
ContainerSubformer::< K, HashSetDefinition< K, Context, Formed, End > >;

// = extension

pub trait HashSetExt< K > : sealed::Sealed
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  fn former() -> HashSetSubformer< K, (), HashSet< K >, ReturnStorage >;
}

impl< K > HashSetExt< K > for HashSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  fn former() -> HashSetSubformer< K, (), HashSet< K >, ReturnStorage >
  {
    HashSetSubformer::< K, (), HashSet< K >, ReturnStorage >::new( ReturnStorage::default() )
  }
}

mod sealed
{
  use super::HashSet;
  pub trait Sealed {}
  impl< K > Sealed for HashSet< K > {}
}

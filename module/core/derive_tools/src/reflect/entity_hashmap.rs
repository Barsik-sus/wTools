//!
//! Implementation of Entity for a HashMap.
//!

use super::*;

/// Internal namespace.
pub mod private
{
  use super::*;

  // qqq : implementation for HashMap 
  use std::collections::HashMap;
  impl< K, V > Instance for HashMap< K, V >
  where
    CollectionDescriptor< HashMap< K, V > > : Entity,
    primitive::Primitive : From< K >,
    K : Clone,
  {
    type Entity = CollectionDescriptor::< HashMap< K, V > >;
    fn _reflect( &self ) -> Self::Entity
    {
      CollectionDescriptor::< Self >::new
      (
        self.len(),
        Some( self.keys().into_iter().map( | k | primitive::Primitive::from( k.clone() ) ).collect::< Vec< _ > >() ),
      )
    }
    #[ inline( always ) ]
    fn Reflect() -> Self::Entity
    {
      CollectionDescriptor::< Self >::new( 0, None )
    }
  }
  
  impl< K, V > Entity for CollectionDescriptor< HashMap< K, V > >
  where
    K : 'static + Instance + IsScalar + Clone,
    primitive::Primitive : From< K >,
    V : 'static + Instance,
  {
    #[ inline( always ) ]
    fn is_container( &self ) -> bool
    {
      true
    }

    #[ inline( always ) ]
    fn len( &self ) -> usize
    {
      self.len
    }

    #[ inline( always ) ]
    fn type_name( &self ) -> &'static str
    {
      core::any::type_name::< HashMap< K, V > >()
    }

    #[ inline( always ) ]
    fn type_id( &self ) -> core::any::TypeId
    {
      core::any::TypeId::of::< HashMap< K, V > >()
    }

    #[ inline( always ) ]
    fn elements( &self ) -> Box< dyn Iterator< Item = KeyVal > >
    {
      let mut result : Vec< KeyVal > = ( 0 .. self.len() )
      .map( | k | KeyVal { key : Primitive::usize( k ), val : Box::new( < V as Instance >::Reflect() ) } )
      .collect();

      let keys = self.keys
      .clone()
      .unwrap_or( ( 0..self.len() ).map( primitive::Primitive::usize ).into_iter().collect() )
      ;
      
      for i in 0..self.len()
      {
          result[ i ] = KeyVal { key : keys[ i ].clone(), val : Box::new( < V as Instance >::Reflect() ) }
      }

      Box::new( result.into_iter() )
    }
  }
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
}

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
  // pub use super::private::
  // {
  // };
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}

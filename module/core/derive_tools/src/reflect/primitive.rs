//!
//! Define primitive and data types.
//!

/// Internal namespace.
pub( crate ) mod private
{
  /// Represents a general-purpose data container that can hold various primitive types
  /// and strings. This enum is designed to encapsulate common data types in a unified
  /// format, simplifying the handling of different types of data in generic contexts.
  ///
  /// # Variants
  ///
  /// - `i8`, `i16`, `i32`, `i64`, `isize`: Signed integer types.
  /// - `u8`, `u16`, `u32`, `u64`, `usize`: Unsigned integer types.
  /// - `f32`, `f64`: Floating-point types.
  /// - `String`: A heap-allocated string (`String`).
  /// - `str`: A borrowed string slice (`&'static str`), typically used for string literals.
  /// - `binary`: A borrowed slice of bytes (`&'static [u8]`), useful for binary data.
  ///
  /// # Example
  ///
  /// Creating a `Primitive` instance with an integer:
  ///
  /// ```
  /// # use derive_tools::reflect::Primitive;
  /// let num = Primitive::i32( 42 );
  /// ```
  ///
  /// Creating a `Primitive` instance with a string:
  ///
  /// ```
  /// # use derive_tools::reflect::Primitive;
  /// let greeting = Primitive::String( "Hello, world!".to_string() );
  /// ```
  ///
  /// Creating a `Primitive` instance with a binary slice:
  ///
  /// ```
  /// # use derive_tools::reflect::Primitive;
  /// let bytes = Primitive::binary( &[ 0xde, 0xad, 0xbe, 0xef ] );
  /// ```
  ///
  #[ allow( non_camel_case_types ) ]
  #[ derive( Debug, PartialEq, Default, Clone ) ]
  pub enum Primitive
  {
    /// None
    #[ default ]
    None,
    /// Represents a signed 8-bit integer.
    i8( i8 ),
    /// Represents a signed 16-bit integer.
    i16( i16 ),
    /// Represents a signed 32-bit integer.
    i32( i32 ),
    /// Represents a signed 64-bit integer.
    i64( i64 ),
    /// Represents a machine-sized signed integer.
    isize( isize ),
    /// Represents an unsigned 8-bit integer.
    u8( u8 ),
    /// Represents an unsigned 16-bit integer.
    u16( u16 ),
    /// Represents an unsigned 32-bit integer.
    u32( u32 ),
    /// Represents an unsigned 64-bit integer.
    u64( u64 ),
    /// Represents a machine-sized unsigned integer.
    usize( usize ),
    /// Represents a 32-bit floating-point number.
    f32( f32 ),
    /// Represents a 64-bit floating-point number.
    f64( f64 ),
    /// Represents a dynamically allocated string.
    String( String ),
    /// Represents a statically allocated string slice.
    str( &'static str ),
    /// Represents a statically allocated slice of bytes.
    binary( &'static [ u8 ] ),
  }

  impl< T > From< &T > for Primitive
  where Primitive : From< T >
  {
    fn from( value: &T ) -> Self
    {
      value.to_owned().into()
    }
  }

  impl From< i32 > for Primitive
  {
    fn from( value: i32 ) -> Self
    {
      Self::i32( value )
    }
  }

  impl From< String > for Primitive
  {
    fn from( value: String ) -> Self
    {
      Self::String( value )
    }
  }

  impl From< u32 > for Primitive
  {
    fn from( value: u32 ) -> Self
    {
      Self::u32( value )
    }
  }

  #[ allow( non_camel_case_types ) ]
  #[ derive( Debug, PartialEq ) ]
  pub enum Data< const N : usize = 0 >
  {
    /// None
    Primitive( Primitive ),
    // /// Array
    // array( &'a [ Data ; N ] ),
  }

  impl< const N : usize > Default for Data< N >
  {
    fn default() -> Self
    {
      Data::Primitive( Primitive::None )
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
  pub use super::private::
  {
    Primitive,
    // Data,
  };
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

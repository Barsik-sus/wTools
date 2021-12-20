#![ warn( missing_docs ) ]

//!
//! Tools for writing procedural macroses.
//!

mod num;
mod interval;

///
/// Macro for diagnostics purpose to print both syntax tree and source code behind it.
///
/// # Sample
/// ```
/// use wproc_macro::*;
/// use quote::quote;
///
/// let code = quote!( std::collections::HashMap< i32, i32 > );
/// let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
/// tree_print!( tree_type );
/// ```
///

#[ macro_export ]
macro_rules! tree_print
{
  ( $src : expr ) =>
  {{
    let result = $crate::tree_export_str!( $src );
    println!( "{}", result );
    result
  }};
  ( $( $src : expr ),+ $(,)? ) =>
  {{
    $( $crate::tree_print!( $src ) );+
  }};
}

///
/// Macro for diagnostics purpose to export both syntax tree and source code behind it into string.
///

#[ macro_export ]
macro_rules! tree_export_str
{
  ( $src : expr ) =>
  {{
    let src2 = &$src;
    format!( "{} : {} :\n{:#?}", stringify!( $src ), quote!{ #src2 }, $src )
  }};
}

///
/// Macro to generate syn error either with span of a syntax tree element or with default one `proc_macro2::Span::call_site()`.
///
/// # Sample
/// ```
/// # use wproc_macro::*;
/// syn_err!( "No attr" );
/// # ()
/// ```
///

#[ macro_export ]
macro_rules! syn_err
{

  ( $msg : expr ) =>
  {
    syn::Error::new( proc_macro2::Span::call_site(), $msg )
  };
  ( $span : expr, $msg : expr ) =>
  {
    syn::Error::new( ( $span ).span(), $msg )
  };

}

/// Kind of container.

/* xxx : qqq : for rust : add HashSet */
#[derive( Debug, PartialEq, Copy, Clone )]
pub enum ContainerKind
{
  /// Not a container.
  No,
  /// Vector-like.
  Vector,
  /// Hash map-like.
  HashMap,
}

/// Return kind of container specified by type.
///
/// Good to verify `alloc::vec::Vec< i32 >` is vector.
/// Good to verify `std::collections::HashMap< i32, i32 >` is hash map.
///
/// # Sample
/// ```
/// use wproc_macro::*;
/// use quote::quote;
///
/// let code = quote!( std::collections::HashMap< i32, i32 > );
/// let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
/// let got = type_container_kind( &tree_type );
/// assert_eq!( got, ContainerKind::HashMap );
/// ```

pub fn type_container_kind( ty : &syn::Type ) -> ContainerKind
{
  if let syn::Type::Path( path ) = ty
  {
    let last = &path.path.segments.last();
    if last.is_none()
    {
      return ContainerKind::No
    }
    match last.unwrap().ident.to_string().as_ref()
    {
      "Vec" => { return ContainerKind::Vector }
      "HashMap" => { return ContainerKind::HashMap }
      _ => { return ContainerKind::No }
    }
  }
  ContainerKind::No
}

/// Check is the rightmost item of path refering a type is specified type.
///
/// Good to verify `core::option::Option< i32 >` is optional.
/// Good to verify `alloc::vec::Vec< i32 >` is vector.
///
/// # Sample
/// ```
/// use wproc_macro::*;
/// use quote::quote;
///
/// let code = quote!( core::option::Option< i32 > );
/// let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
/// let got = type_rightmost( &tree_type );
/// assert_eq!( got, Some( "Option".to_string() ) );
/// ```

// pub fn type_rightmost< R : AsRef< str > >( ty : &syn::Type, rightmost : R ) -> bool
pub fn type_rightmost( ty : &syn::Type ) -> Option< String >
{
  if let syn::Type::Path( path ) = ty
  {
    let last = &path.path.segments.last();
    if last.is_none()
    {
      return None;
      // return false;
    }
    return Some( last.unwrap().ident.to_string() );
    // return last.unwrap().ident == rightmost.as_ref();
  }
  // false
  None
}

pub use crate::interval::*;

/// Return the specified number of parameters of the type.
///
/// Good to getting `i32` from `core::option::Option< i32 >` or `alloc::vec::Vec< i32 >`
///
/// # Sample
/// ```
/// use wproc_macro::*;
/// use quote::quote;
///
/// let code = quote!( core::option::Option< i8, i16, i32, i64 > );
/// let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
/// let got = type_parameters( &tree_type, 0..=2 );
/// got.iter().for_each( | e | println!( "{}", quote!( #e ) ) );
/// // < i8
/// // < i16
/// // < i32
/// ```

pub fn type_parameters< R >( ty : &syn::Type, range : R ) -> Vec< &syn::Type >
where
  R : std::convert::Into< crate::interval::Interval >
{
  let range = range.into();
  if let syn::Type::Path( syn::TypePath{ path : syn::Path { ref segments, .. }, .. } ) = ty
  {
    let last = &segments.last();
    if last.is_none()
    {
      return vec![ &ty ]
    }
    let args = &last.unwrap().arguments;
    if let syn::PathArguments::AngleBracketed( ref args2 ) = args
    {
      let args3 = &args2.args;
      let selected : Vec< &syn::Type > = args3
      .iter()
      .skip_while( | e | if let syn::GenericArgument::Type( _ ) = e { false } else { true } )
      .skip( range.first().try_into().unwrap() )
      .take( range.len().try_into().unwrap() )
      .map( | e | if let syn::GenericArgument::Type( ty ) = e { ty } else { unreachable!( "Expects Type" ) } )
      .collect();
      return selected;
    }
  }
  vec![ &ty ]
}

///
/// For attribute like `#[former( default = 31 )]` return key `default` and value `31`,
/// as well as syn::Meta as the last element of result tuple.
///
/// # Sample
/// ``` ignore
/// let ( key, val, meta ) = attr_pair_single( &attr )?;
/// ```

pub fn attr_pair_single( attr : &syn::Attribute ) -> Result< ( String, syn::Lit, syn::Meta ), syn::Error >
{
  use syn::spanned::Spanned;
  let meta = attr.parse_meta()?;

  let ( key, val );
  match meta
  {
    syn::Meta::List( ref meta_list ) =>
    match meta_list.nested.first()
    {
      Some( nested_meta ) => match nested_meta
      {
        syn::NestedMeta::Meta( meta2 ) => match meta2
        {
          syn::Meta::NameValue( name_value ) => // match &name_value.lit
          {
            if meta_list.nested.len() != 1
            {
              return Err( syn::Error::new( attr.span(), format!( "Expected single element of the list, but got {}", meta_list.nested.len() ) ) );
            }
            key = name_value.path.get_ident().unwrap().to_string();
            val = name_value.lit.clone();
          },
          _ => return Err( syn::Error::new( attr.span(), "Unknown format of attribute, expected syn::Meta::NameValue( name_value )" ) ),
        },
        _ => return Err( syn::Error::new( attr.span(), "Unknown format of attribute, expected syn::NestedMeta::Meta( meta2 )" ) ),
      },
      _ => return Err( syn::Error::new( attr.span(), "Unknown format of attribute, expected Some( nested_meta )" ) ),
    },
    _ => return Err( syn::Error::new( attr.span(), "Unknown format of attribute, expected syn::Meta::List( meta_list )" ) ),
  };

  Ok( ( key, val, meta ) )
}

//

// don't delete!
// good example of overloading to reuse
//
// pub use syn::spanned::Spanned;
//
// /// Trait to implement method span() for those structures which [module::syn](https://docs.rs/syn/latest/syn/spanned/index.html) do not have it implemented.
//
// pub trait Spanned2
// {
//   /// Returns a Span covering the complete contents of this syntax tree node, or Span::call_site() if this node is empty.
//   fn span2( &self ) -> proc_macro2::Span;
// }
//
// //
//
// impl Spanned2 for syn::Data
// {
//   fn span2( &self ) -> proc_macro2::Span
//   {
//     // data_fields_of( &self ).span()
//     match self
//     {
//       syn::Data::Struct( syn::DataStruct { ref fields, .. } ) => fields.span(),
//       syn::Data::Enum( syn::DataEnum { ref variants, .. } ) => variants.span(),
//       syn::Data::Union( syn::DataUnion { ref fields, .. } ) => fields.span(),
//     }
//   }
// }
//
// impl< T : Spanned2 > Spanned2 for &T
// {
//   fn span2( &self ) -> proc_macro2::Span
//   {
//     ( *self ).span2()
//   }
// }
//
// //
//
// #[ doc( hidden ) ]
// pub struct Data< 'a, T >( &'a T );
//
// #[ doc( hidden ) ]
// pub trait Span1
// {
//   fn act( self ) -> proc_macro2::Span;
// }
//
// impl< 'a, T > Span1
// for Data< 'a, T >
// where T : syn::spanned::Spanned,
// {
//   fn act( self ) -> proc_macro2::Span
//   {
//     self.0.span()
//   }
// }
//
//
// #[ doc( hidden ) ]
// pub trait Span2
// {
//   fn act( self ) -> proc_macro2::Span;
// }
//
// impl< 'a, T > Span2
// for Data< 'a, T >
// where T : Spanned2,
// {
//   fn act( self ) -> proc_macro2::Span
//   {
//     self.0.span2()
//   }
// }
//
// #[ doc( hidden ) ]
// pub fn _span_of< T : Sized >( src : &T ) -> Data< T >
// {
//   Data( src )
// }
//
// // fn span2_of< T : Sized >( src : &T )
// // {
// //   _span_of( src ).act()
// // }
//
// /// Returns a Span covering the complete contents of this syntax tree node, or Span::call_site() if this node is empty.
//
// #[ macro_export ]
// macro_rules! span_of
// {
//   ( $src : expr ) =>
//   {
//     $crate::_span_of( &$src ).act()
//   }
// }
//
// /// Returns a Span covering the complete contents of this syntax tree node, or Span::call_site() if this node is empty.
// ///
// /// Works only for items for which span is not implemented in [module::syn](https://docs.rs/syn/latest/syn/spanned/index.html). For other use macro [`span_of!`](span_of!).
//
// pub fn span_of< Src : Spanned2 >( src : &Src ) -> proc_macro2::Span
// {
//   src.span2()
// }

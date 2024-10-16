#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/meta_tools/latest/meta_tools/" ) ]
// #![ deny( rust_2018_idioms ) ]
// // #![ deny( missing_docs ) ]
// // #![ deny( missing_debug_implementations ) ]

//!
//! Collection of general purpose meta tools.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{

  // #[ cfg( feature = "meta_mod_interface" ) ]
  pub use ::mod_interface;
  #[ cfg( feature = "meta_for_each" ) ]
  pub use ::for_each;
  #[ cfg( feature = "meta_impls_index" ) ]
  pub use ::impls_index;

  #[ cfg( feature = "meta_constructors" ) ]
  pub use ::literally;
  #[ cfg( feature = "meta_idents_concat" ) ]
  pub use ::paste;

  // #[ cfg( feature = "former" ) ]
  // pub use ::former;
  // #[ cfg( feature = "options" ) ]
  // pub use ::woptions;

}

//

// qqq : meta interface should be optional dependancy. please fix writing equivalent code manually
#[ cfg( feature = "enabled" ) ]
mod_interface::mod_interface!
{

  layer meta;

}

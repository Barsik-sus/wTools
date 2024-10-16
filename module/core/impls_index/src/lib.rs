#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/impls_index/latest/impls_index/" ) ]
// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]

//!
//! Several of macros to put each function under a named macro to index every function in a class.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Collection of general purpose meta tools.
// #[ path = "./mod.rs" ]
#[ cfg( feature = "enabled" ) ]
pub mod impls_index;

/// Dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  pub use ::impls_index_meta;
}

/// Protected namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::impls_index::orphan::*;
}

#[ cfg( feature = "enabled" ) ]
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Orphan namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::impls_index::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
pub mod prelude
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::impls_index::prelude::*;
}

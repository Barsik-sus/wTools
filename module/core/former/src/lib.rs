#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/former/latest/former/" ) ]
// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]

// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]

//!
//! Former - variation of builder pattern.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Former - variation of builder pattern. Implementation of its runtime.
pub mod runtime;

/// Namespace with dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  // // #[ cfg( any( feature = "runtime", feature = "former_runtime" ) ) ]
  // pub use former_runtime;
  // #[ cfg( any( feature = "meta", feature = "former_meta" ) ) ]
  pub use former_meta;
}

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  // #[ cfg( any( feature = "runtime", feature = "former_runtime" ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  use super::runtime;
  // pub use former_runtime as runtime;
  // #[ cfg( any( feature = "meta", feature = "former_meta" ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use former_meta as derive;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
  // #[ cfg( any( feature = "meta", feature = "former_meta" ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use former_meta::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}

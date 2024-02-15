// qqq : uncomment the next line? /* aaa : Dmytro : uncommented and tested with each feature */
#![ cfg_attr( not( feature = "use_std" ), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico")]
#![ doc( html_root_url = "https://docs.rs/fundamental_data_type/latest/fundamental_data_type/")]
// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]

//!
//! Fundamental data types and type constructors, like Single, Pair, Many.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use ::type_constructor::*;

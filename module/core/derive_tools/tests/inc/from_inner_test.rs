use super::*;

// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq, TheModule::FromInner ) ]
pub struct IsTransparent( bool );

// include!( "./manual/basic.rs" );
include!( "./only_test/from_inner.rs" );

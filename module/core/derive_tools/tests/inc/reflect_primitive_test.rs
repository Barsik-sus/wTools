use super::*;
pub use TheModule::reflect;

#[ test ]
fn data_basic()
{
  use reflect::Primitive;

  let got = Primitive::i32( 13i32 );
  a_id!( got, Primitive::i32( 13i32 ) );

}

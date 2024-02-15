#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn from_named_fields()
{

  #[ derive( Debug, PartialEq ) ]
  struct StructNamedFields
  {
    a : i32,
    b : i32,
    c : i32,
    d : i32,
  }

  impl TheModule::wtools::From_0 for StructNamedFields
  {
    fn from_0() -> Self
    {
      let a = Default::default();
      let b = Default::default();
      let c = Default::default();
      let d = Default::default();
      Self{ a, b, c, d }
    }
  }

  impl TheModule::wtools::From_1< i32 > for StructNamedFields
  {
    fn from_1( a : i32 ) -> Self { Self{ a, b : a, c : a, d : a } }
  }

  impl TheModule::wtools::From_2< i32, i32 > for StructNamedFields
  {
    fn from_2( a : i32, b : i32 ) -> Self { Self{ a, b, c : b, d : b } }
  }

  impl TheModule::wtools::From_3< i32, i32, i32 > for StructNamedFields
  {
    fn from_3( a : i32, b : i32, c : i32 ) -> Self { Self{ a, b, c, d : c } }
  }

  let got : StructNamedFields = TheModule::from!();
  let exp = StructNamedFields{ a : 0, b : 0, c : 0, d : 0 };
  a_id!( got, exp );

  let got : StructNamedFields = TheModule::from!( 13 );
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : StructNamedFields = TheModule::from!( 0, 1 );
  let exp = StructNamedFields{ a : 0, b : 1, c : 1, d : 1 };
  a_id!( got, exp );

  let got : StructNamedFields = TheModule::from!( 0, 1, 2 );
  let exp = StructNamedFields{ a : 0, b : 1, c : 2, d : 2 };
  a_id!( got, exp );

}

//

#[ test ]
fn from_tuple()
{

  #[ derive( Debug, PartialEq ) ]
  struct StructTuple( i32, i32, i32, i32 );

  impl TheModule::wtools::From_0 for StructTuple
  {
    fn from_0() -> Self
    {
      let a = Default::default();
      let b = Default::default();
      let c = Default::default();
      let d = Default::default();
      Self( a, b, c, d )
    }
  }

  impl TheModule::wtools::From_1< i32 > for StructTuple
  {
    fn from_1( a : i32 ) -> Self { Self( a, a, a, a ) }
  }

  impl TheModule::wtools::From_2< i32, i32 > for StructTuple
  {
    fn from_2( a : i32, b : i32 ) -> Self { Self( a, b, b, b ) }
  }

  impl TheModule::wtools::From_3< i32, i32, i32 > for StructTuple
  {
    fn from_3( a : i32, b : i32, c : i32 ) -> Self { Self( a, b, c, c ) }
  }

  let got : StructTuple = TheModule::from!();
  let exp = StructTuple( 0, 0, 0, 0 );
  a_id!( got, exp );

  let got : StructTuple = TheModule::from!( 13 );
  let exp = StructTuple( 13, 13, 13, 13 );
  a_id!( got, exp );

  let got : StructTuple = TheModule::from!( 0, 1 );
  let exp = StructTuple( 0, 1, 1, 1 );
  a_id!( got, exp );

  let got : StructTuple = TheModule::from!( 0, 1, 2 );
  let exp = StructTuple( 0, 1, 2, 2 );
  a_id!( got, exp );

}

//

/// From_0 is auto implemented from Default.
#[ test ]
fn from0_from_default()
{

  #[ derive( Debug, PartialEq, Default ) ]
  struct StructNamedFields
  {
    a : i32,
    b : i32,
  }

  // impl TheModule::wtools::From_0 for StructNamedFields
  // {
  //   fn from_0() -> Self
  //   {
  //     let a = Default::default();
  //     let b = Default::default();
  //     Self{ a, b }
  //   }
  // }

  let got : StructNamedFields = TheModule::from!();
  let exp = StructNamedFields{ a : 0, b : 0 };
  a_id!( got, exp );

  let got : StructNamedFields = TheModule::From_0::from_0();
  let exp = StructNamedFields{ a : 0, b : 0 };
  a_id!( got, exp );

  let got : StructNamedFields = Default::default();
  let exp = StructNamedFields{ a : 0, b : 0 };
  a_id!( got, exp );

}

//

/// Into1 is auto implemented from From_1.
/// From_1< ( All, ) > is auto implemented for From_1< All >.
#[ test ]
fn from_tuple_from_from1()
{
  use TheModule::prelude::*;

  #[ derive( Debug, PartialEq, Default ) ]
  struct StructNamedFields
  {
    a : i32,
    b : i32,
    c : i32,
    d : i32,
  }

  impl TheModule::wtools::From_1< i32 > for StructNamedFields
  {
    fn from_1( a : i32 ) -> Self { Self{ a, b : a, c : a, d : a } }
  }

  let got : StructNamedFields = from!( 13 );
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : StructNamedFields = StructNamedFields::from_1( 13 );
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : StructNamedFields = from!( ( 13, ) );
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : StructNamedFields = StructNamedFields::from_1( ( 13, ) );
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : StructNamedFields = from!( ( ( 13, ), ) );
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : StructNamedFields = StructNamedFields::from_1( ( ( 13, ), ) );
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : StructNamedFields = 13.to();
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : StructNamedFields = ( 13, ).to();
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : StructNamedFields = ( ( 13, ), ).to();
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

}

//

/// Into1 is auto implemented from From_1.
/// From_1< ( All1, All2 ) > is auto implemented for From_2< All1, All2 >.
#[ test ]
fn from_tuple_from_from2()
{
  use TheModule::prelude::*;

  #[ derive( Debug, PartialEq, Default ) ]
  struct StructNamedFields
  {
    a : i32,
    b : i32,
    c : i32,
    d : i32,
  }

  impl TheModule::wtools::From_2< i32, i32 > for StructNamedFields
  {
    fn from_2( a : i32, b : i32 ) -> Self { Self{ a, b, c : b, d : b } }
  }

  let got : StructNamedFields = from!( 13, 14 );
  let exp = StructNamedFields{ a : 13, b : 14, c : 14, d : 14 };
  a_id!( got, exp );

  let got : StructNamedFields = StructNamedFields::from_2( 13, 14 );
  let exp = StructNamedFields{ a : 13, b : 14, c : 14, d : 14 };
  a_id!( got, exp );

  let got : StructNamedFields = from!( ( 13, 14 ) );
  let exp = StructNamedFields{ a : 13, b : 14, c : 14, d : 14 };
  a_id!( got, exp );

  let got : StructNamedFields = StructNamedFields::from_1( ( 13, 14 ) );
  let exp = StructNamedFields{ a : 13, b : 14, c : 14, d : 14 };
  a_id!( got, exp );

  let got : StructNamedFields = from!( ( ( 13, 14 ), ) );
  let exp = StructNamedFields{ a : 13, b : 14, c : 14, d : 14 };
  a_id!( got, exp );

  let got : StructNamedFields = StructNamedFields::from_1( ( ( 13, 14 ), ) );
  let exp = StructNamedFields{ a : 13, b : 14, c : 14, d : 14 };
  a_id!( got, exp );

  let got : StructNamedFields = ( 13, 14 ).to();
  let exp = StructNamedFields{ a : 13, b : 14, c : 14, d : 14 };
  a_id!( got, exp );

  let got : StructNamedFields = ( ( 13, 14 ), ).to();
  let exp = StructNamedFields{ a : 13, b : 14, c : 14, d : 14 };
  a_id!( got, exp );

}

//

/// Into1 is auto implemented from From_1.
/// From_1< ( All1, All2, All3 ) > is auto implemented for From_3< All1, All2, All3 >.
#[ test ]
fn from_tuple_from_from3()
{
  use TheModule::prelude::*;

  #[ derive( Debug, PartialEq, Default ) ]
  struct StructNamedFields
  {
    a : i32,
    b : i32,
    c : i32,
    d : i32,
  }

  impl TheModule::wtools::From_3< i32, i32, i32 > for StructNamedFields
  {
    fn from_3( a : i32, b : i32, c : i32 ) -> Self { Self{ a, b, c, d : c } }
  }

  let got : StructNamedFields = from!( 13, 14, 15 );
  let exp = StructNamedFields{ a : 13, b : 14, c : 15, d : 15 };
  a_id!( got, exp );

  let got : StructNamedFields = StructNamedFields::from_3( 13, 14, 15 );
  let exp = StructNamedFields{ a : 13, b : 14, c : 15, d : 15 };
  a_id!( got, exp );

  let got : StructNamedFields = from!( ( 13, 14, 15 ) );
  let exp = StructNamedFields{ a : 13, b : 14, c : 15, d : 15 };
  a_id!( got, exp );

  let got : StructNamedFields = StructNamedFields::from_1( ( 13, 14, 15 ) );
  let exp = StructNamedFields{ a : 13, b : 14, c : 15, d : 15 };
  a_id!( got, exp );

  let got : StructNamedFields = from!( ( ( 13, 14, 15 ), ) );
  let exp = StructNamedFields{ a : 13, b : 14, c : 15, d : 15 };
  a_id!( got, exp );

  let got : StructNamedFields = StructNamedFields::from_1( ( ( 13, 14, 15 ), ) );
  let exp = StructNamedFields{ a : 13, b : 14, c : 15, d : 15 };
  a_id!( got, exp );

  let got : StructNamedFields = ( 13, 14, 15 ).to();
  let exp = StructNamedFields{ a : 13, b : 14, c : 15, d : 15 };
  a_id!( got, exp );

  let got : StructNamedFields = ( ( 13, 14, 15 ), ).to();
  let exp = StructNamedFields{ a : 13, b : 14, c : 15, d : 15 };
  a_id!( got, exp );

}

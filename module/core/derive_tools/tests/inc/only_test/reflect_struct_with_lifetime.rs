#[ test ]
fn reflect_struct_with_lifetime()
{
  use reflect::{ Instance, Entity };

  // assumptions
  a_id!( core::any::TypeId::of::< &'static str >(), core::any::TypeId::of::< &str >() );

  // structure
  let x = 1;
  let z = "3";
  let ins = Struct1
  {
    f1 : &x,
    f2 : 2,
    f3 : &z,
  };

  // for understanding
  println!( "TypeId< i32 > : {:?}", core::any::TypeId::of::< i32 >() );
  println!( "TypeId< &i32 > : {:?}", core::any::TypeId::of::< & i32 >() ); // qqq : qqq  fro Yuliia : problem. should be distinct id
  println!( "TypeId< String > : {:?}", core::any::TypeId::of::< String >() );
  println!( "TypeId< &String > : {:?}", core::any::TypeId::of::< & String >() );
  println!( "TypeId< str > : {:?}", core::any::TypeId::of::< str >() );
  println!( "TypeId< &str > : {:?}", core::any::TypeId::of::< & str >() );

  println!( "i32 : {:?}", 1i32.reflect().type_id() );
  println!( "&i32 : {:?}", ( &1i32 ).reflect().type_id() );
  println!( "String : {:?}", "abc".to_string().reflect().type_id() );
  println!( "&String : {:?}", ( &"abc".to_string() ).reflect().type_id() );
  println!( "str : {:?}", "abc".reflect().type_id() );
  println!( "&str : {:?}", ( &"abc" ).reflect().type_id() );
  println!( "Struct1 : {:?}", ins.reflect().type_id() );
  println!( "Struct1.f1 : {:?}", ins.reflect().elements().next().unwrap().val.type_id() );
  println!( "Struct1.f2 : {:?}", ins.reflect().elements().skip( 1 ).next().unwrap().val.type_id() );
  println!( "Struct1.f3 : {:?}", ins.reflect().elements().skip( 2 ).next().unwrap().val.type_id() );

  println!( "i32.type_id : {:?}", 1i32.reflect().type_id() );
  println!( "i32.type_name : {:?}", 1i32.reflect().type_name() );
  println!( "&i32.type_id : {:?}", ( &1i32 ).reflect().type_id() );
  println!( "&i32.type_name : {:?}", ( &1i32 ).reflect().type_name() );
  println!( "&i32.type_id : {:?}", reflect::Instance::reflect( &1i32 ).type_id() );
  println!( "&i32.type_name : {:?}", reflect::Instance::reflect( &1i32 ).type_name() );

  // inspection of structure
  a_id!( ins.reflect().is_container(), true );
  a_id!( ins.reflect().len(), 3 );
  a_id!( ins.reflect().type_name(), "derive_tests::inc::reflect_struct_with_lifetime_manual_test::Struct1" );
  a_id!( ins.reflect().type_id(), core::any::TypeId::of::< Struct1< 'static, 'static > >() );
  let names = ins.reflect().elements().map( | e | e.key ).collect::< Vec< _ > >();
  a_id!( names, vec![ reflect::Primitive::str( "f1" ), reflect::Primitive::str( "f2" ), reflect::Primitive::str( "f3" ) ] );
  let types = ins.reflect().elements().map( | e | e.val.type_name() ).collect::< Vec< _ > >();
  a_id!( types, vec![ "&i32", "i32", "&str" ] );

  // inspection of a field
  let f1 = ins.reflect().elements().next().unwrap();
  a_id!( f1.key, reflect::Primitive::str( "f1" ) );
  a_id!( f1.val.is_container(), false );
  a_id!( f1.val.len(), 0 );
  a_id!( f1.val.type_name(), "&i32" );
  a_id!( f1.val.type_id(), core::any::TypeId::of::< &'static i32 >() );
  a_id!( f1.val.elements().collect::< Vec< _ > >(), vec![] );

}

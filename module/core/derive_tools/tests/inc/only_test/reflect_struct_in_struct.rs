#[ test ]
fn reflect_struct_in_struct()
{
  use reflect::{ Instance, Entity };

  let ins = Struct1
  {
    f1 : 1,
    f2 : "2".into(),
    f3 : Struct2 { s1 : 10, s2 : "20".into(), s3 : "30" },
  };

  a_id!( ins.reflect().is_container(), true );
  a_id!( ins.reflect().len(), 3 );
  a_id!( ins.reflect().type_name(), "derive_tests::inc::reflect_struct_in_struct_manual_test::Struct1" );
  let names = ins.reflect().elements().map( | e | e.key ).collect::< Vec< _ > >();
  a_id!( names, vec![ reflect::Primitive::str( "f1" ), reflect::Primitive::str( "f2" ), reflect::Primitive::str( "f3" ) ] );
  let types = ins.reflect().elements().map( | e | e.val.type_name() ).collect::< Vec< _ > >();
  a_id!( types, vec![ "i32", "alloc::string::String", "derive_tests::inc::reflect_struct_in_struct_manual_test::Struct2" ] );

  let f3 = ins.reflect().elements().skip( 2 ).next().unwrap();
  a_id!( f3.key, reflect::Primitive::str( "f3" ) );
  a_id!( f3.val.is_container(), true );
  a_id!( f3.val.len(), 3 );
  a_id!( f3.val.type_name(), "derive_tests::inc::reflect_struct_in_struct_manual_test::Struct2" );
  let names = f3.val.elements().map( | e | e.key ).collect::< Vec< _ > >();
  a_id!( names, vec![ reflect::Primitive::str( "s1" ), reflect::Primitive::str( "s2" ), reflect::Primitive::str( "s3" ) ] );
  let types = f3.val.elements().map( | e | e.val.type_name() ).collect::< Vec< _ > >();
  a_id!( types, vec![ "i32", "alloc::string::String", "&str" ] );

}

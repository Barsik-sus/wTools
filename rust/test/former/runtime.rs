
// use std::env;

// #[test]
// fn trybuild_tests()
// {
//   println!( "current_dir : {:?}", env::current_dir().unwrap() );
//   // let t = trybuild::TestCases::new();
//   // t.pass( "rust/test/former/test/basic_manual.rs" );
// }

include!( "./all/basic_manual.rs" );
include!( "./all/string_slice_manual.rs" );

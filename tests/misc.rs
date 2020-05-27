// Testing... whatever
use tear::prelude::*;

#[test] fn gut_maru () {
	fn f () -> Option<i32> {
		terror! { None => tear::gut };
		Some(5)
	}
	assert_eq![ f(), None ];
}

#[cfg(not(feature = "experimental"))]
#[test] fn option_from_unit () {
	fn f () -> Option<i32> {
		terror! { None => |_| () };
		Some(5)
	}
	assert_eq![ f(), None ];
}

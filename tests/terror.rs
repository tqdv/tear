// Testing `terror!`, pretty short because we use a lot of doctests
#![cfg_attr(feature = "experimental", feature(try_trait))]

use tear::prelude::*;

// Difference between the implementations of Judge for Option between standard and "experimental"

#[cfg(not(feature = "experimental"))]
fn f () -> Option<i32> {
	terror! { Err(1) => |_| () };
	Some(1)
}

#[cfg(feature = "experimental")]
fn f () -> Option<i32> {
	use std::option::NoneError;
	terror! { Err(1) => |_| NoneError };
	Some(1)
}

#[test] fn return_none () {
	assert_eq![ f(), None ];
}

// Test if terror! { $e } automatically converts its argument correctly

#[derive(Debug, PartialEq)]
struct MyInt {
	v :i32
}

impl From<i32> for MyInt {
	fn from (v :i32) -> MyInt {
		MyInt { v }
	}
}

#[test] fn terror_from () {
	fn f () -> Result<(), MyInt> {
		terror! { Err(0) };
		Ok(())
	}
	
	assert_eq![ f(), Err(MyInt { v: 0 }) ];
}

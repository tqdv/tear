// Testing the "experimental" features
#![cfg(feature = "experimental")]

#![feature(try_trait)]

use tear::prelude::*;
use tear::Maru;
use std::ops::Try;

fn try_val () -> Option<i32> {
	let v = Val::<_, Maru>(3)?;
	Some(v)
}

fn try_ret () -> Option<i32> {
	let v = Ret(5).val()?;
	Some(v)
}

#[test]
fn test_all () {
	assert_eq![ try_val(), Some(3) ];
	assert_eq![ try_ret(), None ];
}

/* Test if implementing Try does automatically implement Judge */

#[derive(Debug, PartialEq)]
struct PendingMessage {
	status :bool,
	data: Option<String>,
}

impl Try for PendingMessage {
	type Ok = String;
	type Error = ();

	fn into_result (self) -> Result<String, ()> {
		match self {
			PendingMessage { status: false, .. } => Err(()),
			PendingMessage { data: None, .. } => Err(()),
			PendingMessage { status: true, data: Some(v)} => Ok(v),
		}
	}

	fn from_error(_: ()) -> Self {
		PendingMessage { status: false, data: None }
	}

	fn from_ok(v: String) -> Self {
		PendingMessage { status: true, data: Some(v) }
	}
}

impl_judge_from_try!(PendingMessage);

#[test] fn implemented_try () {
	fn f() -> PendingMessage {
		terror! { None => |_| () };

		PendingMessage { status: true, data: Some("sip".to_string())}
	}

	assert_eq![ f(), PendingMessage { status: false, data: None } ];
}

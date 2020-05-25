#![cfg(feature = "experimental")]

use tear::ValRet::{self, *};

fn try_val () -> Option<i32> {
	let v = Val(3)?;
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
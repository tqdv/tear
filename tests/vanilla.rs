// Testing the crate with no experimental features
#![cfg(not(feature = "experimental"))]

use tear::prelude::*;
use tear::Moral;

/* Test if Judge automatically implements Return */

enum AB<T, U> {
	A(T),
	B(U),
}

impl<T, U> tear::Judge for AB<T, U> {
	type Positive = T;
	type Negative = U;

	fn into_moral (self) -> Moral<T, U> {
		match self {
			AB::A(v) => Moral::Good(v),
			AB::B(v) => Moral::Bad(v),
		}
	}

	fn from_good (v :T) -> Self { AB::A(v) }
	fn from_bad (v :U) -> Self { AB::B(v) }
}

#[test] fn judge_to_return () {
	fn f () -> i32 {
		tear! { AB::A::<_, i32>(5) };
		tear! { AB::B::<_, i32>(6) };
		0
	}
	assert_eq![ f(), 6 ];
}

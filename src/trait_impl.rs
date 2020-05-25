/*! (dev) Implementation of the Judge and Moral traits for common types

This module implements in order
- Judge for Option, Result, ValRet and Moral
- Return for impl Judge
- (f="experimental") Try for ValRet and Moral
*/
use crate::*;

// Implementation of Judge for Option, Result, ValRet and Moral

/// Implementation of Judge for Option based on its implementation of Try
impl<T> Judge for Option<T> {
	type Positive = T;
	type Negative = ();

	fn into_moral (self) -> Moral<T, ()> {
		match self {
			Some(v) => Good(v),
			None => Bad(()),
		}
	}

	fn from_good(v :T) -> Self { Some(v) }
	fn from_bad(_ :()) -> Self { None }
}

/// Implementation of Judge for Result based on its implementation of Try
impl<T, E> Judge for Result<T, E> {
	type Positive = T;
	type Negative = E;

	fn into_moral (self) -> Moral<T, E> {
		match self {
			Ok(v) => Good(v),
			Err(e) => Bad(e),
		}
	}

	fn from_good (v :T) -> Self { Ok(v) }
	fn from_bad (v :E) -> Self { Err(v) }
}

impl<T, R> Judge for ValRet<T, R> {
	type Positive = T;
	type Negative = R;

	fn into_moral (self) -> Moral<T, R> {
		match self {
			Val(v) => Good(v),
			Ret(r) => Bad(r),
		}
	}

	fn from_good (v :T) -> Self { Val(v) }
	fn from_bad (r :R) -> Self { Ret(r) }
}

impl<Y, N> Judge for Moral<Y, N> {
	type Positive = Y;
	type Negative = N;

	fn into_moral (self) -> Moral<Y, N> { self }

	fn from_good (v :Y) -> Self { Good(v) }
	fn from_bad (v :N) -> Self { Bad(v) }
}

// Implement Return for types that implement Judge

/** Automatic implementation of Return for types that implement the Judge trait

`Moral` can convert to `ValRet` and the `Judge` trait lets you convert to `Moral`.
*/
impl<T, E, Me> Return for Me where Me :Judge<Positive = T, Negative = E> {
	type Value = T;
	type Returned = E;
	fn valret (self) -> ValRet<T, E> {
		self.into_moral().into_valret()
	}
}

// Implementation of Try for ValRet and Moral

#[cfg(feature = "experimental")]
impl<T, R> std::ops::Try for ValRet<T, R> {
	type Ok = T;
	type Error = R;

	fn into_result (self) -> Result<T, R> {
		self.into_moral().into_result()
	}

	fn from_ok (v :T) -> Self { Val(v) }
	fn from_error (v :R) -> Self { Ret(v) }
}

#[cfg(feature = "experimental")]
impl<Y, N> std::ops::Try for Moral<Y, N> {
	type Ok = Y;
	type Error = N;

	fn into_result (self) -> Result<Y, N> {
		self.into_moral().into_result()
	}

	fn from_ok (v :Y) -> Self { Good(v) }
	fn from_error (v :N) -> Self { Bad(v) }
}
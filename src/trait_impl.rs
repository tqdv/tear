/*! Implementation of the Judge and Moral traits for common types

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

	#[inline]
	fn into_moral (self) -> Moral<T, ()> {
		match self {
			Some(v) => Good(v),
			None => Bad(()),
		}
	}

	#[inline] fn from_good(v :T) -> Self { Some(v) }
	#[inline] fn from_bad(_ :()) -> Self { None }
}

/// Implementation of Judge for Result based on its implementation of Try
impl<T, E> Judge for Result<T, E> {
	type Positive = T;
	type Negative = E;

	#[inline]
	fn into_moral (self) -> Moral<T, E> {
		match self {
			Ok(v) => Good(v),
			Err(e) => Bad(e),
		}
	}

	#[inline] fn from_good (v :T) -> Self { Ok(v) }
	#[inline] fn from_bad (v :E) -> Self { Err(v) }
}

impl<T, R> Judge for ValRet<T, R> {
	type Positive = T;
	type Negative = R;

	#[inline]
	fn into_moral (self) -> Moral<T, R> {
		match self {
			Val(v) => Good(v),
			Ret(r) => Bad(r),
		}
	}

	#[inline] fn from_good (v :T) -> Self { Val(v) }
	#[inline] fn from_bad (r :R) -> Self { Ret(r) }
}

impl<Y, N> Judge for Moral<Y, N> {
	type Positive = Y;
	type Negative = N;

	#[inline]
	fn into_moral (self) -> Moral<Y, N> { self }

	#[inline] fn from_good (v :Y) -> Self { Good(v) }
	#[inline] fn from_bad (v :N) -> Self { Bad(v) }
}

// Implementation of Return for those that implement Judge

/// Automatic implementation of Return for types that can convert to Moral (Judge trait)
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

	#[inline] fn into_result (self) -> Result<T, R> {
		match self {
			Val(v) => Ok(v),
			Ret(r) => Err(r),
		}
	}

	#[inline] fn from_ok (v :T) -> Self { Val(v) }
	#[inline] fn from_error (v :R) -> Self { Ret(v) }
}

#[cfg(feature = "experimental")]
impl<Y, N> std::ops::Try for Moral<Y, N> {
	type Ok = Y;
	type Error = N;

	#[inline] fn into_result (self) -> Result<Y, N> {
		match self {
			Good(v) => Ok(v),
			Bad(v) => Err(v),
		}
	}

	#[inline] fn from_ok (v :Y) -> Self { Good(v) }
	#[inline] fn from_error (v :N) -> Self { Bad(v) }
}
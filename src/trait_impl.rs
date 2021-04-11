/*! (dev) Implementation of the Judge and Moral traits for common types

This module implements in order
- Maru <-> ()
- Return for impl Judge
- Normal case:
  - Judge for Option, Result, ValRet and Moral
- If using the "experimental" feature flag:
  - Try for ValRet and Moral
  - `impl_judge_from_try!`
  - Judge for Option, Result, Moral and ValRet
  - Maru -> NoneError
*/
use crate::*;

/** A placeholder type with a single value ◯

It mirrors the [`NoneError`](`core::option::NoneError`) type. For example, it is used in conjunction with [`Moral`] to
represent the bad types for `bool` or `Option<T>`.

# Examples

Return `Maru` from the right hand function to return None:
```
# use tear::prelude::*;
fn f() -> Option<i32> {
    terror! { Some(3) => |_| { println!("Darn"); tear::Maru } };
    # Some(3)
}
```

It automatically converts to `()`:
```
# use tear::prelude::*;
fn f() -> () {
    tear! { None => |_| tear::Maru }
}
```

# See also

- the [`gut`] function, that takes over the right-hand side
*/
#[derive(Copy, Debug, Clone)]
pub struct Maru;

// Equivalence to ()

impl From<()> for Maru {
	fn from(_ :()) -> Self { Maru }
}

impl From<Maru> for () {
	#[allow(clippy::unused_unit)]
	fn from(_ :Maru) -> () { () }
}

impl Judge for bool {
	type Positive = Maru;
	type Negative = Maru;

	fn into_moral (self) -> Moral<Maru, Maru> {
		if self { Good(Maru) }
		else { Bad(Maru) }
	}

	fn from_good (_ :Maru) -> Self { true }
	fn from_bad (_ :Maru) -> Self { false }
}

/// Blanket implementation of Return for types that implement Judge
impl<T, E, Me> Return for Me where Me: Judge<Positive=T, Negative=E> {
	type Value = T;
	type Returned = E;

	fn into_valret(self) -> ValRet<T, E> {
		self.into_moral().into_valret()
	}
}

/// Normal Implementations
#[cfg(not(feature = "experimental"))]
mod independent {
	use crate::*;

	/* Implementation of Judge for Option, Result, ValRet and Moral */

	/// Implementation of Judge for Option based on its implementation of Try
	impl<T> Judge for Option<T> {
		type Positive = T;
		type Negative = Maru;

		fn into_moral(self) -> Moral<T, Maru> {
			match self {
				Some(v) => Good(v),
				None => Bad(Maru),
			}
		}

		fn from_good(v: T) -> Self { Some(v) }
		fn from_bad(_: Maru) -> Self { None }
	}

	/// Implementation of Judge for Result based on its implementation of Try
	impl<T, E> Judge for Result<T, E> {
		type Positive = T;
		type Negative = E;

		fn into_moral(self) -> Moral<T, E> {
			match self {
				Ok(v) => Good(v),
				Err(e) => Bad(e),
			}
		}

		fn from_good(v: T) -> Self { Ok(v) }
		fn from_bad(v: E) -> Self { Err(v) }
	}

	impl<T, R> Judge for ValRet<T, R> {
		type Positive = T;
		type Negative = R;

		fn into_moral(self) -> Moral<T, R> {
			match self {
				Val(v) => Good(v),
				Ret(r) => Bad(r),
			}
		}

		fn from_good(v: T) -> Self { Val(v) }
		fn from_bad(r: R) -> Self { Ret(r) }
	}

	impl<Y, N> Judge for Moral<Y, N> {
		type Positive = Y;
		type Negative = N;

		fn into_moral(self) -> Moral<Y, N> { self }

		fn from_good(v: Y) -> Self { Good(v) }
		fn from_bad(v: N) -> Self { Bad(v) }
	}
}

/// Implementations based on experimental features (`try_trait`)
#[cfg(feature = "experimental")]
mod nightly {
	use core::ops::Try;
	use core::option::NoneError;
	use crate::*;

	/* Implementations of Try for ValRet and Moral */

	impl<T, R> Try for ValRet<T, R> {
		type Ok = T;
		type Error = R;

		fn into_result(self) -> Result<T, R> {
			match self {
				Val(v) => Ok(v),
				Ret(r) => Err(r),
			}
		}

		fn from_ok(v: T) -> Self { Val(v) }
		fn from_error(v: R) -> Self { Ret(v) }
	}

	impl<Y, N> Try for Moral<Y, N> {
		type Ok = Y;
		type Error = N;

		fn into_result(self) -> Result<Y, N> {
			Self::into_result(self)
		}

		fn from_ok(v: Y) -> Self { Good(v) }
		fn from_error(v: N) -> Self { Bad(v) }
	}

	/** Implement Judge for a type that implements Try

	Give it the type (`Option<T>`), and the generic type parameters (`T`).

	```text
	impl_judge_from_try!(Result<T, U>, T, U);
	```
	*/
	#[macro_export]
	macro_rules! impl_judge_from_try {
		( $t:ty $(, $i:ident)* $(,)? ) => {
			impl<__Y, __N $(, $i)* > $crate::Judge for $t where $t :core::ops::Try<Ok=__Y, Error=__N> {
				type Positive = __Y;
				type Negative = __N;

				fn into_moral(self) -> $crate::Moral<__Y, __N> {
					match core::ops::Try::into_result(self) {
						Ok(v) => $crate::Moral::Good(v),
						Err(e) => $crate::Moral::Bad(e),
					}
				}

				fn from_good(v: __Y) -> Self { core::ops::Try::from_ok(v) }
				fn from_bad(v: __N) -> Self { core::ops::Try::from_error(v) }
			}
		}
	}

	impl_judge_from_try!(Option<T>, T);
	impl_judge_from_try!(Result<T, U>, T, U);
	impl_judge_from_try!(Moral<T, U>, T, U);
	impl_judge_from_try!(ValRet<T, U>, T, U);

	/// Conversion for creating None with `terror!`
	impl From<Maru> for NoneError {
		fn from (_ :Maru) -> Self { NoneError }
	}
}

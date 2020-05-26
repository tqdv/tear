/*! **Typed early returns and loop control + Syntax sugar for try!-like error handling**

*Works with Rust v1.34+ (released on 11 April 2019)*

# Getting started

The main focus of this crate are the following three macros:
- `tear!` is used with `ValRet` for typed early returns.
- `terror!` is syntax-sugar for `try!` or the `?` operator.
- `twist!` works with `Looping` to implement typed loop control.

Look at the synopsis for a general idea of what is possible,
and then read the documentation for the macro that interests you.

Otherwise, read the `overview` module documentation that mentions *all* the things in this crate.

## Feature flags

- The "experimental" crate feature enables support for the experimental `Try` trait. But it breaks
  the following syntax: `terror! { $e => $f }` in a function returning `Option<T>`
  with `$f` returning `()`. Return `NoneError` instead.

- The "combinators" crate feature adds the `side` method to the `Judge` trait. It lets you convert
  to `Either` any type that implements `Judge`. You can then use `Either`'s combinators to do
  what you want.

- (dev) "ignore-ui" lets you ignore error message tests because all of them are wrong as soon
  as you have any warnings.

## Synopsis

Import the macros into your module:
```rust
use tear::prelude::*;
```

Explicit error-handling syntax with `terror!`:
```rust
# use tear::prelude::*;
# use std::io::{self, ErrorKind};
# fn can_error () -> Result<i32, CustomError> { Ok(1) }
# fn can_io_error () -> io::Result<i32> { Err(io::Error::new(ErrorKind::Other, "nope")) }
# fn print_error<T> (_ :T) -> CustomError { CustomError::Str("a".to_string()) }
# enum CustomError {
#     Io(io::Error),
#     Str(String)
# }
# fn f() -> Result<i32, CustomError> {
let handled = terror! { can_error() => print_error };
let variant = terror! { can_io_error() => CustomError::Io };
# Ok(1)
# }

// Equivalent using `?`:
# fn g() -> Result<i32, CustomError> {
let handled = can_error().map_err(print_error)?;
let variant = can_io_error().map_err(CustomError::Io)?;
# Ok(2)
# }
```

Early loop continue with `twist!`:
```
# use tear::extra::*;
# struct Regex {}
# impl Regex {
#     fn new(_ :&str) -> Result<Regex, ()> { Err(()) }
# }
# let regexes_strings = vec![ "a", "b" ];
for re in regexes_strings {
    // Skip iteration if the regex fails to compile
    let re = twist! { Regex::new(re) => |_| next!() };

    // Use regex...
# }
```

Keyword-like early returns with `tear_if!`:
```rust
# use tear::prelude::*;
fn divide_i32 (num: i32, denom: i32) -> Option<f32> {
    // Return early if dividing by 0
    tear_if! { denom == 0, None };

    // Compute quotient...
    # None
# }
```

Typed returns with `tear!`:
```rust
# use tear::prelude::*;
// Tells the calling function to return early on failure
fn get_value_or_return() -> ValRet<String, i32> { Ret(-1) }

fn status_code() -> i32 {
    let v = tear! { get_value_or_return() };

    // Process value...
    # 1
# }
```

# See also

- [Error Handling in Rust §The real `try!` macro / `?` operator](https://blog.burntsushi.net/rust-error-handling/#the-real-try-macro-operator)
- [guard](https://docs.rs/crate/guard), a crate implementing "guard" expressions,
  the opposite of `tear_if!`.

Finally, please star the [GitHub repo](https://github.com/tqdv/tear) if you found this crate useful.
It helps developer ego !

# Module documentation

Most things are public to allow easy modification. However, things intended only for module
development are marked as `(dev)`.

In this module, we define in order
- ValRet, its implementation, and its associated trait Return
- Moral, its implementation, and its associated trait Judge
- tear!, tear_if! and terror! macros
*/
#![no_std] // But we use std for tests
#![warn(missing_docs)] // Documentation lints

// Optional features
#![cfg_attr(feature = "experimental", feature(try_trait))]

// Modules
pub mod overview; // For documentation
pub mod prelude;
pub mod extra;
pub mod trait_impl; // Move the trait implementions as they are quite noisy
pub mod twist_impl; // Currently only for `twist!`
#[macro_use] pub mod util; // Utility macros that aren't the main focus. To reduce file size.

// Reexports for macros and convenience
pub use twist_impl::BreakValError;
pub use twist_impl::{BREAKVAL_IN_NOT_LOOP, BREAK_WITHOUT_VAL, BAD_BREAKVAL_TYPE};
pub use twist_impl::Looping;
pub use util::gut;

// For convenience, also used in prelude
use ValRet::*;
use Moral::*;
#[cfg(feature = "combinators")] use either::Either::{self, *};

/** Represents a usable value or an early return. Use with `tear!`

# Description

The idea is to type an early return. The early return either evaluates to something (Val) or
returns early (Ret).
*/
#[derive(PartialEq, Debug, Clone)]
pub enum ValRet<V, R> {
	/// The usable value
	Val(V),
	/// The return value
	Ret(R),
}

/**
**NB**: Other combinators such as `and`, `and_then`, `or`, `map_val`
aren't implemented because I didn't need them and not because they aren't useful.

Examples will all use the following two variables
```
# use tear::prelude::*;
let ok:    ValRet<&str, &str> = Val("ok");
let error: ValRet<&str, &str> = Ret("error");
```
*/
impl<V, R> ValRet<V, R> {
	/* Accessors */

	/// Gets the `Val(V)` variant as `Option<V>`
	pub fn val (self) -> Option<V> { maybe_match! { self, Val(v) => v } }
	/// Gets the `Ret(R)` variant as `Option<R>`
	pub fn ret (self) -> Option<R> { maybe_match! { self, Ret(r) => r } }
}

/// Convert into ValRet
pub trait Return where Self :Sized {
	/// The Val in ValRet
	type Value;
	/// The Ret in ValRet
	type Returned;
	
	/// Convert itself to a ValRet
	fn into_valret (self) -> ValRet<Self::Value, Self::Returned>;
}

/// A notion of good and bad for the `terror!` macro
#[derive(PartialEq, Debug, Clone)]
pub enum Moral<Y, N> {
	/// The good
	Good(Y),
	/// And the bad
	Bad(N),
}

impl<Y, N> Moral<Y, N> {
	/* Accessors */

	/// Gets the `Good(Y)` variant as `Option<Y>`
	pub fn good (self) -> Option<Y> { maybe_match! { self, Good(v) => v } }
	/// Gets the `Bad(N)` variant as `Option<N>`
	pub fn bad (self) -> Option<N> { maybe_match! { self, Bad(v) => v } }

	/* Conversions */

	/** Convert to ValRet

	Maps Good to Val and Bad to Ret.
	*/
	pub fn into_valret (self) -> ValRet<Y, N> {
		match self {
			Good(v) => Val(v),
			Bad(v) => Ret(v),
		}
	}

	/** Convert to Result. Use result instead

	Maps Good to Ok and Bad to Err.
	*/
	pub fn into_result (self) -> Result<Y, N> {
		match self {
			Good(v) => Ok(v),
			Bad(v) => Err(v),
		}
	}

	/** Convert to Result. Use side instead

	Maps Good to Right and Bad to Left.
	*/
	#[cfg(feature = "combinators")]
	pub fn into_either (self) -> Either<N, Y> {
		match self {
			Good(v) => Right(v),
			Bad(v) => Left(v),
		}
	}
	
	/* Special conversions */

	/** Convert to a `ValRet` by mapping Good to Val, and Bad to a wrapped error (Judge trait)
	
	Used in the `terror!` macro when you need to wrap the Bad value into another Bad before returning it.
	See `terror!` documentation.
	*/
	pub fn ret_error<O, R :Judge<Positive=O, Negative=N>> (self) -> ValRet<Y, R> {
		match self {
			Good(v) => Val(v),
			Bad(v) => Ret(Judge::from_bad(v)),
		}
	}

	/** Convert to a `Looping` by mapping Good to Resume, and Bad through a function

	The function `f` takes the bad value and maps it to a `Looping` value.

	Used in the `twist!` macro with the mapping (`=>`) syntax. See `twist!` documentation.
	*/
	pub fn resume_or_else<B> (self, f :impl FnOnce(N) -> Looping<Y, B>) -> Looping<Y, B> {
		match self {
			Good(v) => Looping::Resume(v),
			Bad(v) => f(v),
		}
	}
}

/** Convert from and to Moral. Used for the macro map syntax.

This mirrors the `std::ops::Try` trait.

It is used for the `=>` mapping syntax of macros, to differentiate the value we want to keep from
the value we want to map through the function.
*/
pub trait Judge :Sized {
	/// This is considered Good
	type Positive;
	/// This is considered Bad
	type Negative;
	
	/// Convert to Moral
	fn into_moral (self) -> Moral<Self::Positive, Self::Negative>;
	
	/** Wraps a good value into itself
	
	For example `Result::Ok(v)` and `Judge::from_good(v)` are equivalent. Useful for converting types.
	*/
	fn from_good (v :Self::Positive) -> Self;
	
	/** Wraps a bad value into itself
	
	For example `Result::Err(e)` and `Judge::from_bad(e)` are equivalent. Useful for converting types.
	*/
	fn from_bad (v :Self::Negative) -> Self;

	/* Supplied methods */

	/** Convert to result */
	fn result (self) -> Result<Self::Positive, Self::Negative> {
		self.into_moral().into_result()
	}

	/** Convert to Either */
	#[cfg(feature = "combinators")]
	fn side (self) -> Either<Self::Negative, Self::Positive> {
		self.into_moral().into_either()
	}
}

/** Turns a `ValRet` into a value or an early return

It also coerces its argument to a ValRet (Return trait).

# Description

```text
let x = tear! { $e };
```

If $e is `Val(v)`, then v is assigned to x. Otherwise it is `Ret(r)`, in which case
the function immediately returns with a value of r.

This macro is useful when you have functions that return ValRet.

```text
let x = tear! { $e => $f }
```

Same as the previous form, but the return value `r` is first mapped through $f before returning.
In short, we return `$f(r)`.

# Examples

tear! with Val and Ret.

```rust
# #[macro_use] extern crate tear;
# use tear::prelude::*;
#
// "Ian" is assigned to name
let name = tear! { Val("Ian") };
# assert_eq![ name, "Ian" ];

# fn func () -> i32 {
// The function immediately returns -1
let _ = tear! { Ret(-1) };
# 0
# }
# let r = func();
# assert_eq![ r, -1 ];
```

tear! with a function returning ValRet

```rust
# #[macro_use] extern crate tear;
# use tear::prelude::*;
fn get_name () -> ValRet<String, i32> {
    Val("Chris".to_string())
    // or Ret(0)
}

fn func () -> i32 {
    // Will either assign the value to name, or return immediately
    let name = tear! { get_name() };
    name.len() as i32
}
# let x = func();
# assert_eq![ x, 5 ];
```

Mapping the return value

```rust
# #[macro_use] extern crate tear;
# use std::ffi::OsString;
fn string_id(s: OsString) -> String {
    let s: String = tear! { s.into_string() => |_| "No ID".to_string() };
    let id = s.len().to_string();
    id
}
# assert_eq![ string_id(OsString::from("ROOT")), "4" ];
```

# Naming

The name "tear" comes from the image of tearing apart the the usable value from the early return.
It also happens that "tear" looks like "ret(urn)" backwards.
*/
#[macro_export]
macro_rules! tear {
	// `tear! { $e }`
	( $e:expr ) => {
		match $crate::Return::into_valret($e) {
			$crate::ValRet::Val(v) => v,
			$crate::ValRet::Ret(r) => return r,
		}
	};
	// With a mapping function eg. `tear! { $e => |v| v }` or `tear! { $e => func }`
	( $e:expr => $f:expr ) => {
		{
			#[allow(clippy::redundant_closure_call)]
			match $crate::Judge::into_moral($e) {
				$crate::Moral::Good(v) => v,
				$crate::Moral::Bad(v) => return ($f(v)),
			}
		}
	}
}

/** Explicit `if` statement with early return 

# Description

```text
tear_if! { cond,  // <- NB: it's a comma
    do_things();
    v             // Return value
}
```

If cond is true, it executes the statements in its body and returns its value (v here).
It's basically an early return without the return statement at the end.

```text
tear_if! { let pat = expr,
    do_things();
    v
}
```

You can also use the pattern matching `if let`.

# Examples

Early return a value: recursively computing the length of a slice.
```rust
# #[macro_use] extern crate tear;
fn len (v: &[i32]) -> usize {
    // Base case
    tear_if! { v.is_empty(), 0 }
    
    // Recursion
    1 + len(&v[1..])
}
# assert_eq![ len(&[1, 2, 3]), 3 ];
```

Handle simple cases: printing help in a command line utility
```rust
# #[macro_use] extern crate tear;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    tear_if! { args.contains(&String::from("--help")),
        println!("No help available.")
    }
    
    println!("Greetings, human!");
}
```

Use patterns like `if let`
```rust
# #[macro_use] extern crate tear;
fn add_five(x: Option<i32>) -> i32 {
    tear_if! { let None = x, 0 }
    
    x.unwrap() + 5
}

assert_eq![ add_five(Some(2)), 7 ];
assert_eq![ add_five(None), 0 ];
```
*/
#[macro_export]
macro_rules! tear_if {
	// Normal tear_if! { $cond, $block }
	( $c:expr $( , $($b:tt)* )? ) => {
		$crate::tear! {
			if $c {
				$crate::ValRet::Ret({ $($($b)*)? })
			} else {
				$crate::ValRet::Val(())
			}
		}
	};
	// Handle tear_if! { let … }
	( let $p:pat = $e:expr $( , $($b:tt)* )? ) => {
		$crate::tear! {
			if let $p = $e {
				$crate::ValRet::Ret({ $($($b)*)? })
			} else {
				$crate::ValRet::Val(())
			}
		}
	};
}

/** `try!`-like error-handling macro

`terror!` is like `tear!`, but stronger and more righteous.
It automatically converts the Bad value to the return type Bad value (Judge trait).

# Description

```text
let x = terror! { $e };
```

If $e is a good value, it is assigned to x. Otherwise, $e is `Bad(value)`, we return `from_bad(value)`.
This form is equivalent to the `?` operator.

```text
let x = terror! { $e => $f };
```

Same as the previous form, but the bad `value` is first mapped through $f before returning.
In short, we return `from_bad($f(value))`.

# Explanation using examples

The description is especially terse on purpose: it is really hard to explain what `terror!` does without using examples.

## Simple examples

### Ripping Good and Bad values

`even_number` is assigned 2 because `Good(2)` is Good.

```rust
# #[macro_use] extern crate tear;
# use tear::extra::*;
fn return_two() -> Result<i32, String> {
    let even_number: i32 = terror! { Good(2) };
    # assert_eq![ even_number, 2 ];
    # Ok(even_number)
}
```

`error_five` returns early with `Err("five".to_string())` because `Bad("five".to_string())` is Bad.

```rust
# #[macro_use] extern crate tear;
# use tear::extra::*;
fn error_five() -> Result<i32, String> {
    let another_number: i32 = terror! { Bad("five".to_string()) };
    # Ok(5)
}
# assert_eq![ error_five(), Err("five".to_string()) ];
```

### Handling errors

Forwarding errors: If `s.into_string` is `Ok(v)`, the `String` v is assigned to s. If it is `Err(e)` with e being an `OsString`, we return `Err(e)`.

```rust
# #[macro_use] extern crate tear;
# use std::ffi::OsString;
fn len(s: OsString) -> Result<usize, OsString> {
    //        ┌─────────────────┐         │
    //        │        Result<String, OsString>
    //        │         └───────────┐
    let s: String = terror! { s.into_string() };

    Ok(s.len())
}
# assert_eq![ len(OsString::from("aa")), Ok(2) ];
```

Using a mapping function: we converts the error to the return type error

```rust
# #[macro_use] extern crate tear;
# use std::string::FromUtf8Error;
fn to_string(b: Vec<u8>) -> Result<String, String> {
    let s = terror! { String::from_utf8(b) => |e: FromUtf8Error| e.utf8_error().to_string() };

    Ok(s)
}
# assert_eq![ to_string(b"Zach".to_vec()), Ok("Zach".to_string()) ];
```

## The first form: `terror! { $e }`

```rust
# #[macro_use] extern crate tear;
# use std::num::ParseIntError;
fn parse_number (s :String) -> Result<i64, ParseIntError> {
    // Early return on error
    let n: i32 = terror! { s.parse() };
    Ok(n as i64)
}
# assert_eq![ parse_number("2".to_string()), Ok(2) ];
```

In this example, `s.parse()` returns a `Result<i32, ParseIntError>`. The good value is `i32`,
and the bad value is `ParseIntError`.

If we parsed the string succesfully, `terror!` evaluates to the parsed `i32` and
it is assigned to `n`.

But if fails, the ParseIntError is returned *as an error*. This means that
our `Err::<i32, ParseIntError>` is converted to a `Err::<i64, ParseIntError>` and then returned.

This form of `terror!` is especially useful when you just want to forward the error from
a function call to the function return value. Exactly like the `?` operator.

## The second form: `terror! { $e => $f }`

```rust
# #[macro_use] extern crate tear;
# use std::num::ParseIntError;
# use std::io;
# #[derive(Debug)]
enum Error {
    Parse(ParseIntError),
    Io(io::Error),
}

# fn parse_number (s :String) -> Result<i64, ParseIntError> {
#     // Early return on error
#     let n: i32 = terror! { s.parse() };
#     Ok(n as i64)
# }
#
fn square (s: String) -> Result<String, Error> {
    // If parse_number fails, convert the ParseIntError into our Error type and return early
    let number: i64 = terror! { parse_number(s) => Error::Parse };
    
    // Square the number and convert it to string
    let squared = (number * number).to_string();
    Ok(squared)
}
# assert_eq![ square("1".to_string()).unwrap(), "1".to_string() ];
```

We now know that `parse_number` returns a `Result<i64, ParseIntError>`.

We would now like to wrap that `ParseIntError` error into our our custom `Error` error type.
To do so, we extract the `ParseIntError`, and wrap it into our custom error with `Error::Parse`.

That is the role of the function following the `=>` arrow: it converts the error type of
the left statement, into the function return error type.

# `terror!` vs. `?` when moving into closures

The only difference between `terror!` and `?` is that since `terror!` is a macro,
you can move variables into the closure without the borrow checker yelling at you.

In this example, we want to return an error built from `path` using the `?` operator.

```compile_fail
# use std::{fs::File, path::PathBuf};
# enum Error {
#     OpenF(PathBuf),
# }
# 
fn open_file(path: PathBuf) -> Result<(), Error> {
    let file = File::open(&path).map_err(|_| Error::OpenF(path))?;
    
    // Do stuff with path and file
    # drop(path); drop(file);
    # Ok(())
}
```

However, it  fails to compile with the message `` error[E0382]: use of moved value: `path` ``.
This is because the borrow checker can't tell that when the closure is called,
it immediately returns. It sees that `path` is moved into the closure, and refuses
to let you use it in the rest of the function.

But if works if we use `terror!`. That's because since it's a macro, it expands into
code that tells the compiler that we immediately return after calling the closure.

```
# #[macro_use] extern crate tear;
# use std::{fs::File, path::PathBuf};
# enum Error {
#     OpenF(PathBuf),
# }
# 
fn open_file(path: PathBuf) -> Result<(), Error> {
    let file = terror! { File::open(&path) => |_| Error::OpenF(path) };
    
    // Do stuff with path and file
    # drop(path); drop(file);
    # Ok(())
}
```

# Naming

The name terror comes from "return error" and "tear! error".
The mnemonic was "When you need to scream an error from the inside" because of how closures worked (see §`terror!` vs. `?` when moving into closures).
*/
#[macro_export]
macro_rules! terror {
	// `terror! { $e }`
	( $e:expr ) => {
		$crate::tear! { $crate::Judge::into_moral($e).ret_error() }
	};
	// With a mapping function eg. `terror! { $e => |v| v }` or `terror! { $e => func }`
	( $e:expr => $f:expr ) => {
		{
			#[allow(clippy::redundant_closure_call)]
			match $crate::Judge::into_moral($e) {
				$crate::Moral::Good(v) => v,
				$crate::Moral::Bad(v) => return ::tear::Judge::from_bad(From::from($f(v))),
			}
		}
	}
}

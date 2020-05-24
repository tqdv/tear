/*! Typed early returns and syntax sugar macros for try!-like error handling

*Works with Rust v1.34+*

# Description

This crate exports the `tear!` and `terror!` macros.

`tear!` is used with `ValRet` for typed early returns. `terror!` is syntax-sugar for `try!` or the `?` operator.

# Usage

```rust
// Add this in your crate entrypoint (main.rs or lib.rs)
#[macro_use] extern crate tear;

// Import symbols for this example, generally not needed
use tear::prelude::*;
# 
# fn can_error() -> Result<i32, i32> {
#     Err(-5)
# }
# fn rescue_error(e: i32) -> i32 { e }

# fn f() -> Result<(), i32> {
// Error handling. Turn this…
let x = can_error().map_err(rescue_error)?;
// …into this
let x = terror! { can_error() => rescue_error };
# Ok(())
# }

// Early return
fn divide(a: i32, b: i32) -> Option<f32> {
    tear_if! { b == 0, None };
    
    let quotient = (a as f32) / (b as f32);
    Some(quotient)
}

// This function tells the calling function to return early
fn return_from_function() -> ValRet<String, i32> { Ret(-1) }

// Action at a distance
fn status_code() -> i32 {
    tear! { return_from_function() }; // returns
    0
}
# assert_eq![ status_code(), -1 ];
```

See the documentation for `tear!` and `terror!` for more examples.

# Feature flags

The "experimental" feature enables the experimental `Try` trait for `ValRet` and `Moral`.

# Rationale

I wanted to make early returns more explicit.

```text
if $cond {
    $statements;
    return $ret;
}
```

Normally, you can't tell from the outside if a code block will return early or not.
To bring the return statement out of the block requires a way to signal that we want to return early and something to catch that signal.
`ValRet` represents the signal and `tear!` returns early if needed.

Having a typed early return allows you to have functions that can force the caller function to return early.
Action at a distance inspired by how [Slips](https://docs.raku.org/type/Slip) work in Raku.

After reading up on how the `?` operator works, I thought of leveraging this typed early return for an explicit error handling syntax.
I wanted to annotate each potential failure point with a symbol and associate that symbol to an error handler.
Something like this:

```text
let path = find_config_file().mark(A)
let mut file = get_file_bufwriter(&path).mark(B)

// Error handlers
[A]: .ok_or(Error::FindPathF)?;
[B]: .map_err(Error::GetFileF)?;
```

Turns out this is already possible, but noisy, so the `terror!` macro makes a bit more explicit:
```ignore
let path = find_config_file().ok_or(Error::FindPathF)?;
let path = terror! { find_config_file() => Error::FindPathF };
```

# See also

- [Error Handling in Rust §The real `try!` macro / `?` operator](https://blog.burntsushi.net/rust-error-handling/#the-real-try-macro-operator)
- [guard](https://docs.rs/crate/guard), a crate implementing "guard" expressions

*/

// Optional features
#![cfg_attr(feature = "experimental", feature(try_trait))]

// Documentation lints
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

pub mod prelude;
pub mod extra;
mod trait_impl; // Move the trait implementions as they are quite noisy
pub mod twist_impl; // Currently only for `twist!`
pub mod util; // Macros that aren't the focus of the crate, but are useful. To reduce file size.

// Shorthands used in macros
use twist_impl as tw;
pub use tw::BreakValError;
pub use tw::{BREAKVAL_IN_NOT_LOOP, BREAK_WITHOUT_VAL, BAD_BREAKVAL_TYPE};
pub use tw::Looping;

/* CRATE DEV DOCS AND NOTES

# Notes
- Return and Judge are separate because I can't to keep Judge "pure". Also because you might want
  to implement only Return

# TODO
- Improve pitch with shorter examples and less rationale, more "this is cool"
- Check that the combinators are actually being used
- Tutorial for implementing Judge and Return, and what are their effects

# Useful resources
- <https://stackoverflow.com/questions/40302026/what-does-the-tt-metavariable-type-mean-in-rust-macros>
- <https://medium.com/@phoomparin/a-beginners-guide-to-rust-macros-5c75594498f1>
- <https://danielkeep.github.io/tlborm/book/mbe-min-captures-and-expansion-redux.html> even if outdated
- 'tear' related words: crease, cut, fold, split, strip. See <https://words.bighugelabs.com/tear>

# Outline

In this module, we define in order
- ValRet, its implementation, and its associated trait Return
- Moral, its implementation, and its associated trait Judge
- tear!, tear_if! and terror! macros
*/

use ValRet::*;
use Moral::*;

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
	/** Returns a new ValRet where we map the old Ret to the new Ret using the function supplied
	
	```rust
	# use tear::prelude::*;
	# let ok:    ValRet<&str, &str> = Val("ok");
	# let error: ValRet<&str, &str> = Ret("error");
	# 
	assert_eq![    ok.map_ret(|_| -1), Val("ok") ];
	assert_eq![ error.map_ret(|_| -1), Ret(-1)   ];
	```
	*/
	pub fn map_ret<R1> (self, f :impl FnOnce(R) -> R1) -> ValRet<V, R1> {
		match self {
			Val(v) => Val(v),
			Ret(r) => Ret(f(r)),
		}
	}
	
	/** Returns itself if it's a Val, otherwise calls the function to create
	a new ValRet based on the value of Ret.
	
	```rust
	# use tear::prelude::*;
	# let ok:    ValRet<&str, &str> = Val("ok");
	# let error: ValRet<&str, &str> = Ret("error");
	fn recover (e: &str) -> ValRet<&str, i32> {
		if e == "error" { Val("recover") } else { Ret(-1) }
	}
	
	assert_eq![    ok.or_else(recover), Val("ok")      ];
	assert_eq![ error.or_else(recover), Val("recover") ];
	```
	*/
	pub fn or_else<R1> (self, f :impl FnOnce(R) -> ValRet<V, R1>) -> ValRet<V, R1> {
		match self {
			Val(v) => Val(v),
			Ret(r) => f(r),
		}
	}
}

/// Convert into ValRet
pub trait Return where Self :Sized {
	/// The Val in ValRet
	type Value;
	/// The Ret in ValRet
	type Returned;
	
	/// Convert itself to a ValRet
	fn valret(self) -> ValRet<Self::Value, Self::Returned>;
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
	/// Convert to ValRet
	pub fn into_valret (self) -> ValRet<Y, N> {
		match self {
			Good(v) => Val(v),
			Bad(v) => Ret(v),
		}
	}
	
	/// Convert to Result
	pub fn into_result (self) -> Result<Y, N> {
		match self {
			Good(v) => Ok(v),
			Bad(v) => Err(v),
		}
	}
	
	/// Convert to Option. Or get the good variant as an Option.
	pub fn good (self) -> Option<Y> {
		match self {
			Good(v) => Some(v),
			Bad(_) => None,
		}
	}
	
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
pub trait Judge {
	/// This is considered Good
	type Positive;
	/// This is considered Bad
	type Negative;
	
	/// Convert to Moral
	fn into_moral(self) -> Moral<Self::Positive, Self::Negative>;
	
	/** Wraps a good value into itself
	
	For example `Result::Ok(v)` and `Judge::from_good(v)` are equivalent. Useful for converting types.
	*/
	fn from_good (v :Self::Positive) -> Self;
	
	/** Wraps a bad value into itself
	
	For example `Result::Err(e)` and `Judge::from_bad(e)` are equivalent. Useful for converting types.
	*/
	fn from_bad (v :Self::Negative) -> Self;
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
		match $crate::Return::valret($e) {
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

#[macro_export]
/** `try!`-like error-handling macro

`terror!` is like `tear!`, but stronger and more righteous.
It automatically converts the Bad value to the return type Bad value (Judge trait).

# Description

```text
let x = terror! { $e };
```

If $e is a good value, it is assigned to x. Otherwise, $e is `Bad(value)`, we return `from_bad(value)`.

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

# Comparison with `?` (try operator)

terror! behaves like the `?` operator, with the difference that it doesn't automatically
convert the actual error type.

This means that these two statements are generally equivalent:

```rust
# #[macro_use] extern crate tear;
# use std::{fs::File, path::PathBuf, io};
# 
# fn func (path: PathBuf) -> io::Result<()> {
File::open(&path)?;
terror! { File::open(&path) };
# Ok(())
# }
```

Except when we expect automatic conversion, for example when using types like
`Result<T, Box<dyn Error+Send+Sync>>`. See [Error Handling in Rust §The From trait][burntsushi-from]
and [anyhow](https://docs.rs/anyhow/), an error handling crate.
In those cases, you would need to explicitly specify the conversion:

[burntsushi-from]: https://blog.burntsushi.net/rust-error-handling/#the-from-trait

```rust
# #[macro_use] extern crate tear;
# use std::{fs::File, path::PathBuf, io};
use std::convert::From;
#
# fn func (path: PathBuf) -> io::Result<()> {
terror! { File::open(&path) => From::from };
# Ok(())
# }
```

## `terror!` vs. `?` when moving into closures

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
				$crate::Moral::Bad(v) => return ::tear::Judge::from_bad($f(v)),
			}
		}
	}
}

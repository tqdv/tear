/*! Utilitary macros that take too much space in the main file

Since they're macros, they're accessible from the crate root:
- `last!`, `next!`, `resume!` dirty macros
- `anybox!`
- (dev) `__unit!` and `__bool!`
- (not exported) `maybe_match!`
*/
use crate::Maru;

/** Dirty shortcut for creating a `Looping::Break`

# Description

If called with no arguments, it breaks the current loop.

If called with the label index, it breaks the corresponding loop (see `twist!`).

Used for writing short `twist!` statements that break from an enclosing loop. See examples.

Note that this macro will fail to compile if `twist!` can break with a value or when
using `twist -label`.

# Examples

```
use tear::{twist, last};

loop {
    twist! { last!() }
    panic!("We should break before this")
}

'a: loop {
    loop {
        twist! { -label 'a | last!(0) }
       panic!("We should break from the outer loop")
    }
}
```

# Naming

It is named after the equivalent of break in Perl. `break` is a keyword so we can't name
the macro `break!` unless we use `r#break!`.

# See also
- `last_if!`
*/
#[macro_export] macro_rules! last {
	() => { $crate::Looping::Break::<_, $crate::BreakValError> { label: None } };
	( $id:expr ) => { $crate::Looping::Break::<_, $crate::BreakValError> { label: Some($id) } };
}

/** Dirty shortcut for creating a `Looping::Continue`

# Description

If called with no arguments, it skips the current loop.

If called with the label index, it skips the corresponding loop (see `twist!`).

Used for writing short `twist!` statements that continue an enclosing loop. See examples.

Note that this macro will fail to compile if `twist!` can break with a value or when
using `twist -label`.

# Examples

```
use tear::{twist, next};

let mut i = 0;
loop {
    i += 1;
    
    if i < 5 {
        twist! { next!() }
    }
    break;
}
# assert_eq![ i, 5 ];

let mut i = 0;
'a: loop {
    i += 1;
    loop {
        if i < 8 {
            twist! { -label 'a | next!(0) }
        }
        break 'a;
    }
}
# assert_eq![ i, 8 ];
```

# Naming

It is named after the equivalent of continue in Perl. `continue` is a keyword so we can't name
the macro `continue!` unless we use `r#continue!`.

# See also
- `next_if!`
*/
#[macro_export] macro_rules! next {
	() => { $crate::Looping::Continue::<_, $crate::BreakValError> { label: None } };
	( $id:expr ) => { $crate::Looping::Continue::<_, $crate::BreakValError> { label: Some($id) } };
}

/** Dirty shortcut for creating a `Looping::Resume`

# Description

The only argument is the value to wrap in `Looping::Resume`.

Used for writing short `twist!` statements that evaluate to a value. See examples.

Note that this macro will fail to compile if `twist!` can break with a value or when
using `twist -label`.

# Examples

```
use tear::{twist, resume};

let mut i = 0;
loop {
    i = twist! { resume!(7) };
    break;
}
# assert_eq![ i, 7 ];

let mut i = 0;
'a: loop {
    loop {
        i = twist! { -label 'a | resume!(2) };
        break 'a;
    }
}
# assert_eq![ i, 2 ];
```
*/
#[macro_export] macro_rules! resume {
	( $($value:tt)* ) => { $crate::Looping::Resume::<_, $crate::BreakValError> ($($value)*) }
}

/** Turn a value into a `Box<dyn Any>`

# Description

Give it a value or an expression and it will turn it into a `Box<dyn Any>` value.

Used for breaking multiple loops with different values types with `twist!`.

# Examples

Just wrapping the value and getting it back.

```
use tear::anybox;

let boxed = anybox!(3);
let x = match boxed.downcast::<i32>() {
    Ok(v) => *v,
    Err(_) => panic!("Failed to get the integer back."),
};

assert_eq![ x, 3 ];
```

Using it as the breakval with `twist!`.

```
use tear::{twist, anybox};
use tear::Looping;

let e = Looping::BreakVal { label: Some(0), value: anybox!("a".to_string()) };

let x = 'a: loop {
    let _ = 'b: loop {
        twist! { -box -val i32, -label 'a: String | e }
        break 0;
    };
    break "b".to_string();
};
assert_eq![ x, "a".to_string() ];
```
*/
#[macro_export]
macro_rules! anybox {
	( $e:expr ) => {
		{
			let v = $e;
			let b = Box::new(v);
			let x = b as Box<dyn core::any::Any>;
			x
		}
	}
}

/** (dev) Always expands to `()`

Used for conditional expansion in macros as so.

```text
$( __unit!($variable); $code )?
```
*/
#[macro_export] macro_rules! __unit { ( $($whatever:tt)* ) => { () } }

/** (dev) Always expands to `false`

Used for conditional expansion of match arms in macros.
`__bool!` expands to false so that the arm is never executed.

```text
match $something {
    $(
        _ if __bool!($variable) => unreachable!(),
        $match-arm,
    )?
```
*/
#[macro_export] macro_rules! __bool { ( $($whatever:tt)* ) => { false } }

/** Executes match arm, or returns None

Helper for writing enum accessors where you either match the correct pattern, or return None.

The match arm expression is automatically wrapped into `Some`, so you don't need to.

# Example

```
let x: Option<i32> = maybe_match! { "a", "a" => 3 };
assert_eq![ x, Some(3) ];
```
*/
macro_rules! maybe_match {
	( $i:expr, $p:pat => $e:expr ) => {
		match $i {
			$p => Some($e),
			_ => None,
		}
	}
}

/** Always returns `Maru`

This function is used with `terror!` to return None, where you would use `.ok()?.unwrap()` instead.

```
# use tear::prelude::*;
fn f () -> Option<i32> {
	terror! { None => tear::gut }
	Some(1)
}
```
*/
pub fn gut<T> (_ :T) -> Maru { Maru }

/*! `twist!` implementation

We define some macros in this module, but since they're macros, they're accessible from the crate root:
- `__impl_twist`
- `twist!`

We also reexport all the types in this module for convenience.
*/

/** Error message when trying to break with a value in a non-`loop` loop */
pub const BREAKVAL_IN_NOT_LOOP :&str = "\
	error[E0571]: `break` with value is invalid in a `for` or `while` loop. \
	Use Break instead of BreakVal in `twist!` expression \
	or use `twist!` with the `-val` flag.";

/** Error message when trying to break without a value in a `twist -val` statement */
pub const BREAK_WITHOUT_VAL :&str = "\
	error[E0308]: mismatched types. \
	Breaking without a value when using `twist -val`. \
	Use BreakVal instead of Break, or use `twist!` without `-val`";

/** Error message when trying to break with the wrong type in a `twist -val` statement */
pub const BAD_BREAKVAL_TYPE :&str = "\
	error[E0308]: mismatched types. \
	Looping::BreakVal has a value type different from the loop it's breaking from. \
	Check you're breaking from the right loop, or use Break instead of BreakVal.";

/** (dev) Type to provide a nicer error message when trying to breakval from a non-`loop` loop

This type is not meant to be constructed, except by the `resume!`, `next!` and `last!` macros,
as well as the single breakpoint non-`val` forms of `twist!`.

This is because its only purpose is to give a useful error message when it's a sinple case.
In other cases, the compiler will emit a generic type mismatch or "cannot break with value" error.

See `rustc --explain E0571` for what the compiler is warning against.
*/
#[allow(non_camel_case_types)]
pub struct Error0571__Tried_to_break_with_value_using_twist_without_val_flag__Use_Break_instead_of_BreakVal_or_add_the_dash_val_flag_to_twist();

/** (dev) Short name for `Error0571__Tr...twist`

Used in `twist!`. This is to play nice with `rustfmt` or `cargo expand`, so that it doesn't just give up formatting
because the type name is too long.
*/
pub type BreakValError = Error0571__Tried_to_break_with_value_using_twist_without_val_flag__Use_Break_instead_of_BreakVal_or_add_the_dash_val_flag_to_twist;

/** Different loop control signals that `twist!` understands

We map `break`, `break $value` and `continue` to types.
*/
pub enum Looping<T, B> {
	/// Resume loop execution with value of type T
	Resume(T),
	/// Break a loop selected by `label`
	Break {
		/// The index of the label of the loop to break from. `None` means innermost loop
		label: Option<usize>
	},
	/// Break a loop selected by `label` with a value of `value`
	BreakVal {
		/// The index of the label of the loop to break from. `None` means innermost loop
		label: Option<usize>,
		/// The value to break with
		value: B
	},
	/// Skip to the next iteration of the loop selected by `label`
	Continue {
		/// The index of the label of the loop to continue from. `None` means innermost loop
		label: Option<usize>
	}
}

/** (dev) Macro required by `twist!`

Mostly contains step by step (@prefix) parsing for all the entrypoints in `twist!`. When it's done,
it calls `twist!` with the final processed values.

# Input and Output

The syntax for calling `@label-parse` is the following:
```text
(("pass") -> ("break") () ()) [$($tokens)*] ->
  │          │         │  │      └ The tokens that make up the label list and the expression
  │          ├─────────┴──┘        eg. `'a 'b | 1 + 1`
  │          └ Only one the three flags should be filled. In order:
  │            - "break" if the innermost loop can be broken normally
  │            - the type of the innermost loop break value if we break with a value
  │            - the type of the boxed innermost loop break value, if we break
  │              with Box<dyn Any>
  └ "unbox" if we unbox the breakvals, otherwise "pass"
```

We use "flags" to simulate booleans with empty parenthese or non-empty parentheses with
token trees inside. Because we can only conditionally do something when "it's full",
we need to have a slot for each case.

We call `twist! @boxed` with the following syntax:

```text
($($flag)*) ($($bk)*) [ ($($bv)*) () ] $e
    │           │       │         │    └ The expression to match on
    │           │       ├─────────┘
    │           │       └ Only one of these two slots should be filled.
    │           │         The left one is filled if we breakval normally
    │           │         The right one is filled if we unbox the value before breakval'ing
    │           └ The normal breaks
    └ The same three flags from the input
```

See inline documentation for brief explanations of what each `@step` does.
*/
#[macro_export]
macro_rules! __impl_twist {
	// Separate the labels from the expression by getting everything before `|`
	// ≪ (<$flag>*) [ $input ] -> ≫
	// → ≪ (<$flag>*) [ <$expr-token>* ] -> <$label-token>* ≫
	( @label-parse ($($flag:tt)*) [ | $($rest:tt)* ] -> $($l:tt)* ) => {
		$crate::__impl_twist! { @label-expr ($($flag)*) [$($rest)*] -> $($l)* }
	};
	( @label-parse ($($flag:tt)*) [ $token:tt $($rest:tt)* ] -> $($l:tt)* ) => {
		$crate::__impl_twist! { @label-parse ($($flag)*) [$($rest)*] -> $($l)* $token }
	};
	// There is no `|`: There's only an expression
	( @label-parse ($($flag:tt)*) [ ] -> $($rest:tt)* ) => {
		compile_error!("Missing `|` separator after labels in `twist! -label` macro invocation. Add labels, or use `twist!` without `-label`.")
	};
	
	// Parse the expression, or fail
	// ≪ (<$flag>*) [ <$expr-token>* ] -> <$label-token>* ≫
	// → ≪ (<$flag>*) 0, [ <$label-token>* , ] -> [() ()] <$expr> ≫
	( @label-expr ($($flag:tt)*) [ $e:expr ] -> $($l:tt)* ) => {
		// We add an extra comma, so that every label ends with a comma, simplifies parsing
		$crate::__impl_twist! { @label-labels ($($flag)*) 0, [$($l)* ,] -> [() ()] $e }
	};
	// Bad expression
	( @label-expr ($($flag:tt)*) [ $($rest:tt)* ] $($whatever:tt)* ) => {
		compile_error!(concat!("This failed to parse as an expression: ", stringify!($($rest)*)))
	};
	
	// Parse labels (eg. `'a` or `'a: i32`) separated with commas and separate those that
	//   break with values and those that don't. Break = $bk and BreakVal = $bv
	// ≪ (<$flag>*) 0, [ <$label-token>* , ] -> [() ()] <$expr> ≫
	// → ≪ (<$flag>*) (<$bk>*) (<$bv>*) $expr ≫
	// Nothing left to parse
	( @label-labels ($($flag:tt)*) $count:expr, [] -> [($($bk:tt)*) ($($bv:tt)*)] $e:expr ) => {
		$crate::__impl_twist! { @label-box ($($flag)*) ($($bk)*) ($($bv)*) $e }
	};
	// Parse `'a: i32,`
	( @label-labels ($($flag:tt)*) $count:expr, [ $label:lifetime : $type:ty , $($rest:tt)* ] -> [($($bk:tt)*) ($($bv:tt)*)] $e:expr ) => {
		$crate::__impl_twist! { @label-labels ($($flag)*) $count + 1, [$($rest)*] -> [($($bk)*) ( $($bv)* ($count, $label, $type) )] $e }
	};
	// Parse `'a,`
	( @label-labels ($($flag:tt)*) $count:expr, [ $label:lifetime , $($rest:tt)* ] -> [($($bk:tt)*) ($($bv:tt)*)] $e:expr ) => {
		$crate::__impl_twist! { @label-labels ($($flag)*) $count + 1, [$($rest)*] -> [( $($bk)* ($count, $label) ) ($($bv)*)] $e }
	};
	// Bad label syntax
	( @label-labels ($($flag:tt)*) $count:expr, [ $($rest:tt)* ] -> [($($bk:tt)*) ($($bv:tt)*)] $e:expr ) => {
		compile_error!(concat!("Bad label syntax: ", stringify!($($rest)*)))
	};

	// Apply the box flag onto $bv so we can differentiate when consuming it
	// ≪ ( ($box) -> <$flag>*) (<$bk>*) (<$bv>*) $expr ≫
	// → ≪ (<$flag>*)  (<$bk>*) [ (<$bv>*) (<$bx>*) ] $expr ≫
	( @label-box ( ("unbox") -> $($flag:tt)* ) ($($bk:tt)*) ($($bv:tt)*) $e:expr ) => {
		twist! { @boxed ($($flag)*) ($($bk)*) [ () ($($bv)*) ] $e }
	};
	( @label-box ( ("pass") -> $($flag:tt)* ) ($($bk:tt)*) ($($bv:tt)*) $e:expr ) => {
		twist! { @boxed ($($flag)*) ($($bk)*) [ ($($bv)*) () ] $e }
	};
}

/** Breaks loops (or not) based on the `Looping` variant

# Usage

With `$e` and expression that evalutes to a `Looping` value. The general syntax is:

```text
twist! { [-val] $e }
twist! { [-val] -with $label | $e }
twist! { [-box] [-val $type,] -label <$label [: $type]>,* | $e }
```

## Use cases

If you're breaking from the current loop, use one of the following

```ignore
twist! { $e }      // Usual case
twist! { -val $e } // If you're breaking with a value (`loop` loop)
```

If you're breaking a labeled loop:

```ignore
twist! { -with 'label | $e }      // Normal break from the labeled loop
twist! { -val -with 'label | $e } // If you're breaking the labeled loop with a value
```

If you're breaking from multiple loops:

```ignore
twist! { -label 'a, 'b | $e } // Normal break for loops 'a, 'b and innermost
```

If you're breaking from multiple loops and can break with the **same value type**:

```ignore
twist! { -label 'a: i32, 'b, 'c: i32 | $e } // If the innermost loop is a normal break
twist! { -val i32, -label 'a:i32, 'b | $e } // If the innermost loop breaks with a value
                                            // (the type is mandatory)
```

If you're breaking from multiple loops with multiple types by using `Box<dyn Any>` as the value type:

```ignore
// If the innermost loop is a normal break
twist! { -box -label 'a: i32, 'b: String | $e }
// If the innermost loop breaks with a value
twist! { -box -val i32, -label 'a, 'b: String | $e }
```

# Description

`twist!` takes an expression of `Looping` type, and `break`s, `continue`s or resume the loop
execution based on the `Looping` variant. There are various flags that control which loop are
concerned, and what value type to break with (for `loop` loops).

`-box`

## Errors

### Compile failure
A common error (at least for me), is to forget that you **need** to specify if the innermost loop
breaks with a value or not, even if you don't do anything with it.

### Panics
This **will** panic if you use the wrong loop label index; if you try to break a
non-`loop` loop with a value; or if you try to break a `loop`-loop that expects a value,
without a value

# Examples

*All example bring `twist` and `Looping` into scope.*

An infinite loop that immediately gets broken.

```
# use tear::{twist, Looping};
loop {
    twist! { Looping::Break { label: None } }
}
```

Breaking a loop with a value with the `-val` switch.

```
# use tear::{twist, Looping};
let x = loop {
    twist! { -val Looping::BreakVal { label: None, value: 8 } }
};
assert_eq![ x, 8 ];
```

Breaking a labeled loop. `-with` sets the loop on which we act.

```
# use tear::{twist, Looping};
'a: loop {
    loop {
        twist! { -with 'a | Looping::Break { label: None } }
    }
}
```

Breaking multiple loop with different types with `-box`. Labels are counted from 0, so `Some(0)`
refers to `'a: String`. The second loop also breaks with a value type of `i32`, specified in
`twist!` as `-val i32,`.

```
# use tear::{twist, Looping};
use tear::anybox;

let x = 'a: loop {
    let _ = loop {
        twist! { -box -val i32, -label 'a: String |
            Looping::BreakVal { label: Some(0), value: anybox!("a".to_string()) }
        }
    };
};
assert_eq![ x, "a".to_string() ];
```

See more barebones examples for breaking multiple loops in `test/label.rs`.

# See also

- The `last!`, `next!` and `resume!` utility macros.
- The `anybox!` macro when the expression is of type `Box<dyn Any>` and we unbox it

# Developer docs

See inline comments for more information.

Most patterns of the macro are the entrypoints for 2 "templated" implementations for
"single loop break" (`@single`) and "labeled loop break" (`@boxed`).

## `@boxed`: Breaking from multiple loops

The non-`box` versions can only break with a single value type because you can only choose one type
to be the `BreakVal` value type. To circumvent this with the `box` versions, we expect
a `Box<dyn Any>` value that we downcast to the right type.

## `@single`: Breaking from a single loop

When breaking from a single loop without a value, we set the BreakVal type of `Looping`
to `BreakValError`. If the user tries to break with a value, the program will fail to compile
because the types are different. It should then display the full name of `BreakValError`
(which is an error message) in the error message.
*/
#[macro_export]
macro_rules! twist {
	/* When we break from multiple loops */
	
	// Handle a Looping object that can break with labels, and break with a value
	( -label $($tokens:tt)* ) => {
		$crate::__impl_twist! { @label-parse (("pass") -> ("break") () ()) [$($tokens)*] -> }
	};
	// The innermost loop breaks with a value
	( -val $type:ty, -label $($tokens:tt)* ) => {
		$crate::__impl_twist! { @label-parse (("pass") -> () ($type) ()) [$($tokens)*] -> }
	};
	// Same thing, but we unbox the breakval
	( -box -label $($tokens:tt)* ) => {
		$crate::__impl_twist! { @label-parse (("unbox") -> ("break") () ()) [$($tokens)*] -> }
	};
	( -box -val $type:ty, -label $($tokens:tt)* ) => {
		$crate::__impl_twist! { @label-parse (("unbox") -> () () ($type)) [$($tokens)*] -> }
	};

	// Generic implementation for when we handle loop labels
	// We handle Break and BreakVal and boxed Breakval for the innermost loop (3 cases)
	// Syntax: ($($flags:tt)*) ($($bk:tt)*) [( ) ( )] $e:expr
	//             |               |          |   ^If we unbox, fill with $( ($count, $label, $type) )*
	//             |               |          ^If we don't unbox, fill with $( ($count, $label, $type) )*
	//             |               ^Breaks of ($count, $label)
	//             ^"Flags": ($bk) ($bv) ($bx). Whether the innermost loop breaks, breakvals or breakval and unboxes
	//              Specify the usable type for $bv and $bx
	( @boxed ( ($($bk:tt)?) ($($bv:ty)?) ($($bx:ty)?) ) // Flags
		( $( ($c:expr, $l:lifetime) )* ) // Breaks
		[ ($( ($count:expr,  $label:lifetime,  $type:ty)  )*) // Normal breakvals
		  ($( ($bcount:expr, $blabel:lifetime, $btype:ty) )*) ] // Boxed breakvals
		$e:expr
	) => {
		match $e {
			$crate::Looping::Resume(v) => v,
			$( $crate::Looping::Break { label: None } => { $crate::__unit!($bk); break; }, )?
			$( $crate::Looping::Break { label: None } => { $crate::__unit!($bv); panic!($crate::BREAK_WITHOUT_VAL) }, )?
			$( $crate::Looping::Break { label: None } => { $crate::__unit!($bx); panic!($crate::BREAK_WITHOUT_VAL) }, )?
			$crate::Looping::Break { label: Some(l) } => {
				match l {
					$( x if x == $c => { break $l; }, )*
					_ => panic!("Invalid label index in Looping::Break object."),
				};
			},
			$crate::Looping::Continue { label: None } => continue,
			$crate::Looping::Continue { label: Some(l) } => {
				match l {
					$( x if x == $c => { continue $l; }, )*
					$( x if x == $count => { continue $label; }, )*
					$( x if x == $bcount => { continue $blabel; }, )*
					_ => panic!("Invalid label index in Looping::Continue object."),
				};
			},
			$( $crate::Looping::BreakVal { label: None, .. } => { $crate::__unit!($bk); panic!($crate::BREAKVAL_IN_NOT_LOOP); }, )?
			$( $crate::Looping::BreakVal { label: None, value: v } => { $crate::__unit!($bv); break v; }, )?
			$( $crate::Looping::BreakVal { label: None, value: v } => { // Unbox version
				match v.downcast::<$bx>() {
					Ok(v) => { break *v; },
					_ => panic!(format!("At label None with type {}: {}", stringify!($bx), $crate::BAD_BREAKVAL_TYPE)),
				};
			}, )?
			// Add explicit breakval type when it can't be infered by the labeled breaksvals
			// (because there aren't any) but we do breakval the innermost loop
			$crate::Looping::BreakVal $(::<_, $bv> )? { label: Some(l), value: v } => {
				match l {
					$( x if x == $count => { break $label v; }, )*
					$( x if x == $bcount => { // Unbox version
						match v.downcast::<$btype>() {
							Ok(v) => { break $blabel *v; }, // We got a ref so dereference it
							_ => panic!(format!("At label {} with type {}: {}", stringify!($blabel), stringify!($btype), $crate::BAD_BREAKVAL_TYPE)),
						}
					}, )*
					_ => panic!("Invalid label index in Looping::BreakVal object."),
				};
			},
		};
	};
	
	/* When we just break from a single loop */
	
	// Handle a Looping object
	( $e:expr ) => {
		twist! { @single [("break") ()] [] ($e) }
	};
	// Handle a Looping object that breaks a specific label
	( -with $l:lifetime | $e:expr ) => {
		twist! { @single [("break") ($l)] [] ($e) }
	};
	// Handle a Looping object that can break with a value
	( -val $e:expr ) => {
		twist! { @single [] [("breakval") ()] ($e) }
	};
	// Handle a Looping object that can break with a value for a specific label
	( -val -with $l:lifetime | $e:expr ) => {
		twist! { @single [] [("breakval") ($l)] ($e) }
	};
	
	// Generic implementation for when we break from a single loop
	// Syntax is [ ] [ ] ($e)
	//            |   ^If breaking with value, fill with ("breakval") ( $label? )
	//            ^If breaking without value, fill with ("break") ( $label? )
	( @single
		[$( ($breaker:tt) ($($label:lifetime)?) )?] // Break
		[$( ($breakval:tt) ($($vlabel:lifetime)?) )?] // BreakVal
		($e:expr)
	) => {
		match $e {
			$( _ if $crate::__bool!($breaker)  => unreachable!(), $crate::Looping::Resume::<_, $crate::BreakValError> (v) => v, )?
			$( _ if $crate::__bool!($breakval) => unreachable!(), $crate::Looping::Resume(v) => v, )?
			$( _ if $crate::__bool!($breaker)  => unreachable!(), $crate::Looping::Break { .. } => break $($label)?, )?
			$( _ if $crate::__bool!($breakval) => unreachable!(), $crate::Looping::Break { .. } => panic!($crate::BREAK_WITHOUT_VAL), )?
			$crate::Looping::Continue { .. } => continue $($($label)?)? $($($vlabel)?)?,
			$( _ if ::tear::__bool!($breaker)  => unreachable!(), $crate::Looping::BreakVal { .. } => panic!($crate::BREAKVAL_IN_NOT_LOOP), )?
			$( _ if $crate::__bool!($breakval) => unreachable!(), $crate::Looping::BreakVal { value: v, .. } => break $($vlabel)? v, )?
		}
	};
}
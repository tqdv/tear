/*! Overview of things in this crate

# Importing the symbols

```
use tear::prelude::*;
use tear::extra::*;
```

The `prelude` and `extra` modules are meant to be used for bulk importing.
You generally want to use `prelude` only and import extra symbols one-by-one. Some symbols in
`extra` might conflict with yours.

All symbols are accessible directly from the crate root as we reexport them all.

# Early returns

We represent an early return with [`ValRet`] and process it with [`tear!`]. The macro accepts any
type that knows how to convert to a `ValRet` using the [`Return`] trait.

We use `tear!` in [`tear_if!`] to implement early returns as a syntax.

# Mapping syntax

The mapping syntax is one of the following:
```text
tear! { ... => ... }
terror! { ... => ... }
twist! { ... => ... }
```

When using the mapping syntax, we need to separate the wanted value, from the unwanted value
that we pass to the mapping function.
This is why arguments must implement the `Judge` trait that knows how to convert it to either
`Moral::Good` or `Moral::Bad`.

# Error handling

`terror!` is the error-handling macro. It depends on [`Judge`] to decide if the value is usable
or not.

A short way of discarding the error value in a function returning `Option<T>`, is to use the [`gut`]
function:

```
# use tear::prelude::*;
# fn get_value () -> Option<i32> { None }
fn f () -> Option<i32> {
    terror! { get_value() => tear::gut };
# Some(1)
# }
```

If you need to do some things before returning `None`, use a block, and return `tear::Maru` at the
end. [`Maru`] is the placeholder type used to represent the bad value of `Option<T>`, or the good
and bad values of `bool`.

# Loop control

The `twist!` macro has many forms (see its documentation), and it only processes `Looping` types.
They represent a control signal: either resume, break, continue or breakval the loop.

Breakval is the special case of a `loop`-loop that can return with a value:

```
let x: i32 = loop {
    break 3;
};
assert_eq![ x, 3 ];
```

In the complex case where you want to breakval from multiple loops with a different type, you can
use `Box<dyn Any>` to hide those type. We provide the [`anybox!`] macro to take the concrete type,
and wrap it into a `Box<dyn Any>` object. See [`twist!`] documentation for more information.

```
use tear::prelude::*;

let x: i32 = 'a: loop {
	let y: String = 'b: loop {
		let _ = twist! { -box -val String, -label 'a: i32 |
			Looping::BreakVal { label: Some(0), value: anybox!(3) }
		};
		if false { break "a".to_string() }
	};
};
assert_eq![ x, 3 ];
```

For simple cases where you only break from one loop (ie. when you don't use `-labels`), you can
use the [`last!`], [`next!`], and [`resume!`] as shortcuts for the right-hand side of `twist!`:

```
use tear::extra::*;
loop {
    let v = twist! { Some(2) => |_| resume!(5) };
    twist! { None => |_| last!() };
}
```

There's also [`next_if!`] and [`last_if!`] macros that continue or break the loop based on a condition
or a pattern match.

# Add functionality to your own types

If you want to enable the mapping syntax for your type.
```text
terror! { $your-type => ... }
```
or use one of the macros in a function returning your type
```text
fn f () -> $your-type {
	terror! { ... }
}
```

You only need to implement `Judge` trait for that type, because `Return` is automatically implemented
for you.

If using the "experimental" crate feature, then you only need to implement the `Try` trait. The
`Judge` and `Return` trait will be automatically implemented.

*/
#[allow(unused_imports)]
use super::*; // Brings symbols into scope for rustdoc links

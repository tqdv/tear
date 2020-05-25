# tear

Typed early returns and loop control + Syntax sugar for try!-like error handling

*Works with Rust v1.34+ (released on 11 April 2019)*

## Synopsis

Import the macros into your module:
```rust
use tear::prelude::*;
```

Explicit error-handling syntax with `terror!`:
```rust
let handled = terror! { can_error() => print_error };
let variant = terror! { can_io_error() => CustomError::Io };

// Equivalent using `?`:
let handled = can_error().map_err(print_error)?;
let variant = can_io_error.map_err(CustomError::Io)?;
```

Early loop continue with `twist!`:
```rust
for re in regexes_strings {
    // Skip iteration if the regex fails to compile
    let re = twist! { Regex::new(re) => |_| next!() }

    // Use regex...
```

Keyword-like early returns with `tear_if!`:
```rust
fn divide_i32 (num: i32, denom: i32) -> Option<f32> {
    // Return early if dividing by 0
    tear_if! { denom == 0, None };

    // Compute quotient...
```

Typed early returns with `tear!`:
```rust
// Tells the calling function to return early on failure
fn get_value_or_return() -> ValRet<String, i32> { Ret(-1) }

fn status_code() -> i32 {
    let v = tear! { get_value_or_return() };

    // Process value...
```

See [the documentation](https://docs.rs/tear) for more info.

## Rationale

**I wanted to make early returns more explicit.**

Normally, you need to read until the end of the
`if` body to know if it returns early or not. `tear_if` places that information at
the beginning of the block.

**I wanted typed early returns because it is useful for passing exitcodes up the callchain.**

Having a typed early return allows you to have functions that can force their caller
to return early. It's an action at a distance inspired by how
 [Slips](https://docs.raku.org/type/Slip) work in Raku.

**I wanted annotated failure points instead of too many combinators.**

The `?` operator works is essentially a conditional early-return.
To convert the errors you get to the right type, you need to use combinators.
I find it hard to discern that those combinators are meant for error handling.
  
Something like this:
```rust
let path = find_config_file().ok_or(Error::FindPathF)?
let mut file = get_file_buffer(&path).map_err(Error::GetFileF)?;
```

The `terror!` macro makes the error handling more explicit:
```rust
let path = terror! { find_config_file() => Error::FindPathF };
let mut file = terror! { get_file_buffer(&path) => Error::GetFileF };
```

**Loop control and early returns are similar**

I already implemented typed early return, so why not implement typed loop controls well ?
They're the same kind of useful.

## See also

- [Error Handling in Rust §The real `try!` macro / `?` operator][error-handling try]
- [guard](https://docs.rs/crate/guard), a crate implementing "guard" expressions,
  the counterpart to `tear_if!`

[error-handling try]: https://blog.burntsushi.net/rust-error-handling/#the-real-try-macro-operator

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE)
or [MIT license](LICENSE-MIT) at your option.

<small>Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above,
without any additional terms or conditions.</small>

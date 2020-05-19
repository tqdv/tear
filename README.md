# tear

> Typed early returns and syntax sugar macros for try!-like error handling

*Works with Rust v1.34+*

## Description

This crate exports the `tear!` and `rip!` macros.

`tear!` is used with `ValRet` for typed early returns. `rip!` is syntax-sugar for `try!` or the `?` operator.

## Usage

```rust
// Add this in your crate entrypoint (main.rs or lib.rs)
#[macro_use] extern crate tear;

// Import symbols for this example, generally not needed
use tear::prelude::*;

// Error handling. Turn this…
let x = can_error().map_err(rescue_error)?;
// …into this
let x = rip! { can_error() => rescue_error };

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
```

See the documentation for `tear!` and `rip!` for more examples.

## Rationale

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

```rust
let path = find_config_file().mark(A)
let mut file = get_file_bufwriter(&path).mark(B)

// Error handlers
[A]: .ok_or(Error::FindPathF)?;
[B]: .map_err(Error::GetFileF)?;
```

Turns out this is already possible, but noisy, so the `rip!` macro makes a bit more explicit:
```rust
let path = find_config_file().ok_or(Error::FindPathF)?;
let path = rip! { find_config_file() => Error::FindPathF };
```

## See also

- [Error Handling in Rust §The real `try!` macro / `?` operator](https://blog.burntsushi.net/rust-error-handling/#the-real-try-macro-operator)
- [guard](https://docs.rs/crate/guard), a crate implementing "guard" expressions

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
# Notes

## TODO

Nothing currently

## Thoughts

- `Moral` is not meant to be manipulated, nor is `ValRet`. If you need combinators,
  use `Judge`'s side or result methods.
- If there's a use-case where Judge -> Return blanket trait implementation poses a problem, I should
  replace it with a macro. Also, if auto traits get stabilized, we could let the user disable it.
- I should probably use proc\_macros instead of abusing macros for `__impl_twist!`, but I don't know how
  to write one, and docs aren't easily found.
- Convenience functions are named shortly and memorable. Trait functions are named boringly and at least
  two words.

## Useful resources
- <https://stackoverflow.com/questions/40302026/what-does-the-tt-metavariable-type-mean-in-rust-macros>
- <https://medium.com/@phoomparin/a-beginners-guide-to-rust-macros-5c75594498f1>
- <https://danielkeep.github.io/tlborm/book/mbe-min-captures-and-expansion-redux.html> even if outdated
- 'tear' related words: crease, cut, fold, split, strip. See <https://words.bighugelabs.com/tear>

## Useful commands

The usual ones:
```sh
cargo build
cargo test
cargo doc
```

The specific ones:
```sh
cargo +nightly build --features experimental
cargo expand --color=always | less
RUSTFLAGS="-Z macro-backtrace" cargo +nightly test
ack 'TODO|FIXME|IDEA|TMP|TEMP' src/
cargo +nightly rustdoc -- --document-private-items -W missing_doc_code_examples
RUSTFLAGS="-A clippy::tabs_in_doc_comments" cargo clippy
```

# Notes

- If there's a use-case where Judge -> Return blanket trait implementation poses a problem, I should
  replace it with a macro. Also, if auto traits get stabilized, we could let the user disable it.
- I should probably use proc_macros instead of abusing macros for `__impl_twist!`, but I don't know how
  to write one, and docs aren't easily found.

## TODO
- Improve pitch with shorter examples and less rationale, more "this is cool"
- Check if the combinators are actually being used
- Tutorial for implementing Judge and Return, and what are their effects

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
```
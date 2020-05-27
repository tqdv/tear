# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

Make `terror!` work for booleans and add the `next_if!` and `last_if!` macros.

### Migration
- If you were returning `()` in `terror!` in a function returning `Option<T>`, return `Maru` instead.
- _(experimental)_ `Try` → `Judge` implementations are now manual. Use the `impl_judge_from_try!` macro.

### Added
- Implemented `Judge` on `bool`.
- Added `Maru` that's similar to `()` and `NoneError`
- Added `next_if!` and `last_if!`

### Changed
- Use `Maru` instead of `()` in `gut` and `Judge` for `Option`

## [0.2.0] – 2020-05-26

Implemented typed loop control with `twist!`. Make `terror!` fully compatible with `?`.

### Breaking
- Renamed `Return` method `valret` to `into_valret`

### Added
- Implementation of `twist!` with `Looping` type and `anybox!`
- Dirty utility macros `last!`, `next!`, `resume!`
- `tear::extra` For importing everything in one fell swoop
- Integrate with the`Try` trait under the "experimental" feature
- enum accessors for `ValRet` and `Moral`
- Maintenance badge on crates.io
- `gut` for use with `terror!` in functions returning `Option<T>`
- Ensure basic no-std
- Add `combinators` feature flag to convert to `Either` anything that can convert to `Moral`

### Changed
- `tear_if!` lets you use anything in its body, instead of just expressions.
- Make `terror!` the preferred macro over `rip!`
- `terror!` and `fear!` can now take both argument forms
- `terror!` now uses the `From` trait, just like `?`
- Rewrite README

### Removed
- `or_else` and `map_ret` methods on `ValRet`. Use `result().or_else()` or `result().map_err()`
  instead

## [0.1.1] – 2020-05-19

This release was to test if I could overwrite
[the name squatting on docs.rs](https://docs.rs/crate/tear/0.1.1).
Turns out you can, but the squatter is still in the list of owners and authors.

### Added
- Licenses

## [0.1.0] – 2020-05-19

Initial release

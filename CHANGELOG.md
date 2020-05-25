# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

Implemented typed loop control with `twist!`. Become fully compatible with `?`.

### Added
- Implementation of `twist!` with `Looping` type and `anybox!`
- Dirty utility macros `last!`, `next!`, `resume!`
- `tear::extra` For importing everything in one fell swoop
- `Try` trait for `ValRet` and `Moral`, under the "experimental" feature
- enum accessors for `ValRet` and `Moral`
- Maintenance badge on crates.io

### Changed
- `tear_if!` lets you use anything in its body, instead of just expressions.
- Make `terror!` the preferred macro over `rip!`
- `terror!` and `fear!` can now take both argument forms
- `terror!` now uses the `From` trait, just like `?`

## [0.1.1] – 2020-05-19

This release was to test if I could overwrite
[the name squatting on docs.rs](https://docs.rs/crate/tear/0.1.1).
Turns out you can, but the squatter is still in the list of owners and authors.

### Added
- Licenses

## [0.1.0] – 2020-05-19

Initial release

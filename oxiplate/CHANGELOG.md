# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.8](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.1.6...oxiplate-v0.1.8) - 2025-01-06

Merged the macro back in from https://github.com/0b10011/oxiplate-derive/commit/7c965e47dc0714a2e10b9bd8d4f488c101224fff

### Added

- added support for rust keywords as identifiers and cleaned up code along the way
- improved error message for missing `if` expression
- shortened source range for errors that don't provide a span
- building the write format with the templates themselves to reduce the number of arguments needed
- calling `write_str()` instead of `write_fmt()` for a single static token
- combined sequential static text and whitespace into a single concat
- combined sequential static text, whitespace, and writs into a single write call

### Fixed

- improved span information for field/method access
- use correct module for `escape()`
- pass escaper by reference to match expectation in the main crate

### Other

- try using package-specific versions instead
- bring back separate changelog for the macro and fix versions
- move the derive macro back to the main repo
- rewrite readme to add badges and point to book/docs
- `cargo clippy` changes
- Fix newlines
- Move templates to their own directory and support nesting
- Fix hygiene
- Initial attempt to use `include_str!()` to fetch files
- Add message about code being experimental
- Add support for templates stored in separate files
- Add basic documentation
- remove some unused code
- prevent clippy test output from changing based on whether build is required
- check if spans work with clippy
- dependency updates
- fixed repository link
- changed expansion tests to fail when the expected output for a test is missing
- build the path to `oxiplate.toml` from the env instead to help with testing


## [0.1.6](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.1.5...oxiplate-v0.1.6) - 2025-01-01

### Added

- add an escaper for markdown

### Other

- extend `cargo dev` to build docs as well
- fix path in example
- add more tests and refactor a bit

## [0.1.5](https://github.com/0b10011/oxiplate/compare/v0.1.4...v0.1.5) - 2024-12-31

### Added

- add support for custom escapers and move macro to `oxiplate-derive` crate

### Fixed

- use correct name for escaper group in message

### Other

- use an enum for escapers to reduce boilerplate

## [0.1.4](https://github.com/0b10011/oxiplate/compare/v0.1.3...v0.1.4) - 2024-12-30

### Other

- add basic support for custom escapers
- add state to escaper and rename it to prep for external escapers
- add state and move local variables into it

## [0.1.3](https://github.com/0b10011/oxiplate/compare/v0.1.2...v0.1.3) - 2024-12-25

### Other

- add writs doc
- add "Getting started" doc

## [0.1.2](https://github.com/0b10011/oxiplate/compare/v0.1.1...v0.1.2) - 2024-12-23

### Other

- add start of book

## [0.1.1](https://github.com/0b10011/oxiplate/compare/v0.1.0...v0.1.1) - 2024-12-23

### Other

- add funding info
- add release-plz for release management

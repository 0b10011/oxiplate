# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.7](https://github.com/0b10011/oxiplate/compare/oxiplate-traits-v0.1.6...oxiplate-traits-v0.1.7) - 2026-01-10

### Other

- update to latest nightly

## [0.1.6](https://github.com/0b10011/oxiplate/compare/oxiplate-traits-v0.1.5...oxiplate-traits-v0.1.6) - 2025-12-19

### Other

- added test coverage badge

## [0.1.5](https://github.com/0b10011/oxiplate/compare/oxiplate-traits-v0.1.4...oxiplate-traits-v0.1.5) - 2025-11-28

### Other

- inlined `CowStr` related trait methods

## [0.1.4](https://github.com/0b10011/oxiplate/compare/oxiplate-traits-v0.1.3...oxiplate-traits-v0.1.4) - 2025-11-21

### Added

- macro for building `CowStrWrapper` for testing filters dealing with string-like values

## [0.1.3](https://github.com/0b10011/oxiplate/compare/oxiplate-traits-v0.1.2...oxiplate-traits-v0.1.3) - 2025-11-20

### Added

- added efficient conversion to `Cow<'a, str>` via `CowStr` trait for filters

## [0.1.2](https://github.com/0b10011/oxiplate/compare/oxiplate-traits-v0.1.1...oxiplate-traits-v0.1.2) - 2025-08-31

### Added

- moved `itoa` dependency (and fast escaping of integers) behind the `fast-escape-ints` feature flag (fixes #67)

## [0.1.1](https://github.com/0b10011/oxiplate/compare/oxiplate-traits-v0.1.0...oxiplate-traits-v0.1.1) - 2025-08-31

### Fixed

- fixed README path

## [0.1.0](https://github.com/0b10011/oxiplate/releases/tag/oxiplate-traits-v0.1.0) - 2025-08-31

### Added

- split traits into separate crate
- [**breaking**] add estimated length and rendering directly into a string

### Other

- improve highlighting of code block filenames
- fix links in released crate
- use correct error message
- add more information about the main focuses to the READMEs
- rewrite readme to add badges and point to book/docs
- `cargo clippy` changes
- Fix newlines
- Move templates to their own directory and support nesting
- Fix hygiene
- Initial attempt to use `include_str!()` to fetch files
- Add message about code being experimental
- Add support for templates stored in separate files
- Add basic documentation

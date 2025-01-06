# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.7](https://github.com/0b10011/oxiplate-derive/compare/v0.1.6...v0.1.7) - 2025-01-04

### Fixed

- improved span information for field/method access

### Other

- remove some unused code
- prevent clippy test output from changing based on whether build is required
- check if spans work with clippy

## [0.1.6](https://github.com/0b10011/oxiplate-derive/compare/v0.1.5...v0.1.6) - 2025-01-03

### Other

- dependency updates
- fixed repository link

## [0.1.5](https://github.com/0b10011/oxiplate-derive/compare/v0.1.4...v0.1.5) - 2025-01-03

### Added

- added support for rust keywords as identifiers and cleaned up code along the way
- improved error message for missing `if` expression
- shortened source range for errors that don't provide a span

### Other

- changed expansion tests to fail when the expected output for a test is missing

## [0.1.4](https://github.com/0b10011/oxiplate-derive/compare/v0.1.3...v0.1.4) - 2025-01-03

### Added

- building the write format with the templates themselves to reduce the number of arguments needed
- calling `write_str()` instead of `write_fmt()` for a single static token
- combined sequential static text and whitespace into a single concat
- combined sequential static text, whitespace, and writs into a single write call

## [0.1.3](https://github.com/0b10011/oxiplate-derive/compare/v0.1.2...v0.1.3) - 2025-01-01

### Other

- build the path to `oxiplate.toml` from the env instead to help with testing

## [0.1.2](https://github.com/0b10011/oxiplate-derive/compare/v0.1.1...v0.1.2) - 2024-12-31

### Fixed

- use correct module for `escape()`

## [0.1.1](https://github.com/0b10011/oxiplate-derive/compare/v0.1.0...v0.1.1) - 2024-12-30

### Fixed

- pass escaper by reference to match expectation in the main crate

### Other

- release v0.1.0

## [0.1.0](https://github.com/0b10011/oxiplate-derive/releases/tag/v0.1.0) - 2024-12-30

### Other

- initial commit

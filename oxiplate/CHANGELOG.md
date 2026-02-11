# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.16.4](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.16.3...oxiplate-v0.16.4) - 2026-02-11

### Added

- switched to dual-licensed `MIT OR Apache-2.0` from `MIT` to match rust ecosystem

### Other

- cleaned up READMEs a bit
- reordered badges to put more important ones earlier

## [0.16.3](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.16.2...oxiplate-v0.16.3) - 2026-02-09

### Added

- `no_std` support (fixes #72)

### Other

- updated READMEs a bit
- removed a dependency by moving the single function used in-tree

## [0.16.2](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.16.1...oxiplate-v0.16.2) - 2026-02-05

### Added

- replaced `toml` and `serde` with in-tree tokenizer and parser for `oxiplate.toml`

### Other

- added tests for `oxiplate.toml` parsing
- *(deps)* bump trybuild from 1.0.114 to 1.0.115
- update toolchain to `nightly-2026-01-28`
- moved `syntax` to `template/parser` and `tokenizer/parser` to `template/tokenizer`
- made parser more generic
- made tokenizer more generic

## [0.16.1](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.16.0...oxiplate-v0.16.1) - 2026-01-26

### Added

- replaced `nom` with in-tree tokenizer and parser, improving error messages and whitespace handling considerably

### Fixed

- stopped rebuilding literal string each time `span()` is called

### Other

- Merge pull request #130 from 0b10011/toolchain-nightly-2026-01-14
- bump toolchain to `nightly-2026-01-14`
- renamed `Tokens` type to `BuiltTokens`
- added/adjusted several tests to cover more edge cases
- *(deps)* bump quote from 1.0.43 to 1.0.44
- *(deps)* bump proc-macro2 from 1.0.105 to 1.0.106
- cleaned up span range code to make it a single call

## [0.16.0](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.15.12...oxiplate-v0.16.0) - 2026-01-14

### Added

- [**breaking**] added support for `stable` by default by removing nightly-only features from `default` feature; new `better-errors` feature contains all hidden-by-default functionality
- moved the `proc_macro_expand` toggle behind the feature `external-template-spans`

### Other

- specified MSRV and started testing it in CI
- removed if let chaining to be able to use a lower MSRV

## [0.15.12](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.15.11...oxiplate-v0.15.12) - 2026-01-13

### Added

- moved the `proc_macro_diagnostic` toggle behind the feature `better-internal-errors` and improved error messages

### Other

- added correct output for unreachable internal errors with `better-internal-errors` feature turned off
- duplicated tests for unreachable internal errors for testing without nightly-only `better-internal-errors` feature
- *(deps)* bump toml from 0.9.10+spec-1.1.0 to 0.9.11+spec-1.1.0

## [0.15.11](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.15.10...oxiplate-v0.15.11) - 2026-01-10

### Fixed

- improved error messages for unexpected statements
- improved error message for unnecessary parentheses

### Other

- added tests for unexpected statements in `block` and `if` statements
- update to latest nightly
- *(deps)* bump proc-macro2 from 1.0.104 to 1.0.105
- *(deps)* bump quote from 1.0.42 to 1.0.43
- *(deps)* bump syn from 2.0.112 to 2.0.114

## [0.15.10](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.15.9...oxiplate-v0.15.10) - 2026-01-07

### Added

- added support for patterns instead of just identifiers in for statements

### Fixed

- switched to `expect()` from `unwrap()` for `CARGO_MANIFEST_DIR` access

## [0.15.9](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.15.8...oxiplate-v0.15.9) - 2026-01-03

### Added

- added `default` filter (for #73)

### Fixed

- fixed doc test failing due to semicolon
- improved error messages a bit and reduced features of `proc-macro2` in the process
- added missing statements to error message
- stopped including whitespace in unclosed writ tag error and reworded to match other errors
- removed unnecessarily specified feature
- reduced `toml` features to only those that are required
- reduced `syn` features to only those that are required

### Other

- `cargo update`
- split broken tests out to run them after the rest of the tests
- fixed a couple tests
- removed `criterion` and simple benchmarks
- split clippy tests out to run after the rest of the tests

## [0.15.8](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.15.7...oxiplate-v0.15.8) - 2025-12-31

### Added

- added support for `let` statements

## [0.15.7](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.15.6...oxiplate-v0.15.7) - 2025-12-27

### Added

- added support for `continue` and `break` statements

### Fixed

- prevented templates from using/overwriting built-in formatter variable and improved error message for `self`/`super` usage (fixes #96)

## [0.15.6](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.15.5...oxiplate-v0.15.6) - 2025-12-26

### Added

- added support for match guards
- added support for multiple match cases in the same arm
- added support for range patterns in `match`/`if let`
- improved support for pattern matching with a refactor of expressions and patterns
- added basic support for `match` (fixes #34)
- added basic support for tuple expressions and destructuring

### Fixed

- added support for single item tuples as match patterns and trailing comma support in all tuple match patterns
- require using return value of `Source::span()`, `Source::merge()`, and `Source::merge_some()`

### Other

- improved test coverage of match statements
- added test for floats in match patterns
- moved pattern-related code to new mod

## [0.15.5](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.15.4...oxiplate-v0.15.5) - 2025-12-22

### Fixed

- fixed issue link for unhandled whitespace adjustment tag
- improved error message for unhandled default escaper group tag
- improved error message for unhandled whitespace adjustment tag

### Other

- added test for unhandled default escaper group tag
- added test for unhandled whitespace adjustment tag
- added tests for unparseable include statement
- added test for adding if items after it's ended
- added test for adding for items after it's ended
- added tests for adding block items after it's ended and an empty block stack

## [0.15.4](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.15.3...oxiplate-v0.15.4) - 2025-12-22

### Fixed

- improved error message for unhandled whitespace command in tag end
- improved error message for unhandled next whitespace adjustment tag
- improved error message for unhandled whitespace command in next tag start
- improved error message for unhandled whitespace command in tag start
- improved error message for unhandled prefix operator
- improved error message for unhandled operator
- improved error message for unhandled bool

### Other

- added tests for unhandled commands/tags
- added test for unhandled operator
- added tests for invalid chars
- added test for unhandled base prefix
- added test for unhandled bool case
- added tests for invalid escape sequences
- added tests for unreachable match arms for unicode escapes
- added tests for unreachable match arms for 7-bit escapes

## [0.15.3](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.15.2...oxiplate-v0.15.3) - 2025-12-21

### Fixed

- improved error message for unreachable unregistered file extensions error
- when using `oxiplate-derive`, stopped turning on the optimized renderer by default and improved the error message when it is accidentally enabled

### Other

- added test for unreachable unregistered file extensions error
- added test for unhandled prefix operator
- added test for failing to parse derive input

## [0.15.2](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.15.1...oxiplate-v0.15.2) - 2025-12-19

### Fixed

- improved unexpected `(default|replace)_escaper_group` statement errors by highlighting the entire statement
- fixed writ tokens to more consistently not be generated when failing to replace or set a default escaper group could affect their output
- switched span of writs to full tag

### Other

- added test coverage badge
- fixed expansion tests to be more consistent
- started running `rustfmt` against broken test files
- added tests for broken inline templates with escapers
- added test for attempting to use a non-existent escaper group in an inline template
- split default/replace escaper group statement tests
- added test for using default escaper when config is turned off
- started passing `--color` option directly to clippy
- fixed clippy test output to be more consistent
- moved `syn::parse()` call up to `oxiplate_internal()` to make it clear that parsing after that doesn't fail
- reduce uncovered regions by moving EOF error range adjustments to a macro
- added comment about why an unreachable error message is present

## [0.15.1](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.15.0...oxiplate-v0.15.1) - 2025-12-13

### Added

- improved handling of parsing errors to generate impls of `Display` and `Render`

### Other

- added test for when the default escaper is set to raw using oxiplate
- added tests for escaper group config coverage
- improved coverage by including escaper name when default escaper group is set
- added test for invalid fallback escaper groups
- simplified type to get rid of warning
- added test for whitespace followed by `{` to improve coverage

## [0.15.0](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.14.0...oxiplate-v0.15.0) - 2025-12-12

### Added

- [**breaking**] improved whitespace handling to be more consistent between statements and whitespace control tags; `{_}` now produces an error when it's not touching whitespace
- improved spans for errors, stopped throwing away previous items/errors when EOF was reached in a statement, and started testing full template ranges in every test

### Other

- added test for `include` statement in `oxiplate`
- added test for `.raw` file extension
- added test for unclosed `block` statement
- added test for unclosed if tag
- added test for accessing local variables from outside for loop inside of it
- adjusted test to cover `!` and `*` prefix operators
- added tests for 7-bit escape edge cases
- added tests and removed unused trait impls to improve coverage

## [0.14.0](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.13.0...oxiplate-v0.14.0) - 2025-12-10

### Fixed

- started deriving `Debug` for `Writ`
- improved error message for missing space after `if`/`elseif`
- fixed handling of paths in destructuring

### Other

- added test for path segments without a final type name in destructuring
- added test for malformed struct value
- added test for space before comma in destructuring
- added test for unit struct destructuring
- [**breaking**] removed `if let` patterns using a variable instead of a type before the `=`
- added tests for nested struct destructuring
- added test for whitespace around parentheses in tuple structs

## [0.13.0](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.12.2...oxiplate-v0.13.0) - 2025-12-09

### Added

- added support for literals as values when destructuring
- improved span information for if let destructuring
- added `if let` destructuring for tuple structs with more than one field and c/unit structs

### Fixed

- [**breaking**] added support for unit structs, but removed automatic insertion of variable assignment in if let statements to improve related error messages

### Other

- adjusted test to cover more lines
- added test for malformed struct field
- added test for 2 tuple fields without an expression
- removed unused local variable code

## [0.12.2](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.12.1...oxiplate-v0.12.2) - 2025-12-05

### Fixed

- stopped silently ignoring escaper specified in inline templates when the `infer_escaper_group_from_file_extension` option is turned off

### Other

- stopped using static macro to fetch environment variable so coverage profiles will merge properly
- move external test into new dedicated folder
- added test for template paths that are directories
- added test for template paths that are symlinks

## [0.12.1](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.12.0...oxiplate-v0.12.1) - 2025-12-04

### Fixed

- improved error message when a comment tag is not closed
- stopped clearing block suffix if a second parent was found and started properly reporting the error
- replaced todo with unreachable for blocks to match other statements

### Other

- added tests for non-literal macros in attributes
- added test for a malformed config file
- expanded coverage for escaping
- expanded coverage for extends
- added tests for non-string literals in attributes
- remove unused implementation
- switched to deriving debug for `SourceOwned` and removed unused code found as a result
- added tests for whitespace replacement around statements
- split comment and writ whitespace replacement tests for better readability of the errors
- added test for missing block name
- replaced unreachable code with diagnostic info and a panic
- removed some commented out old code
- removed old extends code that's no longer needed
- switched to deriving debug for `Extends`
- moved some operator tests into the new folder for them
- expanded coverage for operators

## [0.12.0](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.11.7...oxiplate-v0.12.0) - 2025-12-03

### Fixed

- [**breaking**] fixed `if let` without an assignment for non-local variables and stopped automatically borrowing `if let` assignments
- fixed calling callbacks on the top level struct

### Other

- improved test coverage
- ignored unreachable code for test coverage
- improved test coverage

## [0.11.7](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.11.6...oxiplate-v0.11.7) - 2025-11-28

### Fixed

- fixed parsing of empty strings
- improved error message for writs after extends
- improved error message for unhandled alternative base prefixes
- improved error message for text after extends
- replaced panics in source parsing code with spanned errors (fixes #82)
- improved error messages when char is not a single char
- simplified error message when attempting to merge disjointed ranges so both spans appear for the main error message
- improved error message if an open tag is ever not handled
- improved error message when `super` or `self` are used in a variable assignment

### Other

- started testing multi-byte characters with the `lower` and `upper` filters
- inlined `CowStr` related trait methods
- added tests for bool usage
- removed code duplicated by the trait itself
- added tests for text/writs in extends
- use `print!()` instead of `panic!()` in a couple tests that don't rely on panic functionality
- added test for `self` and `super` ident usage

## [0.11.6](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.11.5...oxiplate-v0.11.6) - 2025-11-21

### Added

- support for calling filters that don't accept additional arguments without parentheses
- macro for building `CowStrWrapper` for testing filters dealing with string-like values

### Fixed

- `lower` now lowercases owned strings instead of capitalizing them

### Other

- added tests for `lower` and `upper`

## [0.11.5](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.11.4...oxiplate-v0.11.5) - 2025-11-20

### Added

- add `upper` and `lower` filters
- added efficient conversion to `Cow<'a, str>` via `CowStr` trait for filters

## [0.11.4](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.11.3...oxiplate-v0.11.4) - 2025-11-17

### Fixed

- improved spans for error messages and string/char support

### Other

- reorganized broken string tests

## [0.11.3](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.11.2...oxiplate-v0.11.3) - 2025-09-27

### Fixed

- fixed chaining filters

## [0.11.2](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.11.1...oxiplate-v0.11.2) - 2025-08-31

### Added

- moved `itoa` dependency (and fast escaping of integers) behind the `fast-escape-ints` feature flag (fixes #67)

## [0.11.1](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.11.0...oxiplate-v0.11.1) - 2025-08-31

### Fixed

- fixed README path

## [0.11.0](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.10.4...oxiplate-v0.11.0) - 2025-08-31

### Added

- split traits into separate crate
- [**breaking**] add estimated length and rendering directly into a string

### Other

- split descriptions, versions, and readmes
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

## [0.10.4](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.10.3...oxiplate-v0.10.4) - 2025-08-29

### Added

- added support for filters (fixes #26)

### Other

- split operator/prefix operator code into separate mod

## [0.10.3](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.10.2...oxiplate-v0.10.3) - 2025-08-26

### Added

- added basic support for chars (fixes #65)

### Fixed

- improved error messages when a template can't be loaded
- improved error message for non-name-value attributes for external templates

### Other

- split template path code into a separate function
- removed duplicate test

## [0.10.2](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.10.1...oxiplate-v0.10.2) - 2025-08-23

### Fixed

- added handling for some path edge cases and a bunch of new tests for error messages
- improved error message when using the wrong syntax for inline templates
- improved error message when failing to parse inline
- improved error mesage when escaper group is provided to an inline template without using `oxiplate`
- improved error message for inline attribute without a value
- improved error message for missing template attribute

### Other

- updated to 2024 edition
- fixed some linting warnings
- combine some nested `if let` statements
- added tests for a bunch of derive errors

## [0.10.1](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.10.0...oxiplate-v0.10.1) - 2025-08-23

### Added

- added support for indexing and range expressions (fixes #53)
- added support for arguments in function/method calls

### Other

- fix expansion tests for recent nightly update
- updated to latest nightly
- added tests for negative numbers
- removed test that was failing due to a clippy change
- removed unused variable in test

## [0.10.0](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.9.4...oxiplate-v0.10.0) - 2025-08-22

### Fixed

- [**breaking**] fixed order of operations to match rust's

## [0.9.4](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.9.3...oxiplate-v0.9.4) - 2025-08-22

### Added

- added support for parentheses in expressions (fixes #49)

### Fixed

- fixed concat expressions that only contain literals

### Other

- moved return values into match to shorten a bit and aid in future refactoring

## [0.9.3](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.9.2...oxiplate-v0.9.3) - 2025-08-21

### Added

- made it possible to specify/override the escaper group from within a template (fixes #39)

### Fixed

- improve error message for extends statements after other content

### Other

- added tests for extends processing
- remove commented out code

## [0.9.2](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.9.1...oxiplate-v0.9.2) - 2025-08-20

### Other

- move state from parsing to token building and improve length estimation along the way

## [0.9.1](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.9.0...oxiplate-v0.9.1) - 2025-08-18

### Added

- added support for including templates (fixes #35)

### Fixed

- use `unreachable!()` instead of `todo!()` for `add_item()` calls after statement is closed (fixes #23)
- improve error message for extra `else` statements in `for` statement (fixes #25)
- improve error message for extra `else` statements in `if` statement (fixes #24)

### Other

- remove unused templates
- moved extends tests to the macro
- added test for extra `else` statements in `if` statement
- added test for extra `else` statements in `for` statement
- reorganize tests a bit to make them easier to navigate

## [0.9.0](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.8.1...oxiplate-v0.9.0) - 2025-08-17

### Added

- [**breaking**] improve error messages for unrecognized escaper groups

## [0.8.1](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.8.0...oxiplate-v0.8.1) - 2025-08-17

### Fixed

- improved error message when specified template cannot be found
- improve error message for unwritable writs
- get rid of unnecessary clone

### Other

- added tests for invalid template paths

## [0.8.0](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.7.0...oxiplate-v0.8.0) - 2025-08-13

### Added

- added specialization for escaping and outputting raw text for a ~3-5x improvement in runtime performance

### Removed

- [**breaking**] removed `oxiplate::escapers::escape()` that was previously called from macro-generated code

## [0.7.0](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.6.2...oxiplate-v0.7.0) - 2025-08-07

### Added

- [**breaking**] add estimated length and rendering directly into a string

## [0.6.2](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.6.1...oxiplate-v0.6.2) - 2025-08-05

### Added

- simplify generated code to avoid `dyn`, callbacks, and more

### Other

- use more performant render function when extending templates

## [0.6.1](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.6.0...oxiplate-v0.6.1) - 2025-07-28

### Added

- add support for floating-point literals (fixes #22)
- add support for octal (fixes #20) and hexadecimal (fixes #21) literals

## [0.6.0](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.5.1...oxiplate-v0.6.0) - 2025-07-26

### Other

- remove `<` from reserved characters in html attributes
- [**breaking**] pass writer into escaper to avoid extra allocations
- inline escape function for ~10% gains on html
- write braces with rest of static text

## [0.5.1](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.5.0...oxiplate-v0.5.1) - 2025-07-26

### Added

- underscore separator support in numbers (closes #18)

### Other

- add an optimized render function
- use `write_str` instead of `write_fmt` whenever possible
- add basic benchmarking

## [0.5.0](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.4.1...oxiplate-v0.5.0) - 2025-07-24

### Added

- [**breaking**] infer escaper from file extension or extension hint
- make it possible to require specifying escapers on all writs (fixes #33)

## [0.4.1](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.4.0...oxiplate-v0.4.1) - 2025-07-22

### Added

- automatically register built-in escapers and set the default escaper to `html`
- put the custom config file behind a feature flag to make the library usable without serde
- add JSON escaper (fixes #38)
- point to the escaper name in the error when it can't be matched to a variant on the escaper group's enum

### Other

- fix path to HTML and Markdown escapers
- rename test crate to make it more clear in the dependencies what it is
- improve highlighting of code block filenames

## [0.4.0](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.3.0...oxiplate-v0.4.0) - 2025-07-18

### Added

- [**breaking**] infer placement of parent block content from parent tag existence and placement

## [0.3.0](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.2.8...oxiplate-v0.3.0) - 2025-07-16

### Added

- [**breaking**] escape by default and require an escaper to be specified in the project

### Other

- fix links in released crate
- use correct error message

## [0.2.8](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.2.7...oxiplate-v0.2.8) - 2025-07-15

### Added

- only replace comment characters in escaped comments in HTML when disallowed patterns are present

### Fixed

- get rid of warning about simplifying comparison

### Other

- add more information about the main focuses to the READMEs

## [0.2.7](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.2.6...oxiplate-v0.2.7) - 2025-07-15

### Other

- update tests for latest rust nightly version
- upgrade to nom 8
- update `toml` to latest

## [0.2.6](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.2.5...oxiplate-v0.2.6) - 2025-07-13

### Added

- add support for binary literals (closes #19)

### Other

- split literal-related code into separate mod
- split ident-related code into separate mod
- split keyword-related code into separate mod
- move expression mod into a folder in prep for splitting
- remove some TODOs that now have issues

## [0.2.5](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.2.4...oxiplate-v0.2.5) - 2025-03-26

### Fixed

- update name of function to match reality

### Other

- windows -> linux and rust upgrade test output changes
- add some missing docs

## [0.2.4](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.2.3...oxiplate-v0.2.4) - 2025-01-28

### Fixed

- support for multiple blocks in the same file

### Other

- move a couple test files to the proper folder
- crlf test with current and fixed output

## [0.2.3](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.2.2...oxiplate-v0.2.3) - 2025-01-25

### Fixed

- more accurate spans on parent templates

### Other

- support for subdirectories in oxiplate broken tests
- clippy fixes

## [0.2.2](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.2.1...oxiplate-v0.2.2) - 2025-01-14

### Fixed

- extends now uses `oxiplate` instead of `oxiplate_derive` to avoid an extra import by the user

## [0.2.1](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.2.0...oxiplate-v0.2.1) - 2025-01-13

### Fixed

- raw escaping when default escaper is set and escaping non-string slices

## [0.2.0](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.1.12...oxiplate-v0.2.0) - 2025-01-13

### Added

- [**breaking**] prevent whitespace replace characters when there's no matching whitespace to replace

### Other

- simplify whitespace adjustment checks in end tags

## [0.1.12](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.1.11...oxiplate-v0.1.12) - 2025-01-13

### Added

- fail to compile if adjacent tags with only whitespace between explicitly specify different whitespace adjustment characters
- only pass braces in as arguments instead of entire strings that contain them

### Other

- add more text to the format injection text to better show changes with normal text
- reduce item count for static text with whitespace

## [0.1.11](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.1.10...oxiplate-v0.1.11) - 2025-01-10

### Added

- include the parent contents in an overridden block
- support for `{% else %}` in for loops

### Other

- update Cargo.lock dependencies
- remove `Cargo.lock` for the macro and ensure another doesn't get committed
- conditionally compile expansion tests to make it clear they will fail on old rustc versions

## [0.1.10](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.1.9...oxiplate-v0.1.10) - 2025-01-09

### Added

- combine static strings in concats into the format string for smaller templates
- concat operator (~)

### Fixed

- string continuation escapes and spans after escape sequences

### Other

- merge variable tests into a single file
- clear `/actual/` each run, error for leftover files, and fix the check against `expansion.rs`
- turn on `format_strings` and reformat
- updated expected expansion after rust update

## [0.1.9](https://github.com/0b10011/oxiplate/compare/oxiplate-v0.1.8...oxiplate-v0.1.9) - 2025-01-08

### Fixed

- building templates when `std` is overridden

### Other

- add additional rustfmt rules and reformat
- synchronize versions
- cleaned up the attribute/struct parsing code even more
- split out state, add some documentation, and clean up parsing code a bit
- update expected calc expansion
- cover more complicated calculations
- cover more complicated cases of `||` and `&&` checks
- add docs for the derive macro
- moved `Source` and `SourceOwned` into its own module

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

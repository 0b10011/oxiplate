# Oxiplate derive macro

[![Latest Version]][crate] [![MIT OR Apache-2.0 License]][license] [![MSRV]][crate] [![Coverage Status]][coverage] [![Open Issues]][issues] [![Repository][]][repo] [![Docs Build Status]][docs]

[Latest Version]: https://img.shields.io/crates/v/oxiplate-derive
[crate]: https://crates.io/crates/oxiplate-derive
[Repository]: https://img.shields.io/github/commits-since/0b10011/oxiplate/latest?label=unreleased+commits
[repo]: https://github.com/0b10011/oxiplate
[Docs Build Status]: https://img.shields.io/docsrs/oxiplate-derive
[docs]: https://docs.rs/oxiplate-derive/latest/oxiplate_derive/
[Coverage Status]: https://img.shields.io/coverallsCoverage/github/0b10011/oxiplate
[coverage]: https://coveralls.io/github/0b10011/oxiplate?branch=main
[MIT OR Apache-2.0 License]: https://img.shields.io/crates/l/oxiplate-derive
[license]: https://github.com/0b10011/oxiplate/#license
[Open Issues]: https://img.shields.io/github/issues-raw/0b10011/oxiplate
[issues]: https://github.com/0b10011/oxiplate/issues
[MSRV]: https://img.shields.io/crates/msrv/oxiplate-derive

Derive macro for [Oxiplate](https://crates.io/crates/oxiplate).

Can technically be used on its own
as a basic compile-time template system for Rust
with a focus on helpful error messages and whitespace control.
But for escaping functionality and a considerably more efficient renderer,
the [main library](https://crates.io/crates/oxiplate) should be used instead.

**Warning**:
Packages depending on `oxiplate-derive` directly
cannot be used with packages that depend on `oxiplate`
due to [feature unification](https://doc.rust-lang.org/cargo/reference/features.html#feature-unification)
and the `_oxiplate` feature being used
to generate code specific to `oxiplate` or `oxiplate-derive` usage.
It may be possible to build a glue layer
by importing a crate or module as `oxiplate`
everywhere Oxiplate templates are used
and gating it to the `_oxiplate` feature,
but this is not recommended.

Using the macro directly works similarly to the main library,
but `oxiplate_derive::Oxiplate` needs to be imported instead of `oxiplate::prelude::*`
and `format!("{}", Data { .. })` needs to be used instead of `Data { .. }.render()?`.

## Using Oxiplate in your project

- [Oxiplate overview](https://0b10011.io/oxiplate/)
- [Getting started guide](https://0b10011.io/oxiplate/getting-started.html)
- [Release notes](https://github.com/0b10011/oxiplate/releases)
- [API docs](https://docs.rs/oxiplate)

## Hacking on Oxiplate

- [How to contribute](https://github.com/0b10011/oxiplate/blob/main/CONTRIBUTING.md)

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](https://github.com/0b10011/oxiplate/blob/main/LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](https://github.com/0b10011/oxiplate/blob/main/LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

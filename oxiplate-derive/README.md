# Oxiplate 

[![Latest Version]][crate] [![Repository][]][repo] [![Docs Build Status]][docs] [![Coverage Status]][coverage] [![MIT OR Apache-2.0 License]][license] [![Open Issues]][issues] [![MSRV]][crate]

[Latest Version]: https://img.shields.io/crates/v/oxiplate-derive
[crate]: https://crates.io/crates/oxiplate-derive
[Repository]: https://img.shields.io/github/commits-since/0b10011/oxiplate/latest?label=unreleased+commits
[repo]: https://github.com/0b10011/oxiplate
[Docs Build Status]: https://img.shields.io/docsrs/oxiplate
[docs]: https://docs.rs/oxiplate/latest/oxiplate/
[Coverage Status]: https://img.shields.io/coverallsCoverage/github/0b10011/oxiplate
[coverage]: https://coveralls.io/github/0b10011/oxiplate?branch=main
[MIT OR Apache-2.0 License]: https://img.shields.io/crates/l/oxiplate-derive
[license]: https://github.com/0b10011/oxiplate/#license
[Open Issues]: https://img.shields.io/github/issues-raw/0b10011/oxiplate
[issues]: https://github.com/0b10011/oxiplate/issues
[MSRV]: https://img.shields.io/crates/msrv/oxiplate-derive

Oxiplate is an *experimental* compile-time template system for Rust with a focus on helpful error messages, escaping, and whitespace control. Use at your own risk.

## Using Oxiplate in your project

- [Oxiplate overview](https://0b10011.io/oxiplate/)
- [Getting started guide](https://0b10011.io/oxiplate/getting-started.html)
- [Release notes](https://github.com/0b10011/oxiplate/releases)
- [API docs](https://docs.rs/oxiplate)

## Hacking on Oxiplate

- [How to contribute](https://github.com/0b10011/oxiplate/blob/main/CONTRIBUTING.md)

## Helpful error messages

Position information is tracked across files and passed onto Rust.
This results in debuggable error messages
even when issues are caught by Rust instead of Oxiplate.

```html.oxip
<h1>{{ title }}</h1>
<p>{{ message }}</p>
```

```rust,compile_fail
use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "external.html.oxip"]
struct HelloWorld {
    title: &'static str,
    messages: &'static str,
}

let hello_world = HelloWorld {
    title: "Oxiplate error handling",
    messages: "Hello world!",
};

format!("{}", hello_world);
```

```text
error[E0609]: no field `messages` on type `&HelloWorld`
 --> /templates/external.html.oxip:2:7
  |
2 | <p>{{ message }}</p>
  |       ^^^^^^^ unknown field
  |
help: a field with a similar name exists
  |
2 - <p>{{ message }}</p>
2 + <p>{{ messages }}</p>
  |
```

Check out the broken tests directory of 
[`oxiplate`](https://github.com/0b10011/oxiplate/tree/main/oxiplate/tests/broken) and 
[`oxiplate-derive`](https://github.com/0b10011/oxiplate/tree/main/oxiplate-derive/tests/broken)
for (tested) example error messages.

## Whitespace control

Oxiplate supports removing trailing/leading/surrounding whitespace,
or even collapsing it down to a single space.

```html.oxip
{# Say hi and bye -#}
<a href="#">{-}
    Hello {{ name -}}
</a>{_}
<a href="#">{-}
    Goodbye
    {{_ name -}}
</a>
```

```html
<a href="#">Hello Bell</a> <a href="#">Goodbye Bell</a>
```

Read the full [whitespace control chapter](https://0b10011.io/oxiplate/templates/whitespace-control.html) for more information.

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

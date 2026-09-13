# Oxiplate

[![Latest Version]][crate] [![MIT OR Apache-2.0 License]][license] [![MSRV]][crate] [![Coverage Status]][coverage] [![Open Issues]][issues] [![Repository][]][repo] [![Docs Build Status]][docs]

[Latest Version]: https://img.shields.io/crates/v/oxiplate
[crate]: https://crates.io/crates/oxiplate
[Repository]: https://img.shields.io/github/commits-since/0b10011/oxiplate/latest?label=unreleased+commits
[repo]: https://github.com/0b10011/oxiplate
[Docs Build Status]: https://img.shields.io/docsrs/oxiplate
[docs]: https://docs.rs/oxiplate/latest/oxiplate/
[Coverage Status]: https://img.shields.io/coverallsCoverage/github/0b10011/oxiplate
[coverage]: https://coveralls.io/github/0b10011/oxiplate?branch=main
[MIT OR Apache-2.0 License]: https://img.shields.io/crates/l/oxiplate
[license]: https://github.com/0b10011/oxiplate/#license
[Open Issues]: https://img.shields.io/github/issues-raw/0b10011/oxiplate
[issues]: https://github.com/0b10011/oxiplate/issues
[MSRV]: https://img.shields.io/crates/msrv/oxiplate

Oxiplate is a work-in-progress compile-time template engine for Rust
with a focus on helpful error messages, escaping, and whitespace control,
while still including the primary functionality expected from such a library.

## Features

- [Helpful errors](#helpful-error-messages) regardless of where or when the error occurs (`better-errors` feature)
- [Escaping for any markup language][escaping] with escapers scoped accordingly, specified first in writs, and guaranteed to run last
- [Powerful whitespace control][whitespace control] around all tags with `-` and `_`
- [Filters anywhere expressions are accepted][filters] with `EXPRESSION | FILTER` and `EXPRESSION | FILTER(ARGUMENTS)`
- Destructuring (pattern matching) in `for`, `if let`, `elseif let`, and `case` statements
- More efficient string handling with the [cow prefix][cow prefix] and [specialized escaping calls][unescaped text]
- [Straightforward expression precedence][precedence]
- [Separate library for traits][traits] to ensure third-party escapers and filters continue working even when there are breaking changes to the main libraries

[cow prefix]: https://0b10011.io/oxiplate/templates/expressions/index.html#cow-prefix-for-more-efficient-string-conversion
[escaping]: https://0b10011.io/oxiplate/templates/writs/escaping.html
[filters]: https://0b10011.io/oxiplate/templates/expressions/filters.html
[precedence]: https://0b10011.io/oxiplate/templates/expressions/expression-precedence.html
[traits]: https://crates.io/crates/oxiplate-traits
[unescaped text]: https://github.com/0b10011/oxiplate/blob/main/oxiplate-traits/src/unescaped_text.rs
[whitespace control]: https://0b10011.io/oxiplate/templates/whitespace-control.html

### Supported tags

- [Whitespace control short tags][whitespace short tags] with `{-}` and `{_}`
- [Writs to output content][writs] with `{{ ESCAPER_GROUP.ESCAPER: EXPRESSION }}`
- [Statements][statements] with `{% STATEMENT %}`
- [Comments that are discarded from the final template][comments] with `{# COMMENT #}`

[whitespace short tags]: https://0b10011.io/oxiplate/templates/whitespace-control.html#short-tags
[writs]: https://0b10011.io/oxiplate/templates/writs/index.html
[statements]: https://0b10011.io/oxiplate/templates/statements/index.html
[comments]: https://0b10011.io/oxiplate/templates/tags.html#comments

### Supported statements

- [Template inheritance][template inheritance] with `{% extends "TEMPLATE_PATH" %}`, `{% block BLOCK_NAME %}`, `{% parent %}`, and `{% endblock %}`
- [Include content from other templates][include] with `{% include "TEMPLATE_PATH" %}`
- [`for` loops][for] with `{% for PATTERN in EXPRESSION %}`, `{% else %}`, and `{% endfor %}`
- [`if` statements][if] with `{% if EXPRESSION %}`, `{% elseif EXPRESSION %}`, `{% else %}`, and `{% endif %}`
  - Pattern matching also supported with `{% if let PATTERN = EXPRESSION %}` (`elseif` also works)
- [`match` statements][match] with `{% match EXPRESSION %}`, `{% case PATTERNS %}`, and `{% endmatch %}`
  - Match guards are also supported: `{% case PATTERN if EXPRESSION %}`
- [`let` statements][let] with `{% let PATTERN = EXPRESSION %}`
- [In-template default escaper group control][default escaper group] with `{% default_escaper_group GROUP_NAME %}` and `{% replace_escaper_group GROUP_NAME %}`

[default escaper group]: https://0b10011.io/oxiplate/templates/statements/default-escaper-group.html
[expressions]: https://0b10011.io/oxiplate/templates/expressions.html
[if]: https://0b10011.io/oxiplate/templates/statements/if-else.html
[for]: https://0b10011.io/oxiplate/templates/statements/for.html
[include]: https://0b10011.io/oxiplate/templates/statements/include.html
[let]: https://0b10011.io/oxiplate/templates/statements/let.html
[match]: https://0b10011.io/oxiplate/templates/statements/match.html
[template inheritance]: https://0b10011.io/oxiplate/templates/statements/extends.html

### What's missing?

While 100% feature parity isn't a requirement,
ensuring there's at lease _some_ way to achieve everything is important.
Parity is being tracked across a few issues:

- [Add common filters (#73)][common filters]
- [Feature parity with other template engines (#302)][feature parity]
- [Built-in framework support (#303)][framework support]

There's also an [experimental branch to add translation support][translation branch],
but that may or may not make it in before 1.0.

[common filters]: https://github.com/0b10011/oxiplate/issues/73
[feature parity]: https://github.com/0b10011/oxiplate/issues/302
[framework support]: https://github.com/0b10011/oxiplate/issues/303
[translation branch]: https://github.com/0b10011/oxiplate/tree/translation

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

```add.html.oxip
{{ a }} + {{ b }} = {{ a + b }}
```

```rust,compile_fail
use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate = "add.html.oxip"]
struct Add {
    a: u32,
    b: u64,
}

let add = Add {
    a: 10,
    b: 9,
};

print!("{}", add.render()?);

Ok::<(), ::std::fmt::Error>(())
```

```text
error[E0308]: mismatched types
 --> ./templates/add.html.oxip:1:28
  |
1 | {{ a }} + {{ b }} = {{ a + b }}
  |                            ^ expected `u32`, found `u64`

error[E0277]: cannot add `u64` to `u32`
   --> ./templates/add.html.oxip:1:26
    |
  1 | {{ a }} + {{ b }} = {{ a + b }}
    |                          ^ no implementation for `u32 + u64`
    |
    = help: the trait `std::ops::Add<u64>` is not implemented for `u32`
[...]
```

Check out the broken tests directory of
[`oxiplate`](https://github.com/0b10011/oxiplate/tree/main/oxiplate/tests/broken) and
[`oxiplate-derive`](https://github.com/0b10011/oxiplate/tree/main/oxiplate-derive/tests/broken)
for (tested) example error messages.

## Escaping

Escaping is arguably the most important feature of a template system.
The escaper name appears first to make it easier to spot,
and always runs last to ensure the output is always safe.
Creating templates in a language not supported by Oxiplate?
You can add your own escapers!

```html.oxip:profile-link.html.oxip
<!-- Profile link for {{ comment: name }} -->
<a href="{{ attr: url }}">{{ text: name }}</a>
```

```rust
use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate = "profile-link.html.oxip"]
struct ProfileLink {
    url: &'static str,
    name: &'static str,
}

let profile_link = ProfileLink {
    url: r#""><script>alert("hacked!");</script>"#,
    name: r#"<!-- --><script>alert("hacked!");</script><!-- -->"#
};

assert_eq!(
    profile_link.render()?,
    r#"<!-- Profile link for ‹ǃ−− −−›‹script›alert("hackedǃ");‹/script›‹ǃ−− −−› -->
<a href="&#34;><script>alert(&#34;hacked!&#34;);</script>">&lt;!-- -->&lt;script>alert("hacked!");&lt;/script>&lt;!-- --></a>
"#,
);

Ok::<(), ::std::fmt::Error>(())
```

```html
<!-- Profile link for ‹ǃ−− −−›‹script›alert("hackedǃ");‹/script›‹ǃ−− −−› -->
<a href="&#34;><script>alert(&#34;hacked!&#34;);</script>">&lt;!-- -->&lt;script>alert("hacked!");&lt;/script>&lt;!-- --></a>
```

Read the full [escaping chapter](https://0b10011.io/oxiplate/templates/writs/escaping.html) for more information.

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

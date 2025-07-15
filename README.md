# Oxiplate 

[![Latest Version]][crate] [![Repository][]][repo] [![Docs Build Status]][docs] [![MIT License]][license] [![Open Issues]][issues]

[Latest Version]: https://img.shields.io/crates/v/oxiplate
[crate]: https://crates.io/crates/oxiplate
[Repository]: https://img.shields.io/github/commits-since/0b10011/oxiplate/latest?label=unreleased+commits
[repo]: https://github.com/0b10011/oxiplate
[Docs Build Status]: https://img.shields.io/docsrs/oxiplate
[docs]: https://docs.rs/oxiplate/latest/oxiplate/
[MIT License]: https://img.shields.io/github/license/0b10011/oxiplate
[license]: https://github.com/0b10011/oxiplate/blob/main/LICENSE
[Open Issues]: https://img.shields.io/github/issues-raw/0b10011/oxiplate
[issues]: https://github.com/0b10011/oxiplate/issues

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

```oxip
<h1>{{ title }}</h1>
<p>{{ messages }}</p>
```

```rust,compile_fail
use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "external.html.oxip"]
struct HelloWorld {
    title: &'static str,
    message: &'static str,
}

let hello_world = HelloWorld {
    title: "Hello world",
};

print!("{hello_world}");
```

```text
error[E0609]: no field `messages` on type `&HelloWorld`
 --> templates/external.html.oxip
  |
  | <p>{{ messages }}</p>
  |       ^^^^^^^^ unknown field
  |
  = note: available field is: `message`
```

Check out the broken tests directory of 
[`oxiplate`](/oxiplate/tests/broken) and 
[`oxiplate-derive`](/oxiplate-derive/tests/broken)
for (tested) example error messages.

## Escaping

Escaping is arguably the most important feature of a template system.
The escaper name appears first to make it easier to spot,
and always runs last to ensure the output is always safe.
Creating templates in a language not supported by Oxiplate?
You can add your own escapers!

```oxip
<!-- Profile link for {{ comment: name }} -->
<a href="{{ attr: url }}">{{ text: name }}</a>
```

```rust,compile_fail
use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "external.html.oxip"]
struct ProfileLink {
    url: &'static str,
    name: &'static str,
}

let profile_link = ProfileLink {
    url: r#""><script>alert("hacked!");</script>"#,
    name: r#"<!-- --><script>alert("hacked!");</script><!-- -->"#
};

print!("{profile_link}");
```

```html
<!-- Profile link for ‹ǃ−− −−›‹script›alert("hackedǃ");‹/script›‹ǃ−− −−› -->
<a href="&#34;><script>alert(&#34;hacked!&#34;);</script>">&lt;!-- -->&lt;script>alert("hacked!");&lt;/script>&lt;!-- --></a>
```

Read the full [escaping chapter](https://0b10011.io/oxiplate/templates/writs/escaping.html) for more information.

## Whitespace control

Oxiplate supports removing trailing/leading/surrounding whitespace,
or even collapsing it down to a single space.

```oxip
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

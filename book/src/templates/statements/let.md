# Setting local variables with `let`

Values can be bound to a variable with `let` statements.

```oxip
{% let PATTERN = EXPRESSION %}
```

To simplify migrating to Oxiplate,
`set` is an alias for `let`:

```oxip
{% set PATTERN = EXPRESSION %}
```

## Example

```rust
# extern crate oxiplate;
#
use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate_inline(html: r#"
{%- let display_name = name ~ " (" ~ company ~ ")" -%}
<h1 title="{{ attr: display_name }}">{{ display_name }}</h1>"#)]
struct YourStruct {
    name: &'static str,
    company: &'static str,
}

assert_eq!(
    YourStruct {
        name: "Felix",
        company: "ABC Shipping",
    }.render()?,
    r#"<h1 title="Felix (ABC Shipping)">Felix (ABC Shipping)</h1>"#
);
#
# Ok::<(), ::core::fmt::Error>(())
```

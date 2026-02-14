# Setting local variables with `let`

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

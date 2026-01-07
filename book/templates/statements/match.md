# Pattern matching with `match` and `case`

```rust
# extern crate oxiplate;
#
use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"
{%- match value %}
{%- case ..0 -%}
    Less than zero
{%- case 0 -%}
    Zero
{%- case _ -%}
    Greater than zero
{%- endmatch %}"#)]
struct YourStruct {
    value: isize,
}

assert_eq!(
    YourStruct {
        value: -19
    }.render()?,
    "Less than zero"
);
#
# Ok::<(), ::core::fmt::Error>(())
```

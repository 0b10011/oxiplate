# Branching with `if`, `elseif`, and `else`

```text:if.html.oxip
<p>
    {%- if count < 0 -%}
        {{ count }} is negative
    {%- elseif count > 0 -%}
        {{ count }} is positive
    {%- else -%}
        {{ count }} is zero
    {%- endif -%}
</p>{-}
```

```rust
# extern crate oxiplate;
#
use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate = "if.html.oxip"]
struct YourStruct {
    count: i64,
}

assert_eq!(
    YourStruct {
        count: 19,
    }.render()?,
    "<p>19 is positive</p>",
);
#
# Ok::<(), ::core::fmt::Error>(())
```

## `if let` and `elseif let`

Similarly to Rust,
`let` can be used in `if` and `elseif` statements.

```html:if-let.html.oxip
<p>
    {%- if let Some(count) = count -%}
        The count is {{ count }}.
    {%- else -%}
        No count provided.
    {%- endif -%}
</p>{-}
```

```rust
# extern crate oxiplate;
#
use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate = "if-let.html.oxip"]
struct YourStruct {
    count: Option<i64>,
}

assert_eq!(
    YourStruct {
        count: Some(19),
    }.render()?,
    "<p>The count is 19.</p>",
);
#
# Ok::<(), ::core::fmt::Error>(())
```

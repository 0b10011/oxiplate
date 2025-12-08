# Branching with `if`, `elseif`, and `else`

```rust
#[derive(Oxiplate)]
#[oxiplate = "template.html.oxip"]
struct YourStruct {
    count: i64,
}
```

```rust
print!("{}", YourStruct {
    count: 19,
});
```

```html.oxip
<p>
    {%- if count < 0 -%}
        {{ count }} is negative
    {%- elseif count > 0 -%}
        {{ count }} is positive
    {%- else -%}
        {{ count }} is zero
    {%- endif -%}
</p>
```

```html
<p>19 is positive</p>
```

## `if let` and `elseif let`

Similarly to Rust,
`let` can be used in `if` and `elseif` statements.

```rust
#[derive(Oxiplate)]
#[oxiplate_inline(html: r#"
<p>
    {%- if let Some(count) = count -%}
        The count is {{ count }}.
    {%- else -%}
        No count provided.
    {%- endif -%}
</p>
"#)]
struct YourStruct {
    count: Option<i64>,
}

assert_eq!("<p>The count is 19.</p>", format!("{}", YourStruct {
    count: Some(19),
}));
```

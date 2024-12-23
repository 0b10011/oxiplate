# if/else

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

```oxip
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
# Pattern matching with `match` and `case`

```rust
#[derive(Oxiplate)]
#[oxiplate = "page.oxip"]
struct YourStruct {
    value: isize,
}
```

```rust
print!("{}", YourStruct {
    value: -19
});
```

```oxip:page.oxip
{% match value %}
{%- case ..0 -%}
    Less than zero
{%- case 0 -%}
    Zero
{%- case .. -%}
    Greater than zero
{%- endmatch %}
```

```page.txt
Less than zero
```

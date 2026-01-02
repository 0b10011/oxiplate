# Setting local variables with `let`

```rust
#[derive(Oxiplate)]
#[oxiplate = "page.html.oxip"]
struct YourStruct {
    name: &'static str,
    company: &'static str,
}
```

```rust
print!("{}", YourStruct {
    name: "Felix",
    company: "ABC Shipping",
});
```

```html:page.html.oxip
{% let display_name = name ~ " (" ~ company ~ ")" -%}
<h1 title="{{ attr: display_name }}">{{ display_name }}</h1>
```

```html
<h1 title="Felix (ABC Shipping)">Felix (ABC Shipping)</h1>
```

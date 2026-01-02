# Including contents of a template with `include`

```rust
#[derive(Oxiplate)]
#[oxiplate = "template.html.oxip"]
struct YourStruct {
    menu_links: [(&'static str, &'static str)],
    title: &'static str,
}
```

```rust
print!("{}", YourStruct {
    menu_links: [
        ("/", "Home"),
        ("/about/", "About"),
    ],
    title: "Oxiplate",
});
```

```html:template.html.oxip
<!DOCTYPE html>
<nav>{% include "menu.html.oxip" %}</nav>
<main>
    <h1>{{ title }}</h1>
    ...
</main>
```

```html:menu.html.oxip
<ul>
    {%- for (href, text) in menu_links -%}
        <li><a href="{{ attr: link.href }}">{{ link.text }}</a>
    {%- endfor -%}
</ul>
```

```html
<!DOCTYPE html>
<nav><ul><li><a href="/">Home</a><a href="/about/">About</a></ul></nav>
<main>
    <h1>Oxiplate</h1>
    ...
</main>
```

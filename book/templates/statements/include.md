# Including contents of a template with `include`

```rust
# extern crate oxiplate;
#
use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate = "template.html.oxip"]
struct YourStruct {
    menu_links: [(&'static str, &'static str); 2],
    title: &'static str,
}

assert_eq!(
    YourStruct {
        menu_links: [
            ("/", "Home"),
            ("/about/", "About"),
        ],
        title: "Oxiplate",
    }.render()?,
    r#"<!DOCTYPE html>
<nav><ul><li><a href="/">Home</a><li><a href="/about/">About</a></ul></nav>
<main>
    <h1>Oxiplate</h1>
    ...
</main>
"#,
);
#
# Ok::<(), ::core::fmt::Error>(())
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
        <li><a href="{{ attr: href }}">{{ text }}</a>
    {%- endfor -%}
</ul>
```

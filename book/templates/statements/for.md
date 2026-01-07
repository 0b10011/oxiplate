# Iterating with `for` and `else`

```html:for.html.oxip
<ul>
  {% for name in names %}
    <li>{{ name }}
  {% else %}
    <li><em>No names found</em>
  {% endfor %}
</ul>{-}
```

```rust
# extern crate oxiplate;
#
use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate = "for.html.oxip"]
struct YourStruct {
    names: Vec<&'static str>,
}

assert_eq!(
    YourStruct {
        names: vec!["Jasmine", "Malachi", "Imogen"],
    }.render()?,
    r#"<ul>
    <li>Jasmine
    <li>Malachi
    <li>Imogen
</ul>"#,
);

assert_eq!(
  YourStruct {
    names: vec![],
  }.render()?,
  r#"<ul>
    <li><em>No names found</em>
</ul>"#
);
#
# Ok::<(), ::core::fmt::Error>(())
```

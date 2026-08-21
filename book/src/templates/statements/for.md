# Iterating with `for` and `else`

Looping through iterators is straightforward,
and the `loop` filter adds helpful index information.

```html:for.html.oxip
<ul>
  {%- for (loop, name) in &names | loop %}
    <li>
      {{- loop.index1 }} ({{ loop.index0 }}): {{ name }}
      {%- if loop.is_first %} (first){% endif %}
      {%- if loop.is_last %} (last){% endif %}
  {%- else %}
    <li><em>No names found</em>
  {%- endfor %}
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
    <li>1 (0): Jasmine (first)
    <li>2 (1): Malachi
    <li>3 (2): Imogen (last)
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

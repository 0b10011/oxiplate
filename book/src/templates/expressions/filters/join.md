# `join` Filter

`join` can render a list of strings with a separator between each:

```rust
# extern crate oxiplate;
#
# use oxiplate::prelude::*;
#
# #[derive(Oxiplate)]
# #[oxiplate_inline(r#"{{ raw: &values | join(", ") }}"#)]
struct Data<'a> {
    values: Vec<&'a str>,
}
#
# assert_eq!(
#     "red, green, blue",
#     Data { values: vec!["red", "green", "blue"] }.render()?,
# );
#
# Ok::<(), ::core::fmt::Error>(())
```

```oxip
{{ raw: &values | join(", ") }}
```

> red, green, blue

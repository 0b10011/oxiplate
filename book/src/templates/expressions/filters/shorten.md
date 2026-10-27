# `shorten` Filter

`shorten` can limit a string to `max_length` grapheme clusters,
ending with `…` when the string is too long:

```rust
# extern crate oxiplate;
#
# use oxiplate::prelude::*;
#
# #[derive(Oxiplate)]
# #[oxiplate_inline(r#"{{ raw: >title | shorten(12) }}"#)]
struct Data<'a> {
    title: &'a str,
}
#
# assert_eq!(
#     "Hello…",
#     Data { title: "Hello wonderful world" }.render()?,
# );
#
# Ok::<(), ::core::fmt::Error>(())
```

```oxip
{{ raw: >title | shorten(12) }}
```

> Hello…

Length is counted in grapheme clusters,
so characters made up of multiple code points
(like `é` written as `e` followed by a combining accent,
or an emoji sequence like 👨‍👩‍👧)
count as a single character.
The `…` counts toward `max_length`,
and a `max_length` of `0` returns an empty string.

Words are not split
unless the first word alone is longer than `max_length`,
in which case it is cut in the middle
rather than leaving the output empty.

This filter requires the `filter-shorten` feature:

```bash
cargo add oxiplate --features filter-shorten
```

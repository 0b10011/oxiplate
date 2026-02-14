# Filters

<!-- ANCHOR: intro -->
Filters modify expressions that precede them:

```oxip
{{ "foo" | upper }}
```

> FOO
<!-- ANCHOR_END: intro -->

Behind the scenes,
filters are functions
in the `filters_for_oxiplate` module
located in the current scope
that are passed the result of the expression
as the first argument.
Additional arguments can be passed to the filter directly:

```oxip
{{ "hello world" | replace("hello", "goodbye") }}
```

> goodbye world

## Built-in filters

Oxiplate has a number of filters declared in the
[`filters` module](https://github.com/0b10011/oxiplate/blob/main/oxiplate/src/filters/mod.rs).

To use these on their own without any additional filters,
import `::oxiplate::filters as filters_for_oxiplate`.
Or use `::oxiplate::prelude::*` which does the same,
along with importing the `Oxiplate` macro and `Render` trait.

Alternatively, a custom set of filters can be built once:

```rust:all built-in filters plus additional
# extern crate oxiplate;
#
mod filters_for_oxiplate {
    use ::oxiplate::filters::*;

    // Additional filters here
}
```

```rust:subset of built-in filters plus additional
# extern crate oxiplate;
#
mod filters_for_oxiplate {
    use ::oxiplate::filters::{lower, upper};

    // Additional filters here
}
```

And then used throughout the package:

```rust
# extern crate oxiplate;
#
# mod filters_for_oxiplate {}
#
# fn main() {
use ::oxiplate::{Oxiplate, Render};
use crate::filters_for_oxiplate;

// Template structs and functions to call `render()`
# }
```

### `CowStr` filters

`CowStr` is generated via the [cow prefix](./index.html#cow-prefix-for-more-efficient-string-conversion).

| Filter | Description |
|-|-|
| `lower` | Converts to lowercase |
| `trim` | Trims leading and trailing whitespace |
| `trim_end` | Trims trailing whitespace |
| `trim_start` | Trims leading whitespace |
| `upper` | Converts to uppercase |

### `Option` filters

| Filter | Arguments | Description |
|-|-|-|
| `default` | `default_value` | Unwraps `Some` value or uses `default_value` |

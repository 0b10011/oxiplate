# Applying filters to an entire block

Filters are normally reserved for [writs](../writs/index.html),
but occasionally it can be helpful to apply one or more filters
to an entire block of content all at once.

The escaper is specified first on the filter
and run last,
just like with writs.
To prevent double escaping or confusion,
writs cannot specify an escaper themselves
within a filter block.

Behind the scenes,
the entire block is converted into a `CowStr`,
the filters are applied to it left to right,
and finally it is escaped as it is being output.

```html:filter.html.oxip
{{#include ../../../lib/templates/filter.html.oxip}}
```

```rust
# extern crate oxiplate;
#
use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate = "filter.html.oxip"]
struct YourStruct {
    name: &'static str,
}

assert_eq!(
    YourStruct {
        name: "Jasmine & Malachi",
    }.render()?,
    "hello jasmine &amp; malachi!",
);
#
# Ok::<(), ::core::fmt::Error>(())
```

Filters can be chained just like in writs:

```html:filter.html.oxip
{{#include ../../../lib/templates/filter-multiple.html.oxip}}
```

```rust
# extern crate oxiplate;
#
# use oxiplate::prelude::*;
# 
# #[derive(Oxiplate)]
# #[oxiplate = "filter-multiple.html.oxip"]
# struct YourStruct {
#     name: &'static str,
# }
# 
assert_eq!(
    YourStruct {
        name: "Jasmine & Malachi",
    }.render()?,
    "HELLO JASMINE &amp; MALACHI!",
);
#
# Ok::<(), ::core::fmt::Error>(())
```

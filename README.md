# Usage

## External code

```rust
use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[path = "/docs/templates/external.html.oxip"]
struct SomeStruct {
    // Does not need to be `&'static str`,
    // but does need to implement `std::fmt::Display`.
    title: &'static str,
    message: &'static str,
}

fn main() {
    let template = SomeStruct {
        title: "Oxiplate Example",
        message: "Hello world!",
    };

    assert_eq!(
        format!("{}", template),
        "<h1>Oxiplate Example</h1>\n<p>Hello world!</p>\n"
    );
}
```

## Inline code

```rust
use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[code = "<h1>{{ title }}</h1>\n<p>{{ message }}</p>\n"]
struct SomeStruct {
    // Does not need to be `&'static str`,
    // but does need to implement `std::fmt::Display`.
    title: &'static str,
    message: &'static str,
}

fn main() {
    let template = SomeStruct {
        title: "Oxiplate Example",
        message: "Hello world!",
    };

    assert_eq!(
        format!("{}", template),
        "<h1>Oxiplate Example</h1>\n<p>Hello world!</p>\n"
    );
}
```

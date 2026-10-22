use oxiplate_derive::Oxiplate;

struct Foo;

trait Bar {
    type Baz;
}

impl Bar for Foo {
    type Baz = u64;
}

// `Baz` after `<Foo as Bar>::` intentionally skipped to cause error.
#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ Into::<<Foo as Bar>::>::into(unsigned_16) + unsigned_64 }}"#)]
struct Data {
    unsigned_16: u16,
    unsigned_64: u64,
}

fn main() {
    assert_eq!(
        format!(
            "{}",
            Data {
                unsigned_16: 10_u16,
                unsigned_64: 9_u64
            }
        ),
        "19"
    );
}

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("")]
enum Data {
    Foo,
    Bar,
}

fn main() {}

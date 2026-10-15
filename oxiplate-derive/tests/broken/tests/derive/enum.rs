use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("hello")]
enum Data {
    Foo,
    Bar,
}

fn main() {
    print!("{}", Data);
}

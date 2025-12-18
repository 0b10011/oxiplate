use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[rustfmt::skip]
#[oxiplate_inline("hello world \
    {{")]
struct Data {}

fn main() {
    let data = Data {};

    print!("{}", data);
}

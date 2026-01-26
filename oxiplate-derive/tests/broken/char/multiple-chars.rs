use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r"{{ 'hi' }}")]
struct Data {}

fn main() {
    let data = Data {};

    print!("{}", data);
}

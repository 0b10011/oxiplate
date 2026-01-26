use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{% continue }}")]
struct Data {}

fn main() {
    print!("{}", Data {});
}

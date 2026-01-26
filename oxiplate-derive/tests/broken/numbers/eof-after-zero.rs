use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{{ 0")]
struct Data {}

fn main() {
    let data = Data {};

    print!("{}", data);
}

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{{ 0Not_Supported }}")]
struct Data {}

fn main() {
    let data = Data {};

    print!("{}", data);
}

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{{ 0not_supported }}")]
struct Data {}

fn main() {
    let data = Data {};

    print!("{}", data);
}

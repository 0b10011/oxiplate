use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{{ 19e }}")]
struct Data {}

fn main() {
    let data = Data {};

    print!("{}", data);
}

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{{ 0o }}")]
struct Data {}

fn main() {
    let data = Data {};

    print!("{}", data);
}

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{{ 0q19 }}")]
struct Data;

fn main() {
    print!("{}", Data);
}

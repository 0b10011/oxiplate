use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{{ maybe }}")]
struct Data;

fn main() {
    print!("{}", Data);
}

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{{ ''1' }}")]
struct Data;

fn main() {
    print!("{}", Data);
}

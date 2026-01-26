use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{{ 'hi' }}")]
struct Data;

fn main() {
    print!("{}", Data);
}

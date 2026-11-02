use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{% set foo %}")]
struct Data;

fn main() {
    print!("{Data}");
}

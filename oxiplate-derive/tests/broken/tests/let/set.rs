use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{% set %}")]
struct Data;

fn main() {
    print!("{Data}");
}

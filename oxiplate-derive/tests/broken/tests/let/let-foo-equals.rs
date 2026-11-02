use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{% let foo = %}")]
struct Data;

fn main() {
    print!("{Data}");
}

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{% let %}")]
struct Data;

fn main() {
    print!("{Data}");
}

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{% plaintext Hello world!")]
struct Data;

fn main() {
    print!("{Data}");
}

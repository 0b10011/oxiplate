use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "\u{1}\u{12}\u{123}\u{5432}{{ foo }}\u{1}\u{12}\u{123}\u{5432}"]
struct Unicode {}

fn main() {
    let data = Unicode {};

    print!("{}", data);
}

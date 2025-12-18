use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(19)]
struct Inline;

#[derive(Oxiplate)]
#[oxiplate = 19]
struct External;

fn main() {
    print!("{}", Inline);
    print!("{}", External);
}

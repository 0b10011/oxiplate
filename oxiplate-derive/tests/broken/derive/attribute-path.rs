use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline]
struct Inline;

#[derive(Oxiplate)]
#[oxiplate]
struct External;

fn main() {
    print!("{}", Inline);
    print!("{}", External);
}
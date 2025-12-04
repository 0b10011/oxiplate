use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(html: format!("oops"))]
struct Inline;

#[derive(Oxiplate)]
#[oxiplate = format!("oops")]
struct External;

fn main() {
    print!("{}", Inline);
    print!("{}", External);
}

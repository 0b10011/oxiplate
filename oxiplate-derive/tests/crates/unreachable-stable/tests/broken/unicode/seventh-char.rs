use oxiplate_derive::Oxiplate;

// `/` instead of `\` used to reach otherwise unreachable branches.
#[derive(Oxiplate)]
#[oxiplate_inline("/u{1989}y")]
struct Data;

fn main() {
    print!("{}", Data);
}

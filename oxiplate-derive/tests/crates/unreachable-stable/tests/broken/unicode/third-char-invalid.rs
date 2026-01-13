use oxiplate_derive::Oxiplate;

// `/` instead of `\` used to reach otherwise unreachable branches.
#[derive(Oxiplate)]
#[oxiplate_inline("/u{1g")]
struct Data;

fn main() {
    print!("{}", Data);
}

use oxiplate_derive::Oxiplate;

// `/` instead of `\` used to reach otherwise unreachable branches.
// `f00f00` specifically causes 3 additional hex characters to be appended.
#[derive(Oxiplate)]
#[oxiplate_inline("/u{f00f00}")]
struct Data;

fn main() {
    print!("{}", Data);
}

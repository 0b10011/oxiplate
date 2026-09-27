use oxiplate_derive::Oxiplate;

// `/` instead of `\` used to reach otherwise unreachable branches.
// `110000` is first out-of-range value.
#[derive(Oxiplate)]
#[oxiplate_inline("/u{110000}")]
struct Data;

fn main() {
    print!("{}", Data);
}

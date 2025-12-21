use oxiplate_derive::Oxiplate;

// `/` instead of `\` used to reach otherwise unreachable branches.
#[derive(Oxiplate)]
#[oxiplate_inline("/x7")]
struct Data;

fn main() {
    print!("{}", Data);
}

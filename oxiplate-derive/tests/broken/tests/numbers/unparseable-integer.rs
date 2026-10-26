use oxiplate_derive::Oxiplate;

// When `4 ` is encountered after any digit
// with `_unreachable` turned on,
// those two characters are consumed
// and regular integer parsing continues after that.
#[derive(Oxiplate)]
#[oxiplate_inline("{{ 14 19 }}")]
struct Data;

fn main() {
    print!("{}", Data);
}

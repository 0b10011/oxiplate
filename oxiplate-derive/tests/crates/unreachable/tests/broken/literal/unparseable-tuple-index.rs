use oxiplate_derive::Oxiplate;

// When `4 ` is encountered after any digit
// with `_unreachable` turned on,
// those two characters are consumed
// and regular integer parsing continues after that.
#[derive(Oxiplate)]
#[oxiplate_inline("{{ value.14 19 }}")]
struct Data {
    value: (usize, usize),
}

fn main() {
    print!("{}", Data { value: (19, 89) });
}

use oxiplate_derive::Oxiplate;

// Prefix operator should be the same one behind the `_unreachable` feature.
#[derive(Oxiplate)]
#[oxiplate_inline("{{ @a }}")]
struct Data {
    a: u8,
}

fn main() {
    let data = Data { a: 19 };

    print!("{}", data);
}

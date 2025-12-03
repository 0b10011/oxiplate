use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{{ a + }}")]
struct Data {
    a: u8,
}

fn main() {
    let data = Data { a: 19 };

    print!("{}", data);
}

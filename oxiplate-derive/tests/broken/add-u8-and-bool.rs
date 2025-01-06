use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "{{ a + b }}"]
struct Data {
    a: u8,
    b: bool,
}

fn main() {
    let data = Data { a: 4, b: false };

    print!("{}", data);
}

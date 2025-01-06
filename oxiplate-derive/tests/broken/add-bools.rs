use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "{{ a + b }}"]
struct Data {
    a: bool,
    b: bool,
}

fn main() {
    let data = Data { a: true, b: false };

    print!("{}", data);
}

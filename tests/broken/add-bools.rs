use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "{{ a + b }}"]
struct Data {
    a: bool,
    b: bool,
}

fn main() {
    let data = Data {
        a: true,
        b: false,
    };

    print!("{}", data);
}

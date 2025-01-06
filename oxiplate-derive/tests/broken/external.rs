use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "external.html.oxip"]
struct Data {
    title: &'static str,
}

fn main() {
    let data = Data {
        title: "Hello world",
    };

    print!("{}", data);
}

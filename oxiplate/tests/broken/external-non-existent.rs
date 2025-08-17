use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate = "external-non-existent.html.oxip"]
struct Data {
    title: &'static str,
}

fn main() {
    let data = Data {
        title: "Hello world",
    };

    assert_eq!("Hello World", data.render().unwrap());
}

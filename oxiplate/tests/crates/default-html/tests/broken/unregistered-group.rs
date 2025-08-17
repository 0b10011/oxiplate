use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate = "page.json.oxip"]
struct Data<'a> {
    message: &'a str,
}

fn main() {
    Data {
        message: "Hello world"
    }.render().unwrap();
}

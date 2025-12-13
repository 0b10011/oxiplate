use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate_inline("{{ text: data }}")]
struct Data {
    value: &'static str,
}

fn main() {
    assert_eq!(
        Data { value: "Hello world!" }.render().unwrap(),
        "Hello world!"
    );
}

use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate_inline("{% default_escaper_group %}{{ title }}")]
struct Data {
    title: &'static str,
}

fn main() {
    assert_eq!("Hello World", Data { title: "Hello World" }.render().unwrap());
}
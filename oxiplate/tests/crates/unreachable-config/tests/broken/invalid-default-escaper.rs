use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% default_escaper_group html %}{{ message }}"#)]
struct Data {
    message: &'static str,
}

fn main() {
    Data {
        message: "hello world!",
    }
    .render()
    .unwrap();
}

use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate_inline(html "<!DOCTYPE html><title>{{ title }}</title>")]
struct DefaultAfterContent {
    title: &'static str,
}

pub fn main() {
    DefaultAfterContent { title: "Hello world" }.render().unwrap();
}

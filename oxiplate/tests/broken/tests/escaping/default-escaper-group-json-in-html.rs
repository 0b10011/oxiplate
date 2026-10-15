use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate_inline(html: "{% default_escaper_group json %}{{ title }}")]
struct JsonInHtml {
    title: &'static str,
}

pub fn main() {
    JsonInHtml {
        title: "<!DOCTYPE html>Hello world",
    }
    .render()
    .unwrap();
}

use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate_inline("<!DOCTYPE html>{% replace_escaper_group html %}{{ title }}")]
struct ReplaceAfterContent {
    title: &'static str,
}

pub fn main() {
    ReplaceAfterContent {
        title: "<!DOCTYPE html>Hello world",
    }
    .render()
    .unwrap();
}

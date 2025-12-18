use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate_inline("<!DOCTYPE html>{% default_escaper_group html %}{{ title }}")]
struct DefaultAfterContent {
    title: &'static str,
}

pub fn main() {
    DefaultAfterContent {
        title: "<!DOCTYPE html>Hello world",
    }
    .render()
    .unwrap();
}

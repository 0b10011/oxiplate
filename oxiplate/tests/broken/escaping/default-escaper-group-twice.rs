use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate_inline("{% default_escaper_group html %}{% default_escaper_group html %}{{ title }}")]
struct DefaultTwice {
    title: &'static str,
}

pub fn main() {
    DefaultTwice { title: "<!DOCTYPE html>Hello world" }.render().unwrap();
}

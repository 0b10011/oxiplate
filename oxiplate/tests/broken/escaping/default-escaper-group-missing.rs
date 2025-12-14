use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate_inline("{% default_escaper_group escaper_that_does_not_exist %}{{ title }}")]
struct MissingEscaperGroup {
    title: &'static str,
}

pub fn main() {
    MissingEscaperGroup { title: "<!DOCTYPE html>Hello world" }.render().unwrap();
}

use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate_inline(
    // Default and specified escaper are both required
    // to ensure error messages are merged in the final output
    // and both code paths are run (for coverage).
    "{% default_escaper_group html %}{% default_escaper_group html %}{{ title }}{{ text: title }}"
)]
struct DefaultTwice {
    title: &'static str,
}

pub fn main() {
    DefaultTwice {
        title: "<!DOCTYPE html>Hello world",
    }
    .render()
    .unwrap();
}

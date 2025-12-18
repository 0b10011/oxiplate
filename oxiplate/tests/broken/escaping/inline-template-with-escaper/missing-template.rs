use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate_inline(html:)]
struct DefaultAfterContent {
    title: &'static str,
}

pub fn main() {
    DefaultAfterContent {
        title: "Hello world",
    }
    .render()
    .unwrap();
}

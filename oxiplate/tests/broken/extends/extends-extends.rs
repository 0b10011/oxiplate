use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate_inline(html: r#"{% extends "extends-wrapper.html.oxip" %}{% extends "extends-wrapper.html.oxip" %}"#)]
struct Data {
    title: &'static str,
}

fn main() {
    let data = Data {
        title: "Double extends",
    };

    print!("{}", data.render().unwrap());
}

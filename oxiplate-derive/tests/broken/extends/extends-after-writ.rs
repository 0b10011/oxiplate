use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ title }}{% extends "multiple-blocks.html.oxip" %}"#)]
struct Writ {
    title: &'static str,
}

fn main() {
    print!("{}", Writ {
        title: "Double extends",
    });
}

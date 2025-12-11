use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% extends "extends.html.oxip" %}{% block content %}"#)]
struct Data {
    title: &'static str,
    message: &'static str,
}

fn main() {
    print!("{}", Data { title: "Title", message: "Message" });
}

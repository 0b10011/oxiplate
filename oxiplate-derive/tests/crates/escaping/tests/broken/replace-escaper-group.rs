use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% replace_escaper_group foo %}{{ "hello world" }}"#)]
struct Data;

fn main() {
    print!("{Data}");
}

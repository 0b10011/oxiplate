use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% default_escaper_group foo %}{{ "hello world" }}"#)]
struct Data;

fn main() {
    print!("{Data}");
}

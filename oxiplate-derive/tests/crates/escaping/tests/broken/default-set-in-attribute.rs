use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(foo: r#"{{ "hello world" }}"#)]
struct Data;

fn main() {
    print!("{Data}");
}

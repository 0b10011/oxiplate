use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate_inline(html: "Hello world")]
struct Data;

fn main() {
    assert_eq!(Data {}.render().unwrap(), r#"Hello world"#);
}

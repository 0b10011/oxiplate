use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% filter text | lower %}{% endfilter %}{{ "b" }}"#)]
struct Data;

fn main() {
    print!("{}", Data);
}

use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate_inline("{% filter | lower %}{% endfilter %}")]
struct Data;

fn main() {
    print!("{Data}");
}

use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate_inline("{% filter %}{% endfilter %}")]
struct Data;

fn main() {
    print!("{Data}");
}

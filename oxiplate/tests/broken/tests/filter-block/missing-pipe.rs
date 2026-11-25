use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate_inline("{% filter raw lower %}{% endfilter %}")]
struct Data;

fn main() {
    print!("{Data}");
}

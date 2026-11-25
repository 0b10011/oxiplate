use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate_inline("{% filter raw | %}{% endfilter %}")]
struct Data;

fn main() {
    print!("{Data}");
}

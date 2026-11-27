use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{% endplaintext %}")]
struct Data;

fn main() {
    print!("{Data}");
}

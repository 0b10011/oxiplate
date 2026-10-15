use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{% match 19 %}{% case _ %}")]
struct Data;

fn main() {
    print!("{Data}");
}

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{% match 19 %}{%_ case _ %}{% endmatch %}")]
struct Data {}

fn main() {
    let data = Data {};

    print!("{}", data);
}

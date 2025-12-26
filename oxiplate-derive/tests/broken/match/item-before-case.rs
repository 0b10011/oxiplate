use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    "{% match 19 %}Value is 19{% case 19 %}Value is something else{% case _ %}{% endmatch %}"
)]
struct Data {}

fn main() {
    let data = Data {};

    print!("{}", data);
}

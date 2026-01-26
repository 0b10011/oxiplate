use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{{ =value }}")]
struct Data {
    value: &'static str,
}

fn main() {
    print!(
        "{}",
        Data {
            value: "Hello World!"
        }
    );
}

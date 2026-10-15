use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{%- for number in numbers -%}{{ number }}{% endif %}")]
struct Data {
    numbers: Vec<i32>,
}

fn main() {
    print!(
        "{}",
        Data {
            numbers: vec![19, 89],
        }
    );
}

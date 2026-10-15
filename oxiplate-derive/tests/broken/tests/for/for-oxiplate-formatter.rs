use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{%- for oxiplate_formatter in numbers -%}{{ oxiplate_formatter }}{% endfor %}")]
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

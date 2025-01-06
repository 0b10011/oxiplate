use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "{%- for number numbers -%}{{ number }} + {{ number_to_add }} = {{ numb + number_to_add }}{% endfor %}"]
struct Data {
    numbers: Vec<i32>,
    number_to_add: i32,
}

fn main() {
    print!(
        "{}",
        Data {
            numbers: vec![19, 89],
            number_to_add: 4,
        }
    );
}

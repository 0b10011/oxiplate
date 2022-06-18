use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "{%- for number in numbers -%}{{ number }} + 1 = {{ number + 1 }}"]
struct Data {
    numbers: Vec<i32>,
}

fn main() {
    print!("{}", Data {
        numbers: vec![19, 89],
    });
}

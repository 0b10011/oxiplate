use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "{%- for in numbers -%}No ident present{% endfor %}"]
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

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{% for number in numbers %}
    {{ number }}
{% else %}
    foo
{% else %}
    hello
{% else %}
    world
{% endfor %}
{% else %}"
)]
struct Data {
    numbers: Vec<u8>,
}

fn main() {
    print!(
        "{}",
        Data {
            numbers: vec![19, 89]
        }
    );
}

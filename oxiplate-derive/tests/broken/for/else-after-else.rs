use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("
{% for number in numbers %}
    {{ number }}
{% else %}
    foo
{% else %}
    hello
{% else %}
    world
{% endfor %}
{% else %}")]
struct Data {
    foo: bool,
}

fn main() {
    print!("{}", Data { foo: true });
}

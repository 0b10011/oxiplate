use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{% if foo %}
    foo
{% else %}
    bar
{% else %}
    hello
{% else %}
    world
{% elseif foo %}
    hello
{% elseif foo %}
    world
{% endif %}
{% else %}"
)]
struct Data {
    foo: bool,
}

fn main() {
    print!("{}", Data { foo: true });
}

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{% if foo %}
    {% continue %}
{% elseif !foo %}
    {% continue %}
{% else %}
    {% continue %}
{% endif %}"
)]
struct Data {
    foo: bool,
}

fn main() {
    print!("{}", Data { foo: true });
}

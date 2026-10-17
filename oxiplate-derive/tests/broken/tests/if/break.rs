use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{% if foo %}
    {% break %}
{% elseif !foo %}
    {% break %}
{% else %}
    {% break %}
{% endif %}"
)]
struct Data {
    foo: bool,
}

fn main() {
    print!("{}", Data { foo: true });
}

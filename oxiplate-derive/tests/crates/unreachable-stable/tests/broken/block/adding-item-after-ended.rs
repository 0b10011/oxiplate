use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% block foo %}{% endblock %}{{ "b" }}"#)]
struct Data;

fn main() {
    print!("{}", Data);
}

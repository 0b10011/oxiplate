use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% block foo %}{{ "a" }}{% endblock %}"#)]
struct Data;

fn main() {
    print!("{}", Data);
}

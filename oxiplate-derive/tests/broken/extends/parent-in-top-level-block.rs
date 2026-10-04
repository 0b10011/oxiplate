use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% block content %}foo{% parent %}bar{% endblock %}"#)]
struct Data;

fn main() {
    print!("{Data}");
}

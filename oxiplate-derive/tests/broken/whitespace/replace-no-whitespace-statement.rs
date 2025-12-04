use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"
{% block a _%}{%_ endblock %}
{% block b _%}{% endblock %}
{% block c %}{%_ endblock %}
"#)]
struct Data {}

fn main() {
    print!("{}", Data {});
}

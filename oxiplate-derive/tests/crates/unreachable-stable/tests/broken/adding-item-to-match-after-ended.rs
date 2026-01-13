use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% match 19 %}{% case _ %}{% endmatch %}{{ "b" }}"#)]
struct Data;

fn main() {
    print!("{}", Data);
}

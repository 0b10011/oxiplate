use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"
{# "replace" _#}{#_ "replace" #}
{# "replace" _#}{# "preserve" #}
{# "preserve" #}{#_ "replace" #}
"#)]
struct Data {}

fn main() {
    print!("{}", Data {});
}

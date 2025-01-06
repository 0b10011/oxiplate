use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = r#####"
{{- ##"hello world"# -}}
 "#####]
struct Data {}

fn main() {
    print!("{}", Data {});
}

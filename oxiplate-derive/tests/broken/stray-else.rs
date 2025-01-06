use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "{{ action }}{%- else -%}"]
struct Data {
    action: &'static str,
}

fn main() {
    let data = Data {
        action: "do something",
    };

    print!("{}", data);
}

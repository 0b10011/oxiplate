use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "{{ action }}{%- else -%}"]
struct Data {
    action: &'static str,
}

fn main() {
    let data = Data {
        action: "do something",
    };

    print!("{}", data);
}

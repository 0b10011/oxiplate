use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "{%- elseif should_do -%}{{ action }}"]
struct Data {
    should_do: bool,
    action: &'static str,
}

fn main() {
    let data = Data {
        should_do: true,
        action: "do something",
    };

    print!("{}", data);
}

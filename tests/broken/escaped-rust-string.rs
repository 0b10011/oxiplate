use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "\"{{ foo }}\""]
struct Data {}

fn main() {
    let data = Data {};

    print!("{}", data);
}

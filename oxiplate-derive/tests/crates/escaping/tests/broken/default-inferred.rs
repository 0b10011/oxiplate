use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "page.foo.oxip"]
struct Data {
    message: &'static str,
}

fn main() {
    print!(
        "{}",
        Data {
            message: "hello world!"
        }
    );
}

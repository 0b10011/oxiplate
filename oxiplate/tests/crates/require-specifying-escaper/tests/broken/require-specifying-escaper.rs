use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "{{ message }}"]
struct Data<'a> {
    message: &'a str,
}

fn main() {
    let data = Data {
        message: "Hello world!"
    };

    assert_eq!(
        format!("{}", data),
        "Hello world!"
    );
}

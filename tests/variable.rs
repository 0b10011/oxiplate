use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxi_code = "{{ message }}"]
struct Data {
    message: &'static str,
}

#[test]
fn variable() {
    let data = Data {
        message: "Hello world!",
    };

    assert_eq!(format!("{}", data), "Hello world!");
}

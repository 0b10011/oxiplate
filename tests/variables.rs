use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[code = "{{ title }} / {{ message }}"]
struct Data {
    title: &'static str,
    message: &'static str,
}

#[test]
fn variables() {
    let data = Data {
        title: "Foo Bar",
        message: "Hello world!",
    };

    assert_eq!(format!("{}", data), "Foo Bar / Hello world!");
}

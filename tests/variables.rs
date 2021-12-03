use rustem::Rustem;

#[derive(Rustem)]
#[template(code = "{} / {}")]
struct Data {
    title: &'static str,
    message: &'static str,
}

#[test]
fn variable() {
    let data = Data {
        title: "Foo Bar",
        message: "Hello world!",
    };

    assert_eq!(format!("{}", data), "Foo Bar / Hello world!");
}

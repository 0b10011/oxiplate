use rustem::Rustem;

#[derive(Rustem)]
#[template(code = "{}")]
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

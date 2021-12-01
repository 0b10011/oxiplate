use rustem::rustem;

rustem! {
    "{{ message }}"
    struct Data {
        message: &'static str,
    }
}

#[test]
fn variable() {
    let data = Data {
        message: "Hello world!",
    };

    assert_eq!(format!("{}", data), "Hello world!");
}

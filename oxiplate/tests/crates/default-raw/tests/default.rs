use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate_inline("{{ value }}")]
struct Data<'a> {
    value: &'a str,
}

#[test]
fn variable() {
    let data = Data {
        value: "Hello world!",
    };

    assert_eq!(data.render().unwrap(), "Hello world!");
}

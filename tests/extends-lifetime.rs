use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = include_str!("extends.html.oxip")]
struct AbsoluteData<'a> {
    title: &'a str,
    message: &'a str,
}

#[test]
fn absolute() {
    let data = AbsoluteData {
        title: "Oxiplate Example",
        message: "Hello world!",
    };

    assert_eq!(
        format!("{}", data),
        "<!DOCTYPE html>\n<title>Oxiplate Example</title>\n<h1>Oxiplate Example</h1>\n  <p>Hello world!</p>\n"
    );
}

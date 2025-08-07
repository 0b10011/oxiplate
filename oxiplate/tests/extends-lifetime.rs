use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate = "extends-deep.html.oxip"]
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
        data.render().unwrap(),
        "<!DOCTYPE html>\n<title>Oxiplate Example</title>\n<h2>Oxiplate Example</h2>\n  \
         <div>Hello world!</div>\n"
    );
}

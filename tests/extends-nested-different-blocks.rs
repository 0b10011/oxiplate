use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "./extends-nested-different-blocks.html.oxip"]
struct AbsoluteData {
    title: &'static str,
    message: &'static str,
}

#[test]
fn absolute() {
    let data = AbsoluteData {
        title: "Oxiplate Example",
        message: "Hello world!",
    };

    assert_eq!(
        format!("{}", data),
        "<DOCTYPE html>\n<head>\n  <title>Oxiplate Example</title>\n</head>\n<body><main><h1>Oxiplate Example</h1>\n  <p>Hello world!</p></main></body>\n"
    );
}

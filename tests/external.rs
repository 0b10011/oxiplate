use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxi_path = "/tests/external.html.oxip"]
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
        "<h1>Oxiplate Example</h1>\r\n<p>Hello world!</p>\r\n"
    );
}

#[test]
fn absolute_2() {
    let data = AbsoluteData {
        title: "Oxiplate Example #2",
        message: "Goodbye world!",
    };

    assert_eq!(
        format!("{}", data),
        "<h1>Oxiplate Example #2</h1>\r\n<p>Goodbye world!</p>\r\n"
    );
}

// Depends on https://github.com/rust-lang/rust/issues/54725
// #[derive(Oxiplate)]
// #[oxi_path = "./tests/external.html.oxip"]
// struct RelativeData {
//     message: &'static str,
// }

// #[test]
// fn relative() {
//     let data = RelativeData {
//         message: "Hello world!",
//     };
//
//     assert_eq!(format!("{}", data), "Hello world!\r\n");
// }

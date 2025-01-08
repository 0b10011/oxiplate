use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "{{ message }}"]
struct Variable {
    message: &'static str,
}

#[test]
fn variable() {
    let data = Variable {
        message: "Hello world!",
    };

    assert_eq!(format!("{data}"), "Hello world!");
}

#[derive(Oxiplate)]
#[oxiplate_inline = "{{ title }} / {{ message }}"]
struct Variables {
    title: &'static str,
    message: &'static str,
}

#[test]
fn variables() {
    let data = Variables {
        title: "Foo Bar",
        message: "Hello world!",
    };

    assert_eq!(format!("{data}"), "Foo Bar / Hello world!");
}

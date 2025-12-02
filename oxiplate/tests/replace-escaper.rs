use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate = "replace-escaper.html.oxip"]
struct Html {
    name: &'static str,
}

#[test]
fn html() {
    assert_eq!(
        Html { name: r#"foo bar"# }.render().unwrap(),
        r#"
f00 bar
f00 bar
foo b@r
"#
    );
    assert_eq!(
        Html { name: r#"hello"# }.render().unwrap(),
        r#"
hello
hello
hello
"#
    );
}

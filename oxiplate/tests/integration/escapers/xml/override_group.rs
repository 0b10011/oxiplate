use oxiplate::{Oxiplate, Render};

#[test]
fn default() {
    use alloc::format;

    #[derive(Oxiplate)]
    #[oxiplate_inline(xml: r#"{% default_escaper_group xml %}{{ "<hello>" }}"#)]
    struct Data;

    assert_eq!(format!("{Data}"), "&lt;hello&gt;",);
}

#[test]
fn replace() {
    #[derive(Oxiplate)]
    #[oxiplate = "replace-escaper.xml.oxip"]
    struct Xml {
        name: &'static str,
    }

    assert_eq!(
        Xml { name: r#"foo bar"# }.render().unwrap(),
        r#"
f00 bar
f00 bar
foo b@r
"#
    );
    assert_eq!(
        Xml { name: r#"hello"# }.render().unwrap(),
        r#"
hello
hello
hello
"#
    );
}

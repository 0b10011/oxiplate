use oxiplate::{Oxiplate, Render};

#[test]
fn default() {
    use alloc::format;

    #[derive(Oxiplate)]
    #[oxiplate_inline(html: r#"{% default_escaper_group html %}{{ "<hello>" }}"#)]
    struct Data;

    assert_eq!(format!("{Data}"), "&lt;hello>",);
}

#[test]
fn replace() {
    #[derive(Oxiplate)]
    #[oxiplate = "replace-escaper.html.oxip"]
    struct Html {
        name: &'static str,
    }

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

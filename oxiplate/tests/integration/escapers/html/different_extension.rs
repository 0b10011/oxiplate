use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate = "html-with-different-extension.tmpl"]
struct Html {
    name: &'static str,
}

#[test]
fn html() {
    assert_eq!(
        Html {
            name: r#"Hunt "<html>" Mill"#
        }
        .render()
        .unwrap(),
        r#"<!DOCTYPE html>
<p title="Hello Hunt &#34;<html>&#34; Mill">Hello Hunt "&lt;html>" Mill!</p>
<p>Goodbye Hunt "&lt;html>" Mill!</p>
"#
    );
}

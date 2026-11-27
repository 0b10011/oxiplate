use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate = "xml-with-different-extension.tmpl"]
struct Xml {
    name: &'static str,
}

#[test]
fn xml() {
    assert_eq!(
        Xml {
            name: r#"Hunt "<xml>" Mill"#
        }
        .render()
        .unwrap(),
        r#"<?xml version="1.1"?>
<greeting name="Hunt &quot;&lt;xml>&quot; Mill">Hello Hunt "&lt;xml&gt;" Mill!</greeting>
<farewell>Goodbye Hunt "&lt;xml&gt;" Mill!</farewell>
"#
    );
}

use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate_inline(xml: "{% for message in &messages %}\n<p>{{ xml.text: message }}</p>{% endfor %}\n")]
struct Data<'a> {
    messages: Vec<&'a str>,
}

fn main() {
    let data = Data {
        messages: vec![
            "Hello world!",
            "&reg;</p><script>alert('hey');</script><p>&#153;",
        ],
    };

    assert_eq!(
        data.render().unwrap(),
        r#"
<p>Hello world!</p>
<p>&amp;reg;&lt;/p&gt;&lt;script&gt;alert('hey');&lt;/script&gt;&lt;p&gt;&amp;#153;</p>
"#
    );
}

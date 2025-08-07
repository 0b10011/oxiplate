use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate_inline(html: "{% for message in &messages %}\n{{ md.text: message }}\n{% endfor %}")]
struct Data<'a> {
    messages: Vec<&'a str>,
}

#[test]
fn variable() {
    let data = Data {
        messages: vec![
            "Hello world!",
            "&reg;</p><script>alert('hey');</script><p>&#153;",
            "\n\n**oh \t no** ",
        ],
    };

    assert_eq!(
        data.render().unwrap(),
        r"
Hello world\!

\&reg\;\<\/p\>\<script\>alert\(\'hey\'\)\;\<\/script\>\<p\>\&\#153\;

\*\*oh no\*\*
"
    );
}

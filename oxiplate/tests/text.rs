use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(html: "{% for message in &messages %}\n<p>{{ text: message }}</p>{% endfor %}\n")]
struct Data<'a> {
    messages: Vec<&'a str>,
}

#[test]
fn variable() {
    let data = Data {
        messages: vec![
            "Hello world!",
            "&reg;</p><script>alert('hey');</script><p>&#153;",
        ],
    };

    assert_eq!(
        format!("{data}"),
        r"
<p>Hello world!</p>
<p>&amp;reg;&lt;/p>&lt;script>alert('hey');&lt;/script>&lt;p>&amp;#153;</p>
"
    );
}

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{{ messages }} {{ raw: messages }}\n")]
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
        format!("{data}"),
        r#"
<p>Hello world!</p>
<p>&amp;reg;&lt;/p>&lt;script>alert('hey');&lt;/script>&lt;p>&amp;#153;</p>
"#
    );
}

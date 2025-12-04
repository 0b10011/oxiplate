use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate_inline(html: "Message: {{ text }}")]
struct Data {
    text: &'static str,
}

#[test]
fn inline() {
    assert_eq!(
        "Message: 1 &lt; 3",
        Data { text: "1 < 3" }.render().unwrap()
    );
}

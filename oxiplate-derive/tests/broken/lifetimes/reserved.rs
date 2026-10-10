use oxiplate_derive::Oxiplate;

fn function<'a: 'a, 'b: 'b>(left: &'a str, right: &'b str) -> String {
    format!("{left:?} {right:?}")
}

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ function::<'oxiplate_formatter, 'oxiplate_loop_had_item>(left, right) }}"#)]
struct Data<'c, 'd> {
    left: &'c str,
    right: &'d str,
}

fn main() {
    assert_eq!(
        format!(
            "{}",
            Data {
                left: "hello",
                right: "world",
            }
        ),
        r#""hello" "world""#
    );
}

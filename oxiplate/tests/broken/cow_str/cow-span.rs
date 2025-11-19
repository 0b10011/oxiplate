use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ raw: "foo".repeat(> 19) }}"#)]
struct RepeatFoot {}

fn main() {
    assert_eq!(format!("{}", RepeatFoot {}), "world");
}

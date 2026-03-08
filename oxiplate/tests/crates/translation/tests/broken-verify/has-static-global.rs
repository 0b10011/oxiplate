use oxiplate::prelude::*;

#[distributed_slice]
static OXIPLATE_TRANSLATIONS: [TranslationsSignature];

#[derive(Oxiplate)]
#[oxiplate_inline(html: "Hello world")]
struct Data;

fn main() {
    assert_eq!(
        Data {}.render().unwrap(),
        r#"Hello world"#
    );
}

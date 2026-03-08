use oxiplate::prelude::*;

#[distributed_slice]
static OXIPLATE_TRANSLATIONS: [TranslationsSignature];

#[derive(Oxiplate)]
#[oxiplate_inline(html: "Hello world")]
struct Data;

#[test]
fn has_static_global() {
    assert_eq!(Data {}.render().unwrap(), r#"Hello world"#);
    let mut translations = vec![];
    for item_translations in OXIPLATE_TRANSLATIONS {
        translations.extend(item_translations());
    }
    assert_eq!(translations, [("Hello world", "")]);
}

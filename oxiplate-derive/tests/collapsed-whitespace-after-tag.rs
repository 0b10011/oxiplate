#![no_std]

extern crate alloc;

use alloc::format;

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"{# _#}
{% if value %}foo{% endif _%}
{{ "bar" _}}
"#
)]
struct Data {
    value: bool,
}

#[test]
fn adjusted_whitespace() {
    let data = Data { value: true };

    assert_eq!(format!("{data}"), " foo bar ");
}

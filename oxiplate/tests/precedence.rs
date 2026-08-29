#![no_std]

extern crate alloc;

use alloc::format;

use oxiplate::prelude::*;

struct User {
    name: &'static str,
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{{ raw: >" (" ~ (9 + 5 * 2) * 100 + 89 ~ ") " | trim }}
{{ raw: >user.name | upper ~ " is awesome" }}
"#
)]
struct Data {
    user: User,
}

#[test]
fn field() {
    let data = Data {
        user: User { name: "Liv" },
    };

    assert_eq!(
        format!("{data}"),
        "
(1989)
LIV is awesome
"
    );
}

use alloc::format;

use oxiplate::prelude::*;

struct User {
    name: &'static str,
}

impl User {
    fn name(&self) -> &'static str {
        self.name
    }
}

#[test]
fn precedence() {
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

#[test]
fn cow_concat_field() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{{ raw: >"hello " ~ user.name }}"#)]
    struct Data {
        user: User,
    }

    let data = Data {
        user: User { name: "Liv" },
    };

    assert_eq!(format!("{data}"), "hello Liv");
}

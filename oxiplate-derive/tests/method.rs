use oxiplate_derive::Oxiplate;

struct User {
    name: &'static str,
    company: &'static str,
}
impl User {
    pub fn display_name(&self) -> String {
        format!("{} ({})", self.company, self.name)
    }
}

#[derive(Oxiplate)]
#[oxiplate_inline("{{ user.display_name() }}")]
struct Data {
    user: User,
}

#[test]
fn field() {
    let data = Data {
        user: User {
            name: "Kiera",
            company: "Floating Air LLC",
        },
    };

    assert_eq!(format!("{data}"), "Floating Air LLC (Kiera)");
}

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% if user.display_name().contains("i") %}yup!{% endif %}"#)]
struct Argument {
    user: User,
}

#[test]
fn field_with_argument() {
    let data = Argument {
        user: User {
            name: "Kiera",
            company: "Floating Air LLC",
        },
    };

    assert_eq!(format!("{data}"), "yup!");
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"{{ user.display_name().replace("a", "@",) }} {{ user.display_name().replace("a", "@") }}"#
)]
struct Arguments {
    user: User,
}

#[test]
fn field_with_arguments() {
    let data = Arguments {
        user: User {
            name: "Kiera",
            company: "Floating Air LLC",
        },
    };

    assert_eq!(
        format!("{data}"),
        "Flo@ting Air LLC (Kier@) Flo@ting Air LLC (Kier@)"
    );
}

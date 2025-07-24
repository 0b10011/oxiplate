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

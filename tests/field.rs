use oxiplate::Oxiplate;

struct User {
    name: &'static str,
}

#[derive(Oxiplate)]
#[oxiplate_inline = "{{ user.name }}"]
struct Data {
    user: User,
}

#[test]
fn field() {
    let data = Data {
        user: User { name: "Liv" },
    };

    assert_eq!(format!("{}", data), "Liv");
}

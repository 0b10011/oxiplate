use oxiplate::Oxiplate;

struct User {
    name: &'static str,
}

#[derive(Oxiplate)]
#[oxi_code = "{{ user.name }}"]
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

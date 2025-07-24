use oxiplate_derive::Oxiplate;

struct User<'a> {
    name: &'a str,
}

#[derive(Oxiplate)]
#[oxiplate_inline("{{ user.name }}")]
struct Data<'a> {
    user: &'a User<'a>,
}

#[test]
fn field() {
    let name = "Liv";
    let user = User { name };
    let data = Data { user: &user };

    assert_eq!(format!("{data}"), "Liv");
}

use oxiplate::Oxiplate;

struct User<'a> {
    name: &'a str,
}

#[derive(Oxiplate)]
#[oxiplate = "{{ user.name }}"]
struct Data<'a> {
    user: &'a User<'a>,
}

#[test]
fn field() {
    let name = "Liv";
    let user = User { name: &name };
    let data = Data { user: &user };
    // panic!("The problem is the `impl Display for Data {` doesn't specify lifetime like `impl Display for Data<'_>` {`");

    assert_eq!(format!("{}", data), "Liv");
}

use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxi_code = "Hello  \t\n {_} \r\n\t world \n\t {-} \t\n !"]
struct Data {}

#[test]
fn variables() {
    let data = Data {};

    assert_eq!(format!("{}", data), "Hello world!");
}

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "
{{ 19 }}
{{ 10 + 9 }}
{{ 0 }}
{{ 000 }}
{{ 0b10011 }}
{{ 0b0 }}
{{ 0b0000 }}
{{ 0b10011 + 19 }}
{{ 19 + 0b10011 }}"]
struct Data;

#[test]
fn field() {
    let data = Data;

    assert_eq!(
        format!("{data}"),
        "
19
19
0
0
19
0
0
38
38"
    );
}

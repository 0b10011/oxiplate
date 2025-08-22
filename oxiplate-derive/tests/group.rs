use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{{ c * (a + b) }}")]
struct GroupCalc {
    a: usize,
    b: usize,
    c: usize,
}

#[test]
fn group_calc() {
    assert_eq!(format!("{}", GroupCalc { a: 1, b: 2, c: 3 }), "9");
}

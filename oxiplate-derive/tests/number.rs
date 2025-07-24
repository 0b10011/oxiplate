use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{{ 19 }}
{{ 10 + 9 }}
{{ 0 }}
{{ 000 }}
{{ 0b10011 }}
{{ 0b0 }}
{{ 0b0000 }}
{{ 0b10011 + 19 }}
{{ 19 + 0b10011 }}"
)]
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

#[derive(Oxiplate)]
#[oxiplate_inline("{{ 1_234_567 }}")]
struct DecimalNumberSeparators;

#[test]
fn decimal_number_separators() {
    assert_eq!(format!("{DecimalNumberSeparators}"), "1234567");
}

#[derive(Oxiplate)]
#[oxiplate_inline("{{ 0b0001_0011 }}")]
struct BinaryNumberSeparators;

#[test]
fn binary_number_separators() {
    assert_eq!(format!("{BinaryNumberSeparators}"), "19");
}

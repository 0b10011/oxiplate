use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxi_code = "{-}
{{ max }} + {{ min }} = {{ max + min }}
{{ max }} - {{ min }} = {{ max - min }}
{{ max }} * {{ min }} = {{ max * min }}
{{ max }} / {{ min }} = {{ max / min }}
{{ max }} % {{ min }} = {{ max % min }}"]
struct Math {
    min: i16,
    max: i16,
}

#[test]
fn test_math() {
    let data = Math {
        min: 19,
        max: 89,
    };

    assert_eq!(format!("{}", data), "89 + 19 = 108
89 - 19 = 70
89 * 19 = 1691
89 / 19 = 4
89 % 19 = 13");
}

#[derive(Oxiplate)]
#[oxi_code = "{-}
{{ max }} == {{ min }} = {{ max == min }}
{{ max }} != {{ min }} = {{ max != min }}
{{ max }} > {{ min }} = {{ max > min }}
{{ max }} < {{ min }} = {{ max < min }}
{{ max }} >= {{ min }} = {{ max >= min }}
{{ max }} <= {{ min }} = {{ max <= min }}"]
struct Comparisons {
    min: i16,
    max: i16,
}

#[test]
fn test_comparisons() {
    let data = Comparisons {
        min: 19,
        max: 89,
    };

    assert_eq!(format!("{}", data), "89 == 19 = false
89 != 19 = true
89 > 19 = true
89 < 19 = false
89 >= 19 = true
89 <= 19 = false");
}

#[derive(Oxiplate)]
#[oxi_code = "{-}
{{ yes }} || {{ yes }} = {{ yes || yes }}
{{ yes }} || {{ no }} = {{ yes || no }}
{{ no }} || {{ yes }} = {{ no || yes }}
{{ no }} || {{ no }} = {{ no || no }}
{{ yes }} && {{ yes }} = {{ yes && yes }}
{{ yes }} && {{ no }} = {{ yes && no }}
{{ no }} && {{ yes }} = {{ no && yes }}
{{ no }} && {{ no }} = {{ no && no }}"]
struct OrAnd {
    yes: bool,
    no: bool,
}

#[test]
fn test_or_and() {
    let data = OrAnd {
        yes: true,
        no: false,
    };

    assert_eq!(format!("{}", data), "true || true = true
true || false = true
false || true = true
false || false = false
true && true = true
true && false = false
false && true = false
false && false = false");
}

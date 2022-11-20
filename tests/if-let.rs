use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "
{%- if let Some(count) = cats_count -%}
    Found {{ count }} cats!
{%- else -%}
    No cats found :(
{%- endif %}"]
struct Data {
    cats_count: Option<u8>,
}

#[test]
fn test_if_let_some() {
    let data = Data {
        cats_count: Some(5),
    };

    assert_eq!(format!("{}", data), "Found 5 cats!");
}

#[test]
fn test_if_let_none() {
    let data = Data { cats_count: None };

    assert_eq!(format!("{}", data), "No cats found :(");
}

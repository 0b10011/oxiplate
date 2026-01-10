use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate_inline(html: "
{% for (loop, value) in &values | loop -%}
    {% if loop.is_first -%}
        first:
    {%_ endif -%}

    #{{ loop.index1 }} (#{{ loop.index0 }}) {{ value }}
{% endfor %}"
)]
struct Loop {
    values: Vec<usize>,
}

#[test]
fn test_loop() {
    let data = Loop {
        values: vec![19, 89, 42],
    };

    assert_eq!(
        format!("{data}"),
        r"
first: #1 (#0) 19
#2 (#1) 89
#3 (#2) 42
"
    );
}

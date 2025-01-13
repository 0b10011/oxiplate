use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "Hello  \t\n {_} \r\n\t wo{_}r{-}ld \n\t {-} \t\n !"]
struct AdjustedWhitespace {}

#[test]
fn adjusted_whitespace() {
    let template = AdjustedWhitespace {};

    assert_eq!(format!("{template}"), "Hello world!");
}

#[derive(Oxiplate)]
#[oxiplate_inline = "Hello  \t\t  \r\n\t {{_ username _}}  \t\t  \r\n\t (  \t\t  \r\n\t {{- name \
                     -}}  \t\t  \r\n\t )!"]
struct WritWhitespaceControl {
    username: &'static str,
    name: &'static str,
}

#[test]
fn writ_whitespace_control() {
    let template = WritWhitespaceControl {
        username: "dia",
        name: "Diana",
    };

    assert_eq!(format!("{template}"), "Hello dia (Diana)!");
}

#[derive(Oxiplate)]
#[oxiplate_inline = "Hello  \t\t  \r\n\t {#_ Some cool comment _#}  \t\t  \r\n\t (  \t\t  \r\n\t \
                     {#- Hey another comment -#}  \t\t  \r\n\t )!"]
struct CommentWhitespaceControl {}

#[test]
fn comment_whitespace_control() {
    let template = CommentWhitespaceControl {};

    // It might be cool if this collapsed to a single space, but :shrug:.
    assert_eq!(format!("{template}"), "Hello  ()!");
}

#[derive(Oxiplate)]
#[oxiplate_inline = r#"
{{ "leave" }}  {{ "leave" }}
{{ "leave" }}  {{- "remove" }}
{{ "leave" }}  {{_ "replace" }}
{{ "remove" -}}  {{ "leave" }}
{{ "remove" -}}  {{- "remove" }}
{{ "replace" _}}  {{ "leave" }}
{{ "replace" _}}  {{_ "replace" }}
"#]
struct AdjacentTags {}

#[test]
fn adjacent_tags() {
    let template = AdjacentTags {};

    assert_eq!(
        format!("{template}"),
        "
leave  leave
leaveremove
leave replace
removeleave
removeremove
replace leave
replace replace
"
    );
}

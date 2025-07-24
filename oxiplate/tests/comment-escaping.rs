use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(html: "<!--{{ comment: comment }}-->")]
struct Data<'a> {
    comment: &'a str,
}

#[test]
fn comment() {
    let comments = [
        (
            "<tags> and hyphenated-text are fine!",
            "<!--<tags> and hyphenated-text are fine!-->",
            "Comment characters are normally fine as long as they're not in a special place or \
             grouped with others in a specific way",
        ),
        (
            "> hello",
            "<!--› hello-->",
            "Text must not start with the string `>`",
        ),
        (
            "-> hey",
            "<!--−› hey-->",
            "Text must not start with the string `->`",
        ),
        (
            "hello <!-- world",
            "<!--hello ‹ǃ−− world-->",
            "Text must not contain the string `<!--`",
        ),
        (
            "foo --> bar",
            "<!--foo −−› bar-->",
            "Text must not contain the string `-->`",
        ),
        (
            "baz --!> qux",
            "<!--baz −−ǃ› qux-->",
            "Text must not contain the string `--!>`",
        ),
        (
            "hey <!-",
            "<!--hey ‹ǃ−-->",
            "Text must not end with the string `<!-`",
        ),
        (
            "- hi",
            "<!--− hi-->",
            "Hyphens at the beginning of a comment are not allowed in XML because it can cause \
             double hyphens",
        ),
        (
            "--- hi",
            "<!--−−− hi-->",
            "Hyphens at the beginning of a comment are not allowed in XML because it can cause \
             double hyphens",
        ),
        (
            "hi -",
            "<!--hi −-->",
            "Hyphens at the end of a comment are not allowed in XML because it can cause double \
             hyphens",
        ),
        (
            "hi ---",
            "<!--hi −−−-->",
            "Hyphens at the end of a comment are not allowed in XML because it can cause double \
             hyphens",
        ),
        (
            "hi--bye",
            "<!--hi−−bye-->",
            "Double hyphens are not allowed in XML",
        ),
        (
            "hi---bye",
            "<!--hi−−−bye-->",
            "Double hyphens are not allowed in XML",
        ),
    ];
    for (comment, expected, reason) in comments {
        let data = Data { comment };
        assert_eq!(format!("{data}"), expected, "{reason}");
    }
}

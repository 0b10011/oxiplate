use nom::multi::many0;
use super::{super::Source, item::parse_tag, r#static::parse_static, Item, Res, Span, Static};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::{eof, opt};
use nom::sequence::tuple;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug, PartialEq, Eq)]
pub struct Template<'a>(pub Vec<Item<'a>>);

impl ToTokens for Template<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for item in &self.0 {
            tokens.append_all(quote! { #item });
        }
    }
}

pub(crate) fn parse<'a>(source: &'a Source, variables: &'a [&syn::Ident]) -> Template<'a> {
    let input = source.code.as_str();
    match try_parse(input, variables) {
        Ok((_, template)) => template,
        Err(nom::Err::Error(err)) | Err(nom::Err::Failure(err)) => {
            let origin = match &source.origin {
                Some(origin) => format!("Syntax error in {}", origin.display()),
                None => "Syntax error in inline template".into(),
            };
            Template(vec![Item::CompileError(format!(
                "{}:\n{}",
                origin,
                nom::error::convert_error(input, err)
            ))])
        }
        Err(nom::Err::Incomplete(_)) => {
            unreachable!("This should only happen in nom streams which aren't used by Oxiplate.")
        }
    }
}

fn try_parse<'a>(input: Span<'a>, _variables: &'a [&syn::Ident]) -> Res<&'a str, Template<'a>> {
    let (input, items_vec) = many0(parse_item)(input)?;

    // Return error if there's any input remaining.
    // Successful value is `("", "")`, so no need to capture.
    eof(input)?;

    let mut items = Vec::new();
    for mut item_vec in items_vec {
        items.append(&mut item_vec);
    }

    Ok(("", Template(items)))
}

pub(crate) fn parse_item(input: Span) -> Res<&str, Vec<Item>> {
    alt((
        parse_tag,
        parse_static,
        adjusted_whitespace,
    ))(input)
}

pub fn adjusted_whitespace(input: Span) -> Res<&str, Vec<Item>> {
    let (input, (leading_whitespace, tag, trailing_whitespace)) = tuple((
        opt(whitespace),
        alt((tag("{_}"), tag("{-}"))),
        opt(whitespace),
    ))(input)?;

    let whitespace = match tag {
        "{_}" => if leading_whitespace.is_some() || trailing_whitespace.is_some() {
            vec![Static(" ".to_owned()).into()]
        } else {
            vec![]
        },
        "{-}" => vec![],
        _ => unreachable!("Only whitespace control tags should be matched"),
    };

    Ok((input, whitespace))
}

// https://doc.rust-lang.org/reference/whitespace.html
pub fn is_whitespace(char: char) -> bool {
    matches!(
        char,
        '\u{0009}' // (horizontal tab, '\t')
        | '\u{000A}' // (line feed, '\n')
        | '\u{000B}' // (vertical tab)
        | '\u{000C}' // (form feed)
        | '\u{000D}' // (carriage return, '\r')
        | '\u{0020}' // (space, ' ')
        | '\u{0085}' // (next line)
        | '\u{200E}' // (left-to-right mark)
        | '\u{200F}' // (right-to-left mark)
        | '\u{2028}' // (line separator)
        | '\u{2029}' // (paragraph separator)
    )
}

pub fn whitespace(input: Span) -> Res<&str, Span> {
    take_while1(is_whitespace)(input)
}

#[test]
fn test_empty() {
    assert_eq!(
        parse(
            &Source {
                code: "".into(),
                origin: None
            },
            &[]
        ),
        Template(vec![])
    );
}

#[test]
fn test_word() {
    assert_eq!(
        parse(
            &Source {
                code: "Test".into(),
                origin: None
            },
            &[]
        ),
        Template(vec![Item::Static(Static("Test".to_owned()))])
    );
}

#[test]
fn test_phrase() {
    assert_eq!(
        parse(
            &Source {
                code: "Some text.".into(),
                origin: None
            },
            &[]
        ),
        Template(vec![Item::Static(Static("Some text.".to_owned()))])
    );
}

#[test]
fn test_stray_brace() {
    assert_eq!(
        parse(
            &Source {
                code: "Some {text}.".into(),
                origin: None
            },
            &[]
        ),
        Template(vec![Item::Static(Static("Some {text}.".to_owned()))])
    );
}

#[test]
fn test_writ() {
    assert_eq!(
        parse(
            &Source {
                code: "{{ greeting }}".into(),
                origin: None
            },
            &[]
        ),
        Template(vec![Item::Writ(super::Writ(
            super::Expression::Identifier(super::expression::IdentifierOrFunction::Identifier(
                super::expression::Identifier("greeting")
            ))
        )),])
    );
}

#[test]
fn test_trimmed_whitespace() {
    assert_eq!(
        parse(
            &Source {
                code: "Hello \t\n {-} \t\n world!".into(),
                origin: None
            },
            &[]
        ),
        Template(vec![
            Item::Static(Static("Hello".to_owned())),
            Item::Static(Static("world!".to_owned())),
        ])
    );
}

#[test]
fn test_trimmed_leading_whitespace() {
    assert_eq!(
        parse(
            &Source {
                code: "Hello \t\n {{- greeting }}".into(),
                origin: None
            },
            &[]
        ),
        Template(vec![
            Item::Static(Static("Hello".to_owned())),
            Item::Writ(super::Writ(super::Expression::Identifier(
                super::expression::IdentifierOrFunction::Identifier(super::expression::Identifier(
                    "greeting"
                ))
            ))),
        ])
    );
}

#[test]
fn test_trimmed_trailing_whitespace() {
    assert_eq!(
        parse(
            &Source {
                code: "{{ greeting -}} \t\n !".into(),
                origin: None
            },
            &[]
        ),
        Template(vec![
            Item::Writ(super::Writ(super::Expression::Identifier(
                super::expression::IdentifierOrFunction::Identifier(super::expression::Identifier(
                    "greeting"
                ))
            ))),
            Item::Static(Static("!".to_owned())),
        ])
    );
}

#[test]
fn test_collapsed_whitespace() {
    assert_eq!(
        parse(
            &Source {
                code: "Hello \t\n {_} \t\n world!".into(),
                origin: None
            },
            &[]
        ),
        Template(vec![
            Item::Static(Static("Hello".to_owned())),
            Item::Static(Static(" ".to_owned())),
            Item::Static(Static("world!".to_owned())),
        ])
    );
}

#[test]
fn test_collapsed_leading_whitespace() {
    assert_eq!(
        parse(
            &Source {
                code: "Hello \t\n {{_ greeting }}".into(),
                origin: None
            },
            &[]
        ),
        Template(vec![
            Item::Static(Static("Hello".to_owned())),
            Item::Static(Static(" ".to_owned())),
            Item::Writ(super::Writ(super::Expression::Identifier(
                super::expression::IdentifierOrFunction::Identifier(super::expression::Identifier(
                    "greeting"
                ))
            ))),
        ])
    );
}

#[test]
fn test_collapsed_trailing_whitespace_writ() {
    assert_eq!(
        parse(
            &Source {
                code: "{{ greeting _}} \t\n world!".into(),
                origin: None
            },
            &[]
        ),
        Template(vec![
            Item::Writ(super::Writ(super::Expression::Identifier(
                super::expression::IdentifierOrFunction::Identifier(super::expression::Identifier(
                    "greeting"
                ))
            ))),
            Item::Static(Static(" ".to_owned())),
            Item::Static(Static("world!".to_owned())),
        ])
    );
}

#[test]
fn test_collapsed_trailing_whitespace_comment() {
    assert_eq!(
        parse(
            &Source {
                code: "Hello {#- Some comment _#} \t\n world!".into(),
                origin: None
            },
            &[]
        ),
        Template(vec![
            Item::Static(Static("Hello".to_owned())),
            Item::Comment,
            Item::Static(Static(" ".to_owned())),
            Item::Static(Static("world!".to_owned())),
        ])
    );
}

#[test]
fn test_collapsed_whitespace_comment_no_whitespace() {
    assert_eq!(
        parse(
            &Source {
                code: "Hello{#_ Some comment _#}world!".into(),
                origin: None
            },
            &[]
        ),
        Template(vec![
            Item::Static(Static("Hello".to_owned())),
            Item::Comment,
            Item::Static(Static("world!".to_owned())),
        ])
    );
}

#[test]
fn test_collapsed_whitespace_writ_no_whitespace() {
    assert_eq!(
        parse(
            &Source {
                code: "Hello{{_ variable _}}world!".into(),
                origin: None
            },
            &[]
        ),
        Template(vec![
            Item::Static(Static("Hello".to_owned())),
            Item::Writ(super::Writ(super::Expression::Identifier(
                super::expression::IdentifierOrFunction::Identifier(super::expression::Identifier(
                    "variable"
                ))
            ))),
            Item::Static(Static("world!".to_owned())),
        ])
    );
}

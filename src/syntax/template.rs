use super::{item::parse_tag, r#static::parse_static, Item, Res, Span, Static};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::{eof, fail, opt};
use nom::error::VerboseError;
use nom::multi::many0;
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

pub fn parse<'a>(
    input: Span<'a>,
    variables: &'a [&syn::Ident],
) -> Result<Template<'a>, nom::Err<VerboseError<Span<'a>>>> {
    match try_parse(input, variables) {
        Ok((_, template)) => Ok(template),
        Err(err) => match err {
            nom::Err::Error(err) => Ok(Template(vec![Item::CompileError(
                nom::error::convert_error(input, err),
            )])),
            nom::Err::Failure(err) => Ok(Template(vec![Item::CompileError(
                nom::error::convert_error(input, err),
            )])),
            _ => Err(err),
        },
    }
}

fn try_parse<'a>(input: Span<'a>, variables: &'a [&syn::Ident]) -> Res<&'a str, Template<'a>> {
    let (input, items_vec) = many0(alt((
        parse_tag(variables),
        parse_static,
        adjusted_whitespace,
    )))(input)?;

    // Return error if there's any input remaining.
    // Successful value is `("", "")`, so no need to capture.
    eof(input)?;

    let mut items = Vec::new();
    for mut item_vec in items_vec {
        items.append(&mut item_vec);
    }

    Ok(("".into(), Template(items)))
}

pub fn adjusted_whitespace(input: Span) -> Res<&str, Vec<Item>> {
    let (input, (_, tag, _)) = tuple((
        opt(whitespace),
        alt((tag("{_}"), tag("{-}"))),
        opt(whitespace),
    ))(input)?;

    let whitespace = match tag {
        "{_}" => vec![Static(" ".to_owned()).into()],
        "{-}" => vec![],
        _ => return fail(input),
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
    assert_eq!(parse("".into(), &[]), Ok(Template(vec![])));
}

#[test]
fn test_word() {
    assert_eq!(
        parse("Test".into(), &[]),
        Ok(Template(vec![Item::Static(Static("Test".to_owned()))]))
    );
}

#[test]
fn test_phrase() {
    assert_eq!(
        parse("Some text.".into(), &[]),
        Ok(Template(vec![Item::Static(Static(
            "Some text.".to_owned()
        ))]))
    );
}

#[test]
fn test_stray_brace() {
    assert_eq!(
        parse("Some {text}.".into(), &[]),
        Ok(Template(vec![Item::Static(Static(
            "Some {text}.".to_owned()
        ))]))
    );
}

#[test]
fn test_writ() {
    assert_eq!(
        parse("{{ greeting }}".into(), &[]),
        Ok(Template(vec![Item::Writ(super::Writ(
            super::Expression::Identifier(super::expression::IdentifierOrFunction::Identifier(
                super::expression::Identifier("greeting")
            ))
        )),]))
    );
}

#[test]
fn test_trimmed_whitespace() {
    assert_eq!(
        parse("Hello \t\n {-} \t\n world!".into(), &[]),
        Ok(Template(vec![
            Item::Static(Static("Hello".to_owned())),
            Item::Static(Static("world!".to_owned())),
        ]))
    );
}

#[test]
fn test_trimmed_leading_whitespace() {
    assert_eq!(
        parse("Hello \t\n {{- greeting }}".into(), &[]),
        Ok(Template(vec![
            Item::Static(Static("Hello".to_owned())),
            Item::Writ(super::Writ(super::Expression::Identifier(
                super::expression::IdentifierOrFunction::Identifier(super::expression::Identifier(
                    "greeting"
                ))
            ))),
        ]))
    );
}

#[test]
fn test_trimmed_trailing_whitespace() {
    assert_eq!(
        parse("{{ greeting -}} \t\n !".into(), &[]),
        Ok(Template(vec![
            Item::Writ(super::Writ(super::Expression::Identifier(
                super::expression::IdentifierOrFunction::Identifier(super::expression::Identifier(
                    "greeting"
                ))
            ))),
            Item::Static(Static("!".to_owned())),
        ]))
    );
}

#[test]
fn test_collapsed_whitespace() {
    assert_eq!(
        parse("Hello \t\n {_} \t\n world!".into(), &[]),
        Ok(Template(vec![
            Item::Static(Static("Hello".to_owned())),
            Item::Static(Static(" ".to_owned())),
            Item::Static(Static("world!".to_owned())),
        ]))
    );
}

#[test]
fn test_collapsed_leading_whitespace() {
    assert_eq!(
        parse("Hello \t\n {{_ greeting }}".into(), &[]),
        Ok(Template(vec![
            Item::Static(Static("Hello".to_owned())),
            Item::Static(Static(" ".to_owned())),
            Item::Writ(super::Writ(super::Expression::Identifier(
                super::expression::IdentifierOrFunction::Identifier(super::expression::Identifier(
                    "greeting"
                ))
            ))),
        ]))
    );
}

#[test]
fn test_collapsed_trailing_whitespace_writ() {
    assert_eq!(
        parse("{{ greeting _}} \t\n world!".into(), &[]),
        Ok(Template(vec![
            Item::Writ(super::Writ(super::Expression::Identifier(
                super::expression::IdentifierOrFunction::Identifier(super::expression::Identifier(
                    "greeting"
                ))
            ))),
            Item::Static(Static(" ".to_owned())),
            Item::Static(Static("world!".to_owned())),
        ]))
    );
}

#[test]
fn test_collapsed_trailing_whitespace_comment() {
    assert_eq!(
        parse("Hello {#- Some comment _#} \t\n world!".into(), &[]),
        Ok(Template(vec![
            Item::Static(Static("Hello".to_owned())),
            Item::Comment,
            Item::Static(Static(" ".to_owned())),
            Item::Static(Static("world!".to_owned())),
        ]))
    );
}

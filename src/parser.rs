use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1, take_until, take_while, take_while1};
use nom::character::complete::char;
use nom::combinator::{eof, fail, opt, peek, recognize};
use nom::error::VerboseError;
use nom::multi::{many0, many_till};
use nom::sequence::{preceded, tuple};
use nom::Err as SynErr;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

type Res<T, U> = nom::IResult<T, U, VerboseError<T>>;

#[derive(Debug, PartialEq)]
pub struct Template<'a>(Vec<Item<'a>>);

impl ToTokens for Template<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for item in &self.0 {
            tokens.append_all(quote! { #item });
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Item<'a> {
    Comment,
    Writ(Writ<'a>),
    Statement(Statement<'a>),
    Static(Static<'a>),
}

impl ToTokens for Item<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            Item::Comment => quote! {},
            Item::Writ(writ) => quote! { #writ },
            Item::Statement(statement) => quote! { #statement },
            Item::Static(text) => quote! { #text },
        });
    }
}

#[derive(Debug, PartialEq)]
pub struct Writ<'a>(Expression<'a>);

impl<'a> From<Writ<'a>> for Item<'a> {
    fn from(writ: Writ<'a>) -> Self {
        Item::Writ(writ)
    }
}

impl ToTokens for Writ<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expression = &self.0;
        tokens.append_all(quote! { #expression });
    }
}

#[derive(Debug, PartialEq)]
pub struct Statement<'a>(&'a str);

impl<'a> From<Statement<'a>> for Item<'a> {
    fn from(statement: Statement<'a>) -> Self {
        Item::Statement(statement)
    }
}

impl ToTokens for Statement<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expression = &self.0;
        tokens.append_all(quote! {write!(f, "{}", #expression)?;});
    }
}

#[derive(Debug, PartialEq)]
pub struct Comment<'a>(&'a str);

impl<'a> From<Comment<'a>> for Item<'a> {
    fn from(_comment: Comment) -> Self {
        Item::Comment
    }
}

#[derive(Debug, PartialEq)]
pub struct Static<'a>(Vec<&'a str>);

impl<'a> From<Static<'a>> for Item<'a> {
    fn from(r#static: Static<'a>) -> Self {
        Item::Static(r#static)
    }
}

impl ToTokens for Static<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let text = &self.0;
        tokens.append_all(quote! {#(write!(f, "{}", #text)?;)*});
    }
}

pub fn parse<'a>(
    input: &'a str,
    variables: &'a Vec<&syn::Ident>,
) -> Result<Template<'a>, nom::Err<VerboseError<&'a str>>> {
    match try_parse(input, variables) {
        Ok((_, template)) => Ok(template),
        Err(err) => panic!("{:?}", err),
    }
}

fn try_parse<'a>(input: &'a str, variables: &'a Vec<&syn::Ident>) -> Res<&'a str, Template<'a>> {
    let (input, items) = many0(alt((parse_tag(variables), parse_static)))(input)?;

    // Return error if there's any input remaining.
    // Successful value is `("", "")`, so no need to capture.
    eof(input)?;

    Ok(("", Template(items)))
}

// https://doc.rust-lang.org/reference/whitespace.html
fn is_whitespace(char: char) -> bool {
    match char {
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
        => true,
        _ => false,
    }
}

fn whitespace(input: &str) -> Res<&str, &str> {
    take_while1(is_whitespace)(input)
}

#[derive(Debug)]
pub enum TagOpen {
    Writ,
    Statement,
    Comment,
}

pub fn tag_open(input: &str) -> Res<&str, TagOpen> {
    let (input, output) = alt((
        tag("{{"), // writ
        tag("{%"), // statement
        tag("{#"), // comment
    ))(input)?;

    match output {
        "{{" => Ok((input, TagOpen::Writ)),
        "{%" => Ok((input, TagOpen::Statement)),
        "{#" => Ok((input, TagOpen::Comment)),
        _ => panic!("This should never happen"),
    }
}

#[derive(Debug, PartialEq)]
enum Expression<'a> {
    Identifier(&'a str),
}

impl ToTokens for Expression<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            Expression::Identifier(identifier) => {
                let identifier = syn::Ident::new(&identifier, proc_macro2::Span::call_site());
                quote! {write!(f, "{}", self.#identifier)?;}
            }
        });
    }
}

fn expression(input: &str) -> Res<&str, Expression> {
    fn identifier(input: &str) -> Res<&str, Expression> {
        let (input, output) = take_while1(|char| match char {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => true,
            _ => false,
        })(input)?;

        Ok((input, Expression::Identifier(output)))
    }

    alt((identifier,))(input)
}

fn writ(input: &str) -> Res<&str, Item> {
    let (input, _) = opt(take_while(is_whitespace))(input)?;
    let (input, output) = expression(input)?;
    let (input, _) = preceded(opt(take_while(is_whitespace)), tag("}}"))(input)?;

    Ok((input, Item::Writ(Writ(output))))
}

fn statement(input: &str) -> Res<&str, Item> {
    let (input, output) = tag("test")(input)?;

    Ok((input, Item::Statement(Statement(output))))
}

fn comment(input: &str) -> Res<&str, Item> {
    let (input, _) = take_until("#}")(input)?;

    Ok((input, Item::Comment))
}

fn parse_tag<'a>(_variables: &'a Vec<&syn::Ident>) -> impl Fn(&str) -> Res<&str, Item> {
    |input| {
        let tag = match tag_open(input)? {
            (input, TagOpen::Writ) => writ(input),
            (input, TagOpen::Statement) => statement(input),
            (input, TagOpen::Comment) => comment(input),
        };

        match tag {
            Err(SynErr::Error(error)) => Err(SynErr::Failure(error)),
            error => error,
        }
    }
}

fn is_whitespace_or_brace(char: char) -> bool {
    char == '{' || is_whitespace(char)
}

pub fn collapse_whitespace_command(input: &str) -> Res<&str, char> {
    char('_')(input)
}

pub fn trim_whitespace_command(input: &str) -> Res<&str, char> {
    char('-')(input)
}

pub fn parse_static(input: &str) -> Res<&str, Item> {
    let (input, (output, _)) = many_till(
        alt((
            take_till1(is_whitespace_or_brace),
            take_while1(is_whitespace),
            tag("{"),
        )),
        peek(alt((
            recognize(tag_open),
            // Parse whitespace with tag if trimmed/collapsed
            recognize(tuple((
                opt(whitespace),
                tag_open,
                alt((collapse_whitespace_command, trim_whitespace_command)),
            ))),
            eof,
        ))),
    )(input)?;

    // Must be checked for many0() call will fail due to infinite loop
    if output.is_empty() {
        return fail(input);
    }

    Ok((input, Item::Static(Static(output))))
}

#[test]
fn test_empty() {
    assert_eq!(parse("", &vec![]), Ok(Template(vec![])));
}

#[test]
fn test_word() {
    assert_eq!(
        parse("Test", &vec![]),
        Ok(Template(vec![Item::Static(Static(vec!["Test"]))]))
    );
}

#[test]
fn test_phrase() {
    assert_eq!(
        parse("Some text.", &vec![]),
        Ok(Template(vec![Item::Static(Static(vec![
            "Some", " ", "text."
        ]))]))
    );
}

#[test]
fn test_stray_brace() {
    assert_eq!(
        parse("Some {text}.", &vec![]),
        Ok(Template(vec![Item::Static(Static(vec![
            "Some", " ", "{", "text}."
        ]))]))
    );
}

#[test]
fn test_writ() {
    assert_eq!(
        parse("{{ greeting }}", &vec![]),
        Ok(Template(vec![Item::Writ(Writ(Expression::Identifier(
            "greeting"
        ))),]))
    );
}

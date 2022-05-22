use super::{
    item::tag_start,
    template::{adjusted_whitespace, is_whitespace},
    Item, Res, Span,
};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1, take_while1};
use nom::combinator::{eof, fail, peek, recognize};
use nom::multi::many_till;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug, PartialEq, Eq)]
pub struct Static(pub String);

impl<'a> From<Static> for Item<'a> {
    fn from(r#static: Static) -> Self {
        Item::Static(r#static)
    }
}

impl ToTokens for Static {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let text = &self.0;
        tokens.append_all(quote! {write!(f, "{}", #text)?;});
    }
}

pub fn parse_static(input: Span) -> Res<&str, Vec<Item>> {
    let (input, (output, _)) = many_till(
        alt((
            take_till1(is_whitespace_or_brace),
            take_while1(is_whitespace),
            tag("{"),
        )),
        peek(alt((
            recognize(tag_start),
            recognize(adjusted_whitespace),
            eof,
        ))),
    )(input)?;

    // Must be checked for many0() call will fail due to infinite loop
    if output.is_empty() {
        return fail(input);
    }

    let mut string = "".to_owned();
    for item in output {
        string.push_str(&item);
    }

    Ok((input, vec![Item::Static(Static(string))]))
}

fn is_whitespace_or_brace(char: char) -> bool {
    char == '{' || is_whitespace(char)
}

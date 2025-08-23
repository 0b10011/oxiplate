use nom::bytes::complete::take_while1;
use nom::combinator::{cut, opt, peek};
use nom::sequence::pair;
use nom::Parser as _;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

use super::{Expression, Res};
use crate::syntax::expression::arguments::{arguments, ArgumentsGroup};
use crate::{Source, State};

pub(crate) fn identifier(input: Source) -> Res<Source, Expression> {
    let (input, (ident, arguments)) = pair(&ident, opt(arguments)).parse(input)?;

    let field = if let Some(arguments) = arguments {
        IdentifierOrFunction::Function(ident, arguments)
    } else {
        IdentifierOrFunction::Identifier(ident)
    };

    Ok((input, Expression::Identifier(field)))
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Identifier<'a> {
    pub ident: &'a str,
    pub source: Source<'a>,
}

impl ToTokens for Identifier<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = match self.ident.to_ascii_lowercase().as_str() {
            keyword @ ("self" | "super") => panic!("{keyword} cannot be a raw identifier"),

            // Keywords from <https://doc.rust-lang.org/reference/keywords.html>.
            // Prefix with `r#` so Rust will accept them as idents.
            "abstract" | "as" | "async" | "await" | "become" | "box" | "break" | "const"
            | "continue" | "crate" | "do" | "dyn" | "else" | "enum" | "extern" | "false"
            | "final" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "macro"
            | "macro_rules" | "match" | "mod" | "move" | "mut" | "override" | "priv" | "pub"
            | "ref" | "return" | "static" | "struct" | "trait" | "true" | "try" | "type"
            | "typeof" | "union" | "unsafe" | "unsized" | "use" | "virtual" | "where" | "while"
            | "yield" => syn::Ident::new_raw(self.ident, self.source.span()),

            _ => syn::Ident::new(self.ident, self.source.span()),
        };

        tokens.append_all(quote! { #ident });
    }
}

pub(crate) fn ident(input: Source) -> Res<Source, Identifier> {
    // Ignore if it starts with a number
    let (input, _) = peek(take_while1(
        |char: char| matches!(char, 'a'..='z' | 'A'..='Z' | '_'),
    ))
    .parse(input)?;

    let (input, ident) = cut(take_while1(
        |char: char| matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'),
    ))
    .parse(input)?;
    Ok((
        input,
        Identifier {
            ident: ident.as_str(),
            source: ident,
        },
    ))
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum IdentifierOrFunction<'a> {
    Identifier(Identifier<'a>),
    Function(Identifier<'a>, ArgumentsGroup<'a>),
}
impl IdentifierOrFunction<'_> {
    pub fn to_tokens(&self, state: &State) -> TokenStream {
        let mut tokens = TokenStream::new();

        match self {
            IdentifierOrFunction::Identifier(identifier) => {
                tokens.append_all(quote! { #identifier });
            }
            IdentifierOrFunction::Function(identifier, arguments) => {
                let arguments = arguments.to_tokens(state);

                tokens.append_all(quote! { #identifier #arguments });
            }
        }

        tokens
    }
}

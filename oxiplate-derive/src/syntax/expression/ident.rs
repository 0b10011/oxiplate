use nom::bytes::complete::{tag, take_while1};
use nom::combinator::{cut, opt, peek};
use nom::sequence::pair;
use nom::Parser as _;
use proc_macro2::{Group, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};

use super::{Expression, Res};
use crate::{Source, State};

pub(crate) fn identifier<'a>(state: &'a State) -> impl Fn(Source) -> Res<Source, Expression> + 'a {
    |input| {
        let (input, (ident, parens)) = pair(&ident, opt(tag("()"))).parse(input)?;

        let ident_str = ident.ident;
        let field = if let Some(parens) = parens {
            IdentifierOrFunction::Function(ident, parens)
        } else {
            IdentifierOrFunction::Identifier(ident)
        };
        let is_extending = input.original.is_extending;
        let is_local = state.local_variables.contains(ident_str);

        Ok((
            input,
            Expression::Identifier(
                field,
                if is_local {
                    IdentifierScope::Local
                } else if is_extending {
                    IdentifierScope::Data
                } else {
                    IdentifierScope::Parent
                },
            ),
        ))
    }
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
    Function(Identifier<'a>, Source<'a>),
}
impl ToTokens for IdentifierOrFunction<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            IdentifierOrFunction::Identifier(identifier) => {
                tokens.append_all(quote! { #identifier });
            }
            IdentifierOrFunction::Function(identifier, parens) => {
                let span = parens.span();
                let mut parens =
                    Group::new(proc_macro2::Delimiter::Parenthesis, TokenStream::new());
                parens.set_span(span);

                tokens.append_all(quote! { #identifier #parens });
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum IdentifierScope {
    Local,
    Parent,
    Data,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IdentField<'a> {
    parents: Vec<Identifier<'a>>,
    ident_or_function: IdentifierOrFunction<'a>,
    scope: IdentifierScope,
}

use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};

use super::{Expression, Res};
use crate::syntax::Error;
use crate::syntax::expression::arguments::{ArgumentsGroup, arguments};
use crate::syntax::parser::{Parser as _, opt, take};
use crate::tokenizer::parser::{TokenKind, TokenSlice};
use crate::{Source, State};

pub(crate) fn identifier(tokens: TokenSlice) -> Res<Expression> {
    let (tokens, (ident, arguments)) = (Identifier::parse, opt(arguments)).parse(tokens)?;

    let field = if let Some(arguments) = arguments {
        IdentifierOrFunction::Function(ident, arguments)
    } else {
        IdentifierOrFunction::Identifier(ident)
    };

    Ok((tokens, Expression::Identifier(field)))
}

#[derive(Debug)]
pub(crate) struct Identifier<'a> {
    source: &'a Source<'a>,
}

impl<'a> Identifier<'a> {
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, token) = take(TokenKind::Ident).parse(tokens)?;
        let source = token.source();

        let error_message = match source.as_str() {
            "oxiplate_formatter" => "`oxiplate_formatter` is a reserved name",
            "self" => "`self` is a reserved keyword",
            "super" => "`super` is a reserved keyword",
            _ => {
                return Ok((tokens, Self { source }));
            }
        };

        Err(Error::Unrecoverable {
            message: error_message.to_string(),
            source: source.clone(),
            previous_error: None,
            is_eof: false,
        })
    }

    pub fn as_str(&self) -> &'a str {
        self.source.as_str()
    }

    pub fn source(&self) -> &'a Source<'a> {
        self.source
    }
}

impl ToTokens for Identifier<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = match self.source.as_str().to_ascii_lowercase().as_str() {
            // Keywords from <https://doc.rust-lang.org/reference/keywords.html>.
            // Prefix with `r#` so Rust will accept them as idents.
            "abstract" | "as" | "async" | "await" | "become" | "box" | "break" | "const"
            | "continue" | "crate" | "do" | "dyn" | "else" | "enum" | "extern" | "false"
            | "final" | "fn" | "for" | "gen" | "if" | "impl" | "in" | "let" | "loop" | "macro"
            | "macro_rules" | "match" | "mod" | "move" | "mut" | "override" | "priv" | "pub"
            | "ref" | "return" | "static" | "struct" | "trait" | "true" | "try" | "type"
            | "typeof" | "union" | "unsafe" | "unsized" | "use" | "virtual" | "where" | "while"
            | "yield" => syn::Ident::new_raw(self.source.as_str(), self.source.span_token()),

            reserved_name @ ("self" | "super" | "oxiplate_formatter") => {
                unreachable!("`{}` should have generated an error instead", reserved_name);
            }

            _ => syn::Ident::new(self.source.as_str(), self.source.span_token()),
        };

        tokens.append_all(quote! { #ident });
    }
}

#[derive(Debug)]
pub(crate) enum IdentifierOrFunction<'a> {
    Identifier(Identifier<'a>),
    Function(Identifier<'a>, ArgumentsGroup<'a>),
}
impl<'a> IdentifierOrFunction<'a> {
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

    /// Get the `Source` for the entire identifier or function call.
    pub fn source(&self) -> Source<'a> {
        match self {
            IdentifierOrFunction::Identifier(identifier) => identifier.source().clone(),
            IdentifierOrFunction::Function(identifier, arguments_group) => {
                identifier.source().clone().merge(
                    arguments_group.source(),
                    "Arguments group should immediately follow the function name",
                )
            }
        }
    }
}

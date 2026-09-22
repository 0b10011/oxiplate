use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote, quote_spanned};

use super::{Expression, Res};
use crate::parser::{Parser as _, alt, cut, fail, ignore_recoverable_errors, many0, take};
use crate::template::tokenizer::{TokenKind, TokenSlice};
use crate::{Source, State};

#[derive(Debug)]
pub(crate) struct Segment<'a> {
    ident_segment: IdentSegment<'a>,
}

impl<'a> Segment<'a> {
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, ident_segment) = alt((
            take(TokenKind::Ident),
            take(TokenKind::SelfCurrentModule),
            take(TokenKind::SelfCurrentType),
            take(TokenKind::Super),
            take(TokenKind::Crate),
        ))
        .parse(tokens)?;

        let ident_segment = match ident_segment.kind() {
            TokenKind::Ident => IdentSegment::Identifier(Identifier {
                source: ident_segment.source(),
            }),
            _ => IdentSegment::Keyword(ident_segment.source().clone()),
        };

        Ok((tokens, Self { ident_segment }))
    }

    pub fn source(&self) -> Source<'a> {
        self.ident_segment.source().clone()
    }
}

impl ToTokens for Segment<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ident_segment.to_tokens(tokens);
    }
}

/// See: <https://doc.rust-lang.org/reference/paths.html#railroad-PathIdentSegment>
#[derive(Debug)]
enum IdentSegment<'a> {
    Identifier(Identifier<'a>),
    /// `Self`, `self`, `super`, `crate`
    Keyword(Source<'a>),
}

impl<'a> IdentSegment<'a> {
    fn source(&self) -> &Source<'a> {
        match self {
            Self::Identifier(identifier) => identifier.source(),
            Self::Keyword(source) => source,
        }
    }
}

impl ToTokens for IdentSegment<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Identifier(identifier) => identifier.to_tokens(tokens),
            Self::Keyword(source) => {
                let identifier = syn::Ident::new(source.as_str(), source.span_token());
                identifier.to_tokens(tokens);
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct Identifier<'a> {
    source: &'a Source<'a>,
}

impl<'a> Identifier<'a> {
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, token) = take(TokenKind::Ident).parse(tokens)?;
        let source = token.source();

        Ok((tokens, Self { source }))
    }

    /// Parses an identifier while returning an error for reserved keywords.
    /// Reserved keywords apply specifically to keywords
    /// that are parsed a different tokens by Oxiplate:
    /// - crate
    /// - self
    /// - Self
    /// - super
    pub fn parse_prevent_reserved_keywords(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let backup_tokens = tokens.clone();
        let (tokens, token) = alt((
            take(TokenKind::Ident),
            take(TokenKind::Crate),
            take(TokenKind::SelfCurrentType),
            take(TokenKind::SelfCurrentModule),
            take(TokenKind::Super),
        ))
        .parse(tokens)?;

        if !matches!(token.kind(), TokenKind::Ident) {
            return cut("This is a reserved keyword", fail()).parse(backup_tokens);
        }

        let source = token.source();

        Ok((tokens, Self { source }))
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

            _ => syn::Ident::new(self.source.as_str(), self.source.span_token()),
        };

        tokens.append_all(quote! { #ident });
    }
}

#[derive(Debug)]
pub(crate) struct Path<'a> {
    separator: Option<Separator<'a>>,
    segment: Segment<'a>,
    additional_segments: Vec<(Separator<'a>, Segment<'a>)>,
}
impl<'a> Path<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#paths-in-expressions>
    pub(crate) fn parser(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (separator, segment, additional_segments)) = (
            ignore_recoverable_errors(take(TokenKind::PathSeparator)),
            Segment::parse,
            many0((take(TokenKind::PathSeparator), Segment::parse)),
        )
            .parse(tokens)?;

        let separator = separator.map(|separator| Separator(separator.source().clone()));
        let additional_segments = additional_segments
            .into_iter()
            .map(|(separator, segment)| (Separator(separator.source().clone()), segment))
            .collect();

        Ok((
            tokens,
            Path {
                separator,
                segment,
                additional_segments,
            },
        ))
    }

    pub(super) fn template_field(&self, state: &State) -> Option<&Identifier<'_>> {
        if self.separator.is_some() || !self.additional_segments.is_empty() {
            return None;
        }

        let IdentSegment::Identifier(identifier) = &self.segment.ident_segment else {
            return None;
        };

        if !state.local_variables.contains(identifier.as_str())
            && state.fields.contains(identifier.as_str())
        {
            Some(identifier)
        } else {
            None
        }
    }

    pub fn to_tokens(&self, state: &State) -> (TokenStream, usize) {
        let span = self.segment.source().span_token();
        if let Some(identifier) = self.template_field(state) {
            (quote_spanned! {span=> self.#identifier }, 1)
        } else {
            let separator = &self.separator;
            let segment = &self.segment;
            let mut tokens = quote! { #separator #segment };
            for (separator, segment) in &self.additional_segments {
                tokens.append_all(quote! { #separator #segment });
            }
            (tokens, 1)
        }
    }

    /// Get the `Source` for the entire path.
    pub fn source(&self) -> Source<'a> {
        let mut source = if let Some(separator) = &self.separator {
            separator
                .0
                .clone()
                .merge(&self.segment.source(), "Segment expected after separator")
        } else {
            self.segment.source()
        };

        for (separator, segment) in &self.additional_segments {
            source = source
                .merge(&separator.0, "Separator expected after segment")
                .merge(&segment.source(), "Segment expected after separator");
        }

        source
    }
}
impl<'a> From<Path<'a>> for Expression<'a> {
    fn from(value: Path<'a>) -> Self {
        Expression::Path(value)
    }
}

#[derive(Debug)]
struct Separator<'a>(Source<'a>);

impl ToTokens for Separator<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let span = self.0.span_token();
        tokens.append_all(quote_spanned! {span=> :: });
    }
}

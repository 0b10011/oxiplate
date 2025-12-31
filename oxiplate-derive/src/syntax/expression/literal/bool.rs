use nom::Parser as _;
use nom::branch::alt;
use nom::bytes::complete::tag;
use proc_macro::Diagnostic;
use quote::quote;

use crate::syntax::expression::{Expression, Res};
use crate::{Source, Tokens};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Bool<'a> {
    value: bool,
    source: Source<'a>,
}

impl<'a> Bool<'a> {
    /// Parses a bool value: `true` or `false`
    pub(crate) fn parse(input: Source<'a>) -> Res<Source<'a>, Self> {
        let (input, source) = alt((
            tag("true"),
            tag("false"),
            #[cfg(feature = "unreachable")]
            tag("maybe"),
        ))
        .parse(input)?;
        let value = match source.as_str() {
            "true" => true,
            "false" => false,
            _ => {
                Diagnostic::spanned(
                    source.span().unwrap(),
                    proc_macro::Level::Error,
                    "Internal Oxiplate error. Unhandled bool.",
                )
                .help("Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Unhandled+bool")
                .help("Include template that caused the issue.")
                .emit();
                unreachable!("Internal Oxiplate error. See previous error for more information.");
            }
        };

        Ok((input, Self { value, source }))
    }

    pub(crate) fn source(&self) -> &Source<'a> {
        &self.source
    }

    pub(crate) fn to_tokens(&self) -> Tokens {
        let literal = ::syn::LitBool::new(self.value, self.source.span());
        (quote! { #literal }, 0)
    }
}

impl<'a> From<Bool<'a>> for Expression<'a> {
    fn from(value: Bool<'a>) -> Self {
        Expression::Bool(value)
    }
}

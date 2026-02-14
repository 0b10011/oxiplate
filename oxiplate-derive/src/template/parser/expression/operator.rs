use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote_spanned};

use super::super::Res;
use crate::parser::{Parser as _, alt, take};
use crate::template::tokenizer::{TokenKind, TokenSlice};
use crate::{Source, internal_error};

pub(super) fn parse_operator(tokens: TokenSlice) -> Res<Operator> {
    let (tokens, token) = alt((
        take(TokenKind::Plus),
        take(TokenKind::Minus),
        take(TokenKind::Asterisk),
        take(TokenKind::ForwardSlash),
        take(TokenKind::Percent),
        take(TokenKind::Eq),
        take(TokenKind::NotEq),
        take(TokenKind::GreaterThanOrEqualTo),
        take(TokenKind::LessThanOrEqualTo),
        take(TokenKind::GreaterThan),
        take(TokenKind::LessThan),
        take(TokenKind::Or),
        take(TokenKind::And),
        take(TokenKind::RangeInclusive),
        take(TokenKind::RangeExclusive),
        #[cfg(feature = "_unreachable")]
        take(TokenKind::Exclamation),
    ))
    .parse(tokens)?;

    macro_rules! op {
        ($variant:ident, $operator:ident, $token:ident) => {
            Operator {
                source: $token.source(),
                kind: OperatorKind::$variant,
            }
        };
    }

    let operator = token.source().clone();
    let operator = match operator.as_str() {
        "+" => op!(Addition, operator, token),
        "-" => op!(Subtraction, operator, token),
        "*" => op!(Multiplication, operator, token),
        "/" => op!(Division, operator, token),
        "%" => op!(Remainder, operator, token),

        "==" => op!(Equal, operator, token),
        "!=" => op!(NotEqual, operator, token),
        ">" => op!(GreaterThan, operator, token),
        "<" => op!(LessThan, operator, token),
        ">=" => op!(GreaterThanOrEqual, operator, token),
        "<=" => op!(LessThanOrEqual, operator, token),

        "||" => op!(Or, operator, token),
        "&&" => op!(And, operator, token),

        "..=" => op!(RangeInclusive, operator, token),
        ".." => op!(RangeExclusive, operator, token),

        _ => {
            internal_error!(operator.span_token().unwrap(), "Unhandled operator");
        }
    };

    Ok((tokens, operator))
}

#[derive(Debug)]
pub(crate) struct Operator<'a> {
    source: &'a Source<'a>,
    kind: OperatorKind,
}

#[derive(Debug)]
enum OperatorKind {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Remainder,

    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,

    Or,
    And,

    /// `start..=end` that matches all values where `start <= x <= end`.
    /// See: <https://doc.rust-lang.org/core/ops/struct.RangeInclusive.html>
    RangeInclusive,

    /// `start..end` that matches all values where `start <= x < end`.
    /// `start..` that matches all values where `start <= x`.
    /// See: <https://doc.rust-lang.org/core/ops/struct.Range.html>
    RangeExclusive,
}

impl<'a> Operator<'a> {
    pub(super) fn requires_expression_after(&self) -> bool {
        match self.kind {
            OperatorKind::Addition
            | OperatorKind::Subtraction
            | OperatorKind::Multiplication
            | OperatorKind::Division
            | OperatorKind::Remainder
            | OperatorKind::Equal
            | OperatorKind::NotEqual
            | OperatorKind::GreaterThan
            | OperatorKind::LessThan
            | OperatorKind::GreaterThanOrEqual
            | OperatorKind::LessThanOrEqual
            | OperatorKind::Or
            | OperatorKind::And
            | OperatorKind::RangeInclusive => true,

            // `expr..` is valid as well as `expr..expr`.
            OperatorKind::RangeExclusive => false,
        }
    }

    /// Get the `Source` for the operator and any leading whitespace.
    pub fn source(&self) -> &'a Source<'a> {
        self.source
    }
}

impl ToTokens for Operator<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        macro_rules! quote_op {
            ($token:tt) => {{
                let span = self.source.span_token();
                quote_spanned!(span=> $token)
            }};
        }

        tokens.append_all(match self.kind {
            OperatorKind::Addition => quote_op!(+),
            OperatorKind::Subtraction => quote_op!(-),
            OperatorKind::Multiplication => quote_op!(*),
            OperatorKind::Division => quote_op!(/),
            OperatorKind::Remainder => quote_op!(%),

            OperatorKind::Equal => quote_op!(==),
            OperatorKind::NotEqual => quote_op!(!=),
            OperatorKind::GreaterThan => quote_op!(>),
            OperatorKind::LessThan => quote_op!(<),
            OperatorKind::GreaterThanOrEqual => quote_op!(>=),
            OperatorKind::LessThanOrEqual => quote_op!(<=),

            OperatorKind::Or => quote_op!(||),
            OperatorKind::And => quote_op!(&&),

            OperatorKind::RangeInclusive => quote_op!(..=),
            OperatorKind::RangeExclusive => quote_op!(..),
        });
    }
}

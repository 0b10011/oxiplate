use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote_spanned};

use super::super::Res;
use crate::parser::{Parser as _, alt, cut, take};
use crate::template::parser::expression::{Expression, expression};
use crate::template::tokenizer::{TokenKind, TokenSlice};
use crate::{Source, internal_error};

fn parse_prefix_operator(tokens: TokenSlice) -> Res<PrefixOperator> {
    let (tokens, token) = alt((
        take(TokenKind::Ampersand),
        take(TokenKind::Asterisk),
        take(TokenKind::Exclamation),
        take(TokenKind::Minus),
        take(TokenKind::RangeInclusive),
        take(TokenKind::RangeExclusive),
        #[cfg(feature = "_unreachable")]
        take(TokenKind::Equal),
    ))
    .parse(tokens)?;

    let kind = match token.kind() {
        TokenKind::Ampersand => PrefixOperatorKind::Borrow,
        TokenKind::Asterisk => PrefixOperatorKind::Dereference,
        TokenKind::Exclamation => PrefixOperatorKind::Not,
        TokenKind::Minus => PrefixOperatorKind::Negative,
        TokenKind::RangeInclusive => PrefixOperatorKind::RangeInclusive,
        TokenKind::RangeExclusive => PrefixOperatorKind::RangeExclusive,
        _ => {
            internal_error!(
                token.source().span_token().unwrap(),
                "Unhandled prefix operator"
            );
        }
    };

    Ok((
        tokens,
        PrefixOperator {
            source: token.source(),
            kind,
        },
    ))
}
pub(super) fn parse_prefixed_expression<'a>(
    allow_generic_nesting: bool,
) -> impl Fn(TokenSlice<'a>) -> Res<'a, Expression<'a>> {
    move |tokens| {
        let (tokens, prefix_operator) = parse_prefix_operator.parse(tokens)?;

        let (tokens, expression) = if prefix_operator.cut_if_not_followed_by_expression() {
            cut(
                "Expected an expression after prefix operator",
                expression(allow_generic_nesting, true),
            )
            .parse(tokens)?
        } else {
            expression(allow_generic_nesting, true).parse(tokens)?
        };

        Ok((
            tokens,
            Expression::Prefixed(prefix_operator, Box::new(expression)),
        ))
    }
}

#[derive(Debug)]
pub struct PrefixOperator<'a> {
    source: &'a Source<'a>,
    kind: PrefixOperatorKind,
}

#[derive(Debug)]
enum PrefixOperatorKind {
    Borrow,
    Dereference,
    Not,

    /// `-` results in a negative value in the following expression.
    /// See: <https://doc.rust-lang.org/reference/expressions/operator-expr.html#negation-operators>
    Negative,

    /// `..=end` that matches all values where `x <= end`.
    /// See: <https://doc.rust-lang.org/core/ops/struct.RangeToInclusive.html>
    RangeInclusive,

    /// `..end` that matches all values where `x < end`.
    /// See: <https://doc.rust-lang.org/core/ops/struct.RangeTo.html>
    RangeExclusive,
}

impl<'a> PrefixOperator<'a> {
    fn cut_if_not_followed_by_expression(&self) -> bool {
        match self.kind {
            PrefixOperatorKind::Borrow
            | PrefixOperatorKind::Dereference
            | PrefixOperatorKind::Not
            | PrefixOperatorKind::Negative
            | PrefixOperatorKind::RangeInclusive => true,

            // The full range expression is this operator
            // without an expression after it
            // so this has to be recoverable
            // for that expression to be matched later.
            PrefixOperatorKind::RangeExclusive => false,
        }
    }

    /// Get the `Source` for the prefix operator.
    pub fn source(&self) -> &Source<'a> {
        self.source
    }
}

impl ToTokens for PrefixOperator<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        macro_rules! op {
            ($op:tt) => {{
                let span = self.source.span_token();
                quote_spanned! {span=> $op }
            }};
        }

        tokens.append_all(match self.kind {
            PrefixOperatorKind::Borrow => op!(&),
            PrefixOperatorKind::Dereference => op!(*),
            PrefixOperatorKind::Not => op!(!),
            PrefixOperatorKind::Negative => op!(-),
            PrefixOperatorKind::RangeInclusive => op!(..=),
            PrefixOperatorKind::RangeExclusive => op!(..),
        });
    }
}

use nom::Parser as _;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{cut, opt};
use nom::error::context;
use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote_spanned};

use super::super::Res;
use super::super::template::whitespace;
use crate::Source;
use crate::syntax::expression::{Expression, expression};

fn parse_prefix_operator(input: Source) -> Res<Source, PrefixOperator> {
    let (input, operator) = alt((
        tag("&"),
        tag("*"),
        tag("!"),
        tag("-"),
        tag("..="),
        tag(".."),
        #[cfg(feature = "unreachable")]
        tag("@"),
    ))
    .parse(input)?;
    let operator = match operator.as_str() {
        "&" => PrefixOperator::Borrow(operator),
        "*" => PrefixOperator::Dereference(operator),
        "!" => PrefixOperator::Not(operator),
        "-" => PrefixOperator::Negative(operator),
        "..=" => PrefixOperator::RangeInclusive(operator),
        ".." => PrefixOperator::RangeExclusive(operator),
        _ => unreachable!("All prefix operator cases should be covered"),
    };

    Ok((input, operator))
}
pub(super) fn parse_prefixed_expression(input: Source) -> Res<Source, Expression> {
    let (input, (_leading_whitespace, prefix_operator, _trailing_whitespace)) =
        (opt(whitespace), parse_prefix_operator, opt(whitespace)).parse(input)?;

    let (input, expression) = if prefix_operator.cut_if_not_followed_by_expression() {
        context(
            "Expected an expression after prefix operator",
            cut(expression(true, true)),
        )
        .parse(input)?
    } else {
        expression(true, true).parse(input)?
    };

    Ok((
        input,
        Expression::Prefixed(prefix_operator, Box::new(expression)),
    ))
}

#[derive(Debug, PartialEq, Eq)]
pub enum PrefixOperator<'a> {
    Borrow(Source<'a>),
    Dereference(Source<'a>),
    Not(Source<'a>),

    /// `-` results in a negative value in the following expression.
    /// See: <https://doc.rust-lang.org/reference/expressions/operator-expr.html#negation-operators>
    Negative(Source<'a>),

    /// `..=end` that matches all values where `x <= end`.
    /// See: <https://doc.rust-lang.org/core/ops/struct.RangeToInclusive.html>
    RangeInclusive(Source<'a>),

    /// `..end` that matches all values where `x < end`.
    /// See: <https://doc.rust-lang.org/core/ops/struct.RangeTo.html>
    RangeExclusive(Source<'a>),
}

impl<'a> PrefixOperator<'a> {
    fn cut_if_not_followed_by_expression(&self) -> bool {
        match self {
            PrefixOperator::Borrow(_)
            | PrefixOperator::Dereference(_)
            | PrefixOperator::Not(_)
            | PrefixOperator::Negative(_)
            | PrefixOperator::RangeInclusive(_) => true,

            // The full range expression is this operator
            // without an expression after it
            // so this has to be recoverable
            // for that expression to be matched later.
            PrefixOperator::RangeExclusive(_) => false,
        }
    }

    /// Get the `Source` for the prefix operator.
    pub fn source(&self) -> Source<'a> {
        match self {
            PrefixOperator::Borrow(source)
            | PrefixOperator::Dereference(source)
            | PrefixOperator::Not(source)
            | PrefixOperator::Negative(source)
            | PrefixOperator::RangeInclusive(source)
            | PrefixOperator::RangeExclusive(source) => source.clone(),
        }
    }
}

impl ToTokens for PrefixOperator<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        macro_rules! op {
            ($source:ident, $op:tt) => {{
                let span = $source.span();
                quote_spanned! {span=> $op }
            }};
        }
        tokens.append_all(match self {
            Self::Borrow(source) => op!(source, &),
            Self::Dereference(source) => op!(source, *),
            Self::Not(source) => op!(source, !),
            Self::Negative(source) => op!(source, -),
            Self::RangeInclusive(source) => op!(source, ..=),
            Self::RangeExclusive(source) => op!(source, ..),
        });
    }
}

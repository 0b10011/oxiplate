use nom::Parser as _;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{cut, fail, not, opt};
use nom::error::context;
use nom::multi::{many0, many1};
use nom::sequence::pair;
use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote, quote_spanned};
use syn::token::Dot;

mod arguments;
mod ident;
mod keyword;
mod literal;
mod operator;
mod prefix_operator;

use self::arguments::arguments;
use self::ident::IdentifierOrFunction;
pub(super) use self::ident::{Identifier, ident, identifier};
pub(super) use self::keyword::{Keyword, keyword};
use self::literal::{bool, char, number, string};
use super::Res;
use super::template::whitespace;
use crate::syntax::expression::operator::{Operator, parse_operator};
use crate::syntax::expression::prefix_operator::{PrefixOperator, parse_prefixed_expression};
use crate::syntax::item::tag_end;
use crate::{Source, State};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Field<'a> {
    dot: Source<'a>,
    ident_or_fn: IdentifierOrFunction<'a>,
}
impl Field<'_> {
    pub fn to_tokens(&self, state: &State) -> TokenStream {
        let span = self.dot.span();
        let dot = syn::parse2::<Dot>(quote_spanned! {span=> . })
            .expect("Dot should be able to be parsed properly here");

        let ident_or_fn = &self.ident_or_fn.to_tokens(state);
        quote! { #dot #ident_or_fn }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Expression<'a> {
    Identifier(IdentifierOrFunction<'a>),
    Char(Source<'a>),
    String(Source<'a>),
    Integer(Source<'a>),
    Float(Source<'a>),
    Bool(bool, Source<'a>),
    Group(Source<'a>, Box<ExpressionAccess<'a>>, Source<'a>),
    Concat(Vec<ExpressionAccess<'a>>, Source<'a>),
    Calc(
        Box<ExpressionAccess<'a>>,
        Operator<'a>,
        Box<Option<ExpressionAccess<'a>>>,
    ),
    Prefixed(PrefixOperator<'a>, Box<ExpressionAccess<'a>>),

    /// `..` that represents a range
    /// where the start/end matches whatever it is applied to.
    /// See: <https://doc.rust-lang.org/core/ops/struct.RangeFull.html>
    FullRange(Source<'a>),

    /// `expr[expr]`
    /// See:
    /// - <https://doc.rust-lang.org/reference/expressions/array-expr.html#array-and-slice-indexing-expressions>
    /// - <https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices>
    Index(
        Box<ExpressionAccess<'a>>,
        Source<'a>,
        Box<ExpressionAccess<'a>>,
        Source<'a>,
    ),
}

impl Expression<'_> {
    pub(crate) fn to_tokens(&self, state: &State) -> (TokenStream, usize) {
        match self {
            Expression::Identifier(identifier) => match &identifier {
                IdentifierOrFunction::Identifier(identifier) => {
                    let span = identifier.source.span();
                    if state.local_variables.contains(identifier.ident) {
                        (quote! { #identifier }, 1)
                    } else {
                        (quote_spanned! {span=> self.#identifier }, 1)
                    }
                }
                IdentifierOrFunction::Function(identifier, arguments) => {
                    let arguments = arguments.to_tokens(state);

                    let span = identifier.source.span();
                    if state.local_variables.contains(identifier.ident) {
                        (quote! { #identifier #arguments }, 1)
                    } else {
                        (quote_spanned! {span=> self.#identifier #arguments }, 1)
                    }
                }
            },
            Expression::Group(open_paren, expression, _close_paren) => {
                let (expression, expression_length) = expression.to_tokens(state);
                let span = open_paren.span();
                (quote_spanned! {span=> ( #expression ) }, expression_length)
            }
            Expression::Concat(expressions, concat_operator) => {
                Self::concat_to_tokens(state, expressions, concat_operator)
            }
            Expression::Calc(left, operator, right) => {
                let (left, left_length) = left.to_tokens(state);
                let (right, right_length) = if let Some(right) = right.as_ref() {
                    right.to_tokens(state)
                } else {
                    (TokenStream::new(), left_length)
                };
                (
                    quote! { #left #operator #right },
                    left_length.min(right_length),
                )
            }
            Expression::Prefixed(operator, expression) => {
                let (expression, expression_length) = expression.to_tokens(state);
                (quote! { #operator #expression }, expression_length)
            }
            Expression::Char(char) => {
                let literal = ::syn::LitChar::new(
                    char.as_str()
                        .chars()
                        .nth(0)
                        .expect("Char should always be 1 length"),
                    char.span(),
                );
                (quote! { #literal }, 1)
            }
            Expression::String(string) => {
                let literal = ::syn::LitStr::new(string.as_str(), string.span());
                (quote! { #literal }, string.as_str().len())
            }
            Expression::Integer(number) => {
                let literal = ::syn::LitInt::new(number.as_str(), number.span());
                (quote! { #literal }, number.as_str().len())
            }
            Expression::Float(number) => {
                let literal = ::syn::LitFloat::new(number.as_str(), number.span());
                (quote! { #literal }, number.as_str().len())
            }
            Expression::Bool(bool, source) => {
                let literal = ::syn::LitBool::new(*bool, source.span());
                (quote! { #literal }, 0)
            }
            Expression::FullRange(operator) => {
                let span = operator.span();
                (quote_spanned! {span=> .. }, 0)
            }
            Expression::Index(expression, open_bracket, range, _close_bracket) => {
                let span = open_bracket.span();
                let (expression, estimated_length) = expression.to_tokens(state);
                let (range, _range_length) = range.to_tokens(state);
                (
                    quote_spanned! {span=> #expression [ #range ] },
                    estimated_length,
                )
            }
        }
    }

    fn concat_to_tokens(
        state: &State,
        expressions: &Vec<ExpressionAccess>,
        concat_operator: &Source,
    ) -> (TokenStream, usize) {
        {
            let span = concat_operator.span();

            let mut format_tokens = vec![];
            let mut argument_tokens = vec![];
            let mut estimated_length = 0;
            for expression in expressions {
                match expression {
                    ExpressionAccess {
                        expression: Expression::String(string),
                        fields,
                    } if fields.is_empty() => {
                        estimated_length += string.as_str().len();
                        let string = syn::LitStr::new(string.as_str(), string.span());
                        format_tokens.push(quote_spanned! {span=> #string });
                    }
                    _ => {
                        format_tokens.push(quote_spanned! {span=> "{}" });
                        let (expression, expression_length) = expression.to_tokens(state);
                        estimated_length += expression_length;
                        argument_tokens.push(quote!(#expression));
                    }
                }
            }

            let format_concat_tokens = quote! { concat!(#(#format_tokens),*) };
            format_tokens.clear();

            if argument_tokens.is_empty() {
                (format_concat_tokens, estimated_length)
            } else {
                (
                    quote_spanned! {span=> format!(#format_concat_tokens, #(#argument_tokens),*) },
                    estimated_length,
                )
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ExpressionAccess<'a> {
    expression: Expression<'a>,
    fields: Vec<Field<'a>>,
}
impl ExpressionAccess<'_> {
    pub(crate) fn to_tokens(&self, state: &State) -> (TokenStream, usize) {
        let mut tokens = TokenStream::new();
        let (expression, estimated_length) = self.expression.to_tokens(state);
        tokens.append_all(expression);
        for field in &self.fields {
            tokens.append_all(field.to_tokens(state));
        }
        (tokens, estimated_length)
    }
}

pub(super) fn expression<'a>(
    allow_generic_nesting: bool,
    allow_concat_nesting: bool,
) -> impl Fn(Source) -> Res<Source, ExpressionAccess> + 'a {
    move |input| {
        let (input, (expression, fields)) = pair(
            alt((
                concat(allow_concat_nesting),
                calc(allow_generic_nesting),
                index(allow_generic_nesting),
                char,
                string,
                number,
                bool,
                identifier,
                parse_prefixed_expression,
                group,
                full_range,
            )),
            many0(field()),
        )
        .parse(input)?;

        Ok((input, ExpressionAccess { expression, fields }))
    }
}

fn field<'a>() -> impl Fn(Source) -> Res<Source, Field> + 'a {
    |input| {
        let (input, (dot, ident, arguments)) = (tag("."), &ident, opt(arguments)).parse(input)?;

        let ident_or_fn = if let Some(arguments) = arguments {
            IdentifierOrFunction::Function(ident, arguments)
        } else {
            IdentifierOrFunction::Identifier(ident)
        };

        Ok((input, Field { dot, ident_or_fn }))
    }
}

fn concat<'a>(allow_concat: bool) -> impl Fn(Source) -> Res<Source, Expression> + 'a {
    move |input| {
        if !allow_concat {
            return fail().parse(input);
        }
        let (input, (left, concats)) = (
            expression(false, false),
            many1((
                opt(whitespace),
                tag("~"),
                opt(whitespace),
                context("Expected an expression", cut(expression(true, false))),
            )),
        )
            .parse(input)?;
        let mut expressions = vec![left];
        let mut tilde_operator = None;
        for (_leading_whitespace, tilde, _trailing_whitespace, expression) in concats {
            if tilde_operator.is_none() {
                tilde_operator = Some(tilde);
            }
            expressions.push(expression);
        }
        Ok((
            input,
            Expression::Concat(
                expressions,
                tilde_operator.expect("Tilde should be guaranteed here"),
            ),
        ))
    }
}

fn calc<'a>(allow_generic_nesting: bool) -> impl Fn(Source) -> Res<Source, Expression> + 'a {
    move |input| {
        if !allow_generic_nesting {
            return fail().parse(input);
        }

        let (input, (left, _leading_whitespace, (), operator, _trailing_whitespace)) = (
            expression(false, false),
            opt(whitespace),
            // End tags like `-}}` and `%}` could be matched by operator; this ensures we can use `cut()` later.
            not(alt((tag_end("}}"), tag_end("%}"), tag_end("#}")))),
            parse_operator,
            opt(whitespace),
        )
            .parse(input)?;

        let (input, right) = if operator.requires_expression_after() {
            let (input, expression) =
                context("Expected an expression", cut(expression(true, true))).parse(input)?;
            (input, Some(expression))
        } else {
            opt(expression(true, true)).parse(input)?
        };

        Ok((
            input,
            Expression::Calc(Box::new(left), operator, Box::new(right)),
        ))
    }
}

fn group(input: Source) -> Res<Source, Expression> {
    let (input, (open, (inner, close))) = (
        tag("("),
        context(
            "Expected an expression",
            cut((expression(true, true), tag(")"))),
        ),
    )
        .parse(input)?;

    Ok((input, Expression::Group(open, Box::new(inner), close)))
}

/// Parses a full range expression (`..`).
/// See: <https://doc.rust-lang.org/core/ops/struct.RangeFull.html>
fn full_range(input: Source) -> Res<Source, Expression> {
    let (input, operator) = tag("..").parse(input)?;

    Ok((input, Expression::FullRange(operator)))
}

/// Parses an index expression (`expr[expr]`).
/// See: <https://doc.rust-lang.org/reference/expressions/array-expr.html#array-and-slice-indexing-expressions>
fn index(allow_generic_nesting: bool) -> impl Fn(Source) -> Res<Source, Expression> {
    move |input| {
        if !allow_generic_nesting {
            return fail().parse(input);
        }

        let (input, (expression, open, (range, close))) = (
            expression(false, false),
            tag("["),
            context(
                "Expected an expression",
                cut((expression(true, true), tag("]"))),
            ),
        )
            .parse(input)?;

        Ok((
            input,
            Expression::Index(Box::new(expression), open, Box::new(range), close),
        ))
    }
}

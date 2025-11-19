use nom::Parser as _;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{cut, fail, not, opt};
use nom::error::context;
use nom::multi::{many0, many1};
use nom::sequence::pair;
use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::token::Dot;

mod arguments;
mod concat;
mod ident;
mod keyword;
mod literal;
mod operator;
mod prefix_operator;

use self::arguments::arguments;
use self::concat::Concat;
use self::ident::IdentifierOrFunction;
pub(super) use self::ident::{Identifier, ident, identifier};
pub(super) use self::keyword::{Keyword, keyword};
use self::literal::{bool, char, number, string};
use super::Res;
use super::expression::arguments::ArgumentsGroup;
use super::expression::operator::{Operator, parse_operator};
use super::expression::prefix_operator::{PrefixOperator, parse_prefixed_expression};
use super::item::tag_end;
use super::template::whitespace;
use crate::{Source, State};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Field<'a> {
    dot: Source<'a>,
    ident_or_fn: IdentifierOrFunction<'a>,
}
impl<'a> Field<'a> {
    pub fn to_tokens(&self, state: &State) -> TokenStream {
        let span = self.dot.span();
        let dot = syn::parse2::<Dot>(quote_spanned! {span=> . })
            .expect("Dot should be able to be parsed properly here");

        let ident_or_fn = &self.ident_or_fn.to_tokens(state);
        quote! { #dot #ident_or_fn }
    }

    /// Get the `Source` for the field.
    pub(crate) fn source(&self) -> Source<'a> {
        self.dot.clone().merge(
            &self.ident_or_fn.source(),
            "Field or method name should immediately follow the dot",
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Expression<'a> {
    Identifier(IdentifierOrFunction<'a>),
    Char {
        value: char,
        source: Source<'a>,
    },
    String {
        value: Source<'a>,
        source: Source<'a>,
    },
    Integer(Source<'a>),
    Float(Source<'a>),
    Bool(bool, Source<'a>),
    Group(Source<'a>, Box<ExpressionAccess<'a>>, Source<'a>),
    Concat(Concat<'a>),
    Calc(
        Box<ExpressionAccess<'a>>,
        Operator<'a>,
        Box<Option<ExpressionAccess<'a>>>,
    ),
    Prefixed(PrefixOperator<'a>, Box<ExpressionAccess<'a>>),
    Cow {
        prefix: Source<'a>,
        expression: Box<ExpressionAccess<'a>>,
        source: Source<'a>,
    },

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

    /// `expr | filter(args)`
    Filter {
        name: Identifier<'a>,
        expression: Box<ExpressionAccess<'a>>,
        vertical_bar: Source<'a>,
        cow_prefix: Option<Source<'a>>,
        arguments: ArgumentsGroup<'a>,
        source: Source<'a>,
    },
}

impl<'a> Expression<'a> {
    #[allow(clippy::too_many_lines)]
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
            Expression::Concat(concat) => concat.to_tokens(state),
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
            Expression::Cow {
                prefix: _,
                expression,
                source,
            } => {
                #[cfg_attr(not(feature = "oxiplate"), allow(unused_variables))]
                let (expression, expression_length) = expression.to_tokens(state);
                let span = source.span();

                #[cfg(feature = "oxiplate")]
                let expression = quote_spanned! {span=>
                    ::oxiplate::CowStrWrapper::new((&&::oxiplate::ToCowStrWrapper::new(&(#expression))).to_cow_str())
                };

                #[cfg(not(feature = "oxiplate"))]
                let expression = quote_spanned! {span=>
                    compile_error!("Cow prefix requires the `oxiplate` library due to trait usage")
                };

                (expression, expression_length)
            }
            Expression::Char { value, source } => {
                let literal = ::syn::LitChar::new(*value, source.span());
                (quote! { #literal }, 1)
            }
            Expression::String { value, source } => {
                let literal = ::syn::LitStr::new(value.as_str(), source.span());
                (quote! { #literal }, value.as_str().len())
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
            Expression::Filter {
                name,
                expression,
                vertical_bar,
                cow_prefix,
                arguments,
                source,
            } => Self::filter(
                state,
                name,
                expression,
                vertical_bar,
                cow_prefix.as_ref(),
                arguments,
                source,
            ),
        }
    }

    /// Generate tokens for a filter expression.
    fn filter(
        state: &State,
        name: &Identifier,
        expression: &ExpressionAccess,
        vertical_bar: &Source,
        cow_prefix: Option<&Source>,
        arguments: &ArgumentsGroup,
        source: &Source,
    ) -> (TokenStream, usize) {
        let (expression, estimated_length) = expression.to_tokens(state);
        let mut argument_tokens = expression;

        if let Some((first_argument, remaining_arguments)) = &arguments.arguments {
            // First argument
            let comma_span = vertical_bar.span();
            argument_tokens.append_all(quote_spanned! {comma_span=> , });
            argument_tokens.append_all(first_argument.to_tokens(state).0);

            // Remaining arguments
            for (comma, expression) in remaining_arguments {
                let comma_span = comma.span();
                argument_tokens.append_all(quote_spanned! {comma_span=> , });
                argument_tokens.append_all(expression.to_tokens(state).0);
            }
        }

        let mut group =
            proc_macro2::Group::new(proc_macro2::Delimiter::Parenthesis, argument_tokens);
        let mut arguments_source = arguments.open_paren.clone();
        arguments_source.range.end = arguments.close_paren.range.end;
        group.set_span(source.span());
        let arguments = group.to_token_stream();

        let span = name.span();
        if let Some(cow_prefix) = cow_prefix {
            let span = cow_prefix.span();

            if cfg!(feature = "oxiplate") {
                (
                    quote_spanned! {span=>
                        ::oxiplate::CowStrWrapper::new(
                            (
                                &&::oxiplate::ToCowStrWrapper::new(
                                    &(crate::filters_for_oxiplate::#name #arguments)
                                )
                            ).to_cow_str()
                        )
                    },
                    estimated_length,
                )
            } else {
                (
                    quote_spanned! {span=>
                        compile_error!("Cow prefix requires the `oxiplate` library due to trait usage")
                    },
                    0,
                )
            }
        } else {
            (
                quote_spanned! {span=>
                    crate::filters_for_oxiplate::#name #arguments
                },
                estimated_length,
            )
        }
    }

    /// Get the `Source` for the entire expression.
    pub(crate) fn source(&self) -> Source<'a> {
        match self {
            Expression::Identifier(identifier_or_function) => identifier_or_function.source(),
            Expression::Char { value: _, source }
            | Expression::String { value: _, source }
            | Expression::Integer(source)
            | Expression::Float(source)
            | Expression::Bool(_, source)
            | Expression::FullRange(source)
            | Expression::Filter { source, .. }
            | Expression::Cow { source, .. } => source.clone(),
            Expression::Group(open_paren, expression_access, close_paren) => open_paren
                .clone()
                .merge(
                    &expression_access.source(),
                    "Expression should immediately follow open parentheses",
                )
                .merge(
                    close_paren,
                    "Closing parenthese should immediately follow the contained expression",
                ),
            Expression::Concat(concat) => concat.source.clone(),
            Expression::Calc(left, operator, right) => {
                if let Some(right) = &**right {
                    left.source()
                        .merge(operator.source(), "Operator should follow left expression")
                        .merge(&right.source(), "Right expression should follow operator")
                } else {
                    left.source()
                        .merge(operator.source(), "Operator should follow left expression")
                }
            }
            Expression::Prefixed(prefix_operator, expression) => prefix_operator
                .source()
                .merge(&expression.source(), "Expression should follow operator"),
            Expression::Index(left, open_bracket, index, close_bracket) => left
                .source()
                .merge(open_bracket, "Open bracket should follow left expression")
                .merge(&index.source(), "Index should follow open bracket")
                .merge(close_bracket, "Close bracket should follow index"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ExpressionAccess<'a> {
    expression: Expression<'a>,
    fields: Vec<Field<'a>>,
}
impl<'a> ExpressionAccess<'a> {
    pub(crate) fn to_tokens(&self, state: &State) -> (TokenStream, usize) {
        let mut tokens = TokenStream::new();
        let (expression, estimated_length) = self.expression.to_tokens(state);
        tokens.append_all(expression);
        for field in &self.fields {
            tokens.append_all(field.to_tokens(state));
        }
        (tokens, estimated_length)
    }

    /// Get the `Source` for expression accesses.
    pub(crate) fn source(&self) -> Source<'a> {
        let mut source: Source<'a> = self.expression.source();
        for field in &self.fields {
            source = source.merge(
                &field.source(),
                "Field source should be immediately after the rest of the expression",
            );
        }
        source
    }
}
impl<'a> From<Expression<'a>> for ExpressionAccess<'a> {
    fn from(expression: Expression<'a>) -> Self {
        ExpressionAccess {
            expression,
            fields: Vec::new(),
        }
    }
}

pub(super) fn expression<'a>(
    allow_generic_nesting: bool,
    allow_concat_nesting: bool,
) -> impl Fn(Source<'a>) -> Res<Source<'a>, ExpressionAccess<'a>> {
    move |input| {
        let (input, (expression, fields)) = pair(
            alt((
                Concat::parser(allow_concat_nesting),
                calc(allow_generic_nesting),
                index(allow_generic_nesting),
                filters(allow_generic_nesting),
                parse_cow_prefix,
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

/// Parses filters (`expr | filter()`).
fn filters(allow_generic_nesting: bool) -> impl Fn(Source) -> Res<Source, Expression> {
    move |input| {
        if !allow_generic_nesting {
            return fail().parse(input);
        }

        let (input, (expression, filters)) = (
            expression(false, false),
            many1((
                opt(whitespace),
                tag("|"),
                opt(whitespace),
                opt(tag(">")),
                opt(whitespace),
                context("Expected a filter name", cut(ident)),
                opt(whitespace),
                context(
                    "Expected parentheses surrounding zero or more arguments for the filter",
                    cut(arguments),
                ),
            )),
        )
            .parse(input)?;

        let mut source = expression.source();
        let mut expression_access = expression;
        for (
            leading_ws,
            vertical_bar,
            cow_prefix_leading_ws,
            cow_prefix,
            cow_prefix_trailing_ws,
            name,
            trailing_ws,
            arguments,
        ) in filters
        {
            source = source
                .merge_some(
                    leading_ws.as_ref(),
                    "Leading whitespace should follow expression",
                )
                .merge(
                    &vertical_bar,
                    "Vertical bar should follow leading whitespace",
                )
                .merge_some(
                    cow_prefix_leading_ws.as_ref(),
                    "Whitespace should follow vertical bar",
                )
                .merge_some(cow_prefix.as_ref(), "Cow prefix should follow whitespace")
                .merge_some(
                    cow_prefix_trailing_ws.as_ref(),
                    "Whitespace should follow cow prefix",
                )
                .merge(&name.source, "Filter name should follow whitespace")
                .merge_some(
                    trailing_ws.as_ref(),
                    "Trailing whitespace should follow filter name",
                )
                .merge(
                    &arguments.source(),
                    "Arguments should follow trailing whitespace",
                );

            expression_access = ExpressionAccess {
                expression: Expression::Filter {
                    name,
                    expression: Box::new(expression_access),
                    vertical_bar,
                    cow_prefix,
                    arguments,
                    source: source.clone(),
                },
                fields: Vec::new(),
            }
        }

        Ok((input, expression_access.expression))
    }
}

fn parse_cow_prefix(input: Source) -> Res<Source, Expression> {
    let (input, (prefix, (whitespace, expression))) = (
        tag(">"),
        context(
            "Expected an expression after cow prefix",
            cut((opt(whitespace), expression(false, false))),
        ),
    )
        .parse(input)?;

    let source = prefix
        .clone()
        .merge_some(whitespace.as_ref(), "Whitespace should follow cow prefix")
        .merge(&expression.source(), "Expression should follow whitespace");

    Ok((
        input,
        Expression::Cow {
            prefix,
            expression: Box::new(expression),
            source,
        },
    ))
}

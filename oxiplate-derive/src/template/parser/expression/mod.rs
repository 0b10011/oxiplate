use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote, quote_spanned};
use syn::token::Dot;

mod arguments;
mod concat;
mod group;
mod ident;
mod keyword;
mod literal;
mod operator;
mod prefix_operator;
mod tuple;

use self::arguments::arguments;
use self::concat::Concat;
use self::ident::IdentifierOrFunction;
pub(super) use self::ident::{Identifier, identifier};
pub(super) use self::keyword::{Keyword, KeywordParser};
pub(super) use self::literal::{Bool, Char, Float, Integer, Number, String};
use super::Res;
use super::expression::arguments::ArgumentsGroup;
use super::expression::operator::{Operator, parse_operator};
use super::expression::prefix_operator::{PrefixOperator, parse_prefixed_expression};
use crate::parser::{Parser as _, alt, context, cut, fail, into, many0, many1, opt, take};
use crate::template::parser::expression::group::Group;
use crate::template::parser::expression::tuple::Tuple;
use crate::template::tokenizer::{Token, TokenKind, TokenSlice};
use crate::{BuiltTokens, Source, State};

#[derive(Debug)]
pub(crate) struct Field<'a> {
    dot: Source<'a>,
    ident_or_fn: IdentifierOrFunction<'a>,
}
impl<'a> Field<'a> {
    pub fn to_tokens(&self, state: &State) -> TokenStream {
        let span = self.dot.span_token();
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

#[derive(Debug)]
pub(crate) enum Expression<'a> {
    Identifier(IdentifierOrFunction<'a>),
    Char(Char<'a>),
    String(String<'a>),
    Integer(Integer<'a>),
    Float(Float<'a>),
    Bool(Bool<'a>),
    Group(Group<'a>),
    Tuple(Tuple<'a>),
    Concat(Concat<'a>),
    Calc {
        left: Box<ExpressionAccess<'a>>,
        operator: Operator<'a>,
        right: Box<Option<ExpressionAccess<'a>>>,
        source: Source<'a>,
    },
    Prefixed(PrefixOperator<'a>, Box<ExpressionAccess<'a>>),
    Cow {
        prefix: Source<'a>,
        expression: Box<ExpressionAccess<'a>>,
        source: Source<'a>,
    },

    /// `..` that represents a range
    /// where the start/end matches whatever it is applied to.
    /// See: <https://doc.rust-lang.org/core/ops/struct.RangeFull.html>
    FullRange {
        source: Source<'a>,
    },

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
        arguments: Option<ArgumentsGroup<'a>>,
        source: Source<'a>,
    },
}

impl<'a> Expression<'a> {
    pub(crate) fn to_tokens(&self, state: &State) -> BuiltTokens {
        match self {
            Expression::Identifier(identifier) => match &identifier {
                IdentifierOrFunction::Identifier(identifier) => {
                    let span = identifier.source().span_token();
                    if state.local_variables.contains(identifier.as_str()) {
                        (quote! { #identifier }, 1)
                    } else {
                        (quote_spanned! {span=> self.#identifier }, 1)
                    }
                }
                IdentifierOrFunction::Function(identifier, arguments) => {
                    let arguments = arguments.to_tokens(state);

                    let span = identifier.source().span_token();
                    if state.local_variables.contains(identifier.as_str()) {
                        (quote! { #identifier #arguments }, 1)
                    } else {
                        (quote_spanned! {span=> (self.#identifier)#arguments }, 1)
                    }
                }
            },
            Expression::Group(group) => group.to_tokens(state),
            Expression::Tuple(tuple) => tuple.to_tokens(state),
            Expression::Concat(concat) => concat.to_tokens(state),
            Expression::Calc {
                left,
                operator,
                right,
                ..
            } => {
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
                prefix, expression, ..
            } => {
                #[cfg_attr(not(feature = "oxiplate"), allow(unused_variables))]
                let (expression, expression_length) = expression.to_tokens(state);
                let span = prefix.span_token();

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
            Expression::Char(char) => char.to_tokens(),
            Expression::String(string) => string.to_tokens(),
            Expression::Integer(number) => number.to_tokens(),
            Expression::Float(number) => number.to_tokens(),
            Expression::Bool(bool) => bool.to_tokens(),
            Expression::FullRange { source, .. } => {
                let span = source.span_token();
                (quote_spanned! {span=> .. }, 0)
            }
            Expression::Index(expression, open_bracket, range, _close_bracket) => {
                let span = open_bracket.span_token();
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
                arguments.as_ref(),
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
        arguments: Option<&ArgumentsGroup>,
        source: &Source,
    ) -> BuiltTokens {
        let (expression, estimated_length) = expression.to_tokens(state);
        let mut argument_tokens = expression;

        let arguments = if let Some(arguments) = arguments {
            if let Some((first_argument, remaining_arguments)) = &arguments.arguments {
                // First argument
                let comma_span = vertical_bar.span_token();
                argument_tokens.append_all(quote_spanned! {comma_span=> , });
                argument_tokens.append_all(first_argument.to_tokens(state).0);

                // Remaining arguments
                for (comma, expression) in remaining_arguments {
                    let comma_span = comma.span_token();
                    argument_tokens.append_all(quote_spanned! {comma_span=> , });
                    argument_tokens.append_all(expression.to_tokens(state).0);
                }
            }

            let mut group =
                proc_macro2::Group::new(proc_macro2::Delimiter::Parenthesis, argument_tokens);
            group.set_span(source.span_token());
            group.to_token_stream()
        } else {
            let mut group =
                proc_macro2::Group::new(proc_macro2::Delimiter::Parenthesis, argument_tokens);
            group.set_span(name.source().span_token());
            group.to_token_stream()
        };

        let span = name.source().span_token();
        if let Some(cow_prefix) = cow_prefix {
            let span = cow_prefix.span_token();

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

    /// Get the `Source` for the expression.
    pub(crate) fn source(&self) -> Source<'a> {
        match self {
            Expression::Identifier(identifier_or_function) => identifier_or_function.source(),
            Expression::Char(value) => value.source().clone(),
            Expression::String(value) => value.source().clone(),
            Expression::Integer(value) => value.source().clone(),
            Expression::Float(value) => value.source().clone(),
            Expression::Bool(value) => value.source().clone(),
            Expression::Calc { source, .. }
            | Expression::FullRange { source, .. }
            | Expression::Filter { source, .. }
            | Expression::Cow { source, .. } => source.clone(),
            Expression::Group(group) => group.source().clone(),
            Expression::Tuple(tuple) => tuple.source().clone(),
            Expression::Concat(concat) => concat.source().clone(),
            Expression::Prefixed(prefix_operator, expression) => prefix_operator
                .source()
                .clone()
                .merge(&expression.source(), "Expression should follow operator"),
            Expression::Index(left, open_bracket, index, close_bracket) => left
                .source()
                .merge(open_bracket, "Open bracket should follow left expression")
                .merge(&index.source(), "Index should follow open bracket")
                .merge(close_bracket, "Close bracket should follow index"),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ExpressionAccess<'a> {
    expression: Expression<'a>,
    fields: Vec<Field<'a>>,
}
impl<'a> ExpressionAccess<'a> {
    pub(crate) fn to_tokens(&self, state: &State) -> BuiltTokens {
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

pub(super) fn expression<'a>(
    allow_generic_nesting: bool,
    allow_concat_nesting: bool,
) -> impl Fn(TokenSlice<'a>) -> Res<'a, ExpressionAccess<'a>> {
    move |tokens| {
        let (tokens, (expression, fields)) = (
            alt((
                filters(allow_generic_nesting),
                Concat::parser(allow_concat_nesting),
                calc(allow_generic_nesting),
                index(allow_generic_nesting),
                parse_cow_prefix,
                into(Char::parse),
                into(String::parse),
                into(Number::parse),
                into(Bool::parse),
                identifier,
                parse_prefixed_expression(allow_generic_nesting),
                into(Group::parse),
                Tuple::parse,
                full_range,
            )),
            many0(field()),
        )
            .parse(tokens)?;

        Ok((tokens, ExpressionAccess { expression, fields }))
    }
}

fn field<'a>() -> impl Fn(TokenSlice<'a>) -> Res<'a, Field<'a>> + 'a {
    |tokens| {
        let (tokens, (dot, ident, arguments)) =
            (take(TokenKind::Period), Identifier::parse, opt(arguments)).parse(tokens)?;

        let ident_or_fn = if let Some(arguments) = arguments {
            IdentifierOrFunction::Function(ident, arguments)
        } else {
            IdentifierOrFunction::Identifier(ident)
        };

        Ok((
            tokens,
            Field {
                dot: dot.source().clone(),
                ident_or_fn,
            },
        ))
    }
}

fn calc<'a>(
    allow_generic_nesting: bool,
) -> impl Fn(TokenSlice<'a>) -> Res<'a, Expression<'a>> + 'a {
    move |tokens| {
        if !allow_generic_nesting {
            return context(
                "Generic nesting of calc not allowed in this context",
                fail(),
            )
            .parse(tokens);
        }

        let (tokens, (left, operator)) =
            (expression(false, false), parse_operator).parse(tokens)?;

        let (tokens, right) = if operator.requires_expression_after() {
            let (tokens, expression) =
                cut("Expected an expression", expression(true, true)).parse(tokens)?;
            (tokens, Some(expression))
        } else {
            opt(expression(true, true)).parse(tokens)?
        };

        let source = if let Some(right) = &right {
            left.source()
                .merge(operator.source(), "Operator should follow whitespace")
                .merge(&right.source(), "Right expression should follow whitespace")
        } else {
            left.source()
                .merge(operator.source(), "Operator should follow left expression")
        };

        Ok((
            tokens,
            Expression::Calc {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                source,
            },
        ))
    }
}

/// Parses a full range expression (`..`).
/// See: <https://doc.rust-lang.org/core/ops/struct.RangeFull.html>
fn full_range(tokens: TokenSlice) -> Res<Expression> {
    let (tokens, token) = take(TokenKind::RangeExclusive).parse(tokens)?;

    Ok((
        tokens,
        Expression::FullRange {
            source: token.source().clone(),
        },
    ))
}

/// Parses an index expression (`expr[expr]`).
/// See: <https://doc.rust-lang.org/reference/expressions/array-expr.html#array-and-slice-indexing-expressions>
fn index<'a>(allow_generic_nesting: bool) -> impl Fn(TokenSlice<'a>) -> Res<'a, Expression<'a>> {
    move |tokens| {
        if !allow_generic_nesting {
            return context(
                "Generic nesting of index not allowed in this context",
                fail(),
            )
            .parse(tokens);
        }

        let (tokens, (expression, open, (range, close))) = (
            expression(false, false),
            take(TokenKind::OpenBracket),
            cut(
                "Expected an expression",
                (expression(true, true), take(TokenKind::CloseBracket)),
            ),
        )
            .parse(tokens)?;

        Ok((
            tokens,
            Expression::Index(
                Box::new(expression),
                open.source().clone(),
                Box::new(range),
                close.source().clone(),
            ),
        ))
    }
}

/// Parses filters (`expr | filter()`).
fn filters<'a>(allow_generic_nesting: bool) -> impl Fn(TokenSlice<'a>) -> Res<'a, Expression<'a>> {
    move |tokens| {
        if !allow_generic_nesting {
            return context(
                "Generic nesting of filters not allowed in this context",
                fail(),
            )
            .parse(tokens);
        }

        let (tokens, (expression, filters)) = (
            expression(false, false),
            many1((
                take(TokenKind::VerticalBar),
                opt(take(TokenKind::GreaterThan)),
                cut("Expected a filter name", Identifier::parse),
                opt(arguments),
            )),
        )
            .parse(tokens)?;

        let mut source = expression.source();
        let mut expression_access = expression;
        for (vertical_bar, cow_prefix, name, arguments) in filters {
            source = source
                .merge(
                    vertical_bar.source(),
                    "Vertical bar should follow leading whitespace",
                )
                .merge_some(
                    cow_prefix.map(Token::source),
                    "Cow prefix should follow whitespace",
                )
                .merge(name.source(), "Filter name should follow whitespace")
                .merge_some(
                    arguments.as_ref().map(ArgumentsGroup::source),
                    "Arguments should follow trailing whitespace",
                );

            expression_access = ExpressionAccess {
                expression: Expression::Filter {
                    name,
                    expression: Box::new(expression_access),
                    vertical_bar: vertical_bar.source().clone(),
                    cow_prefix: cow_prefix.map(|token| token.source().clone()),
                    arguments,
                    source: source.clone(),
                },
                fields: Vec::new(),
            }
        }

        Ok((tokens, expression_access.expression))
    }
}

fn parse_cow_prefix(tokens: TokenSlice) -> Res<Expression> {
    let (tokens, (prefix, expression)) = (
        take(TokenKind::GreaterThan),
        cut(
            "Expected an expression after cow prefix",
            expression(false, false),
        ),
    )
        .parse(tokens)?;

    let source = prefix
        .source()
        .clone()
        .merge(&expression.source(), "Expression should follow whitespace");

    Ok((
        tokens,
        Expression::Cow {
            prefix: prefix.source().clone(),
            expression: Box::new(expression),
            source,
        },
    ))
}

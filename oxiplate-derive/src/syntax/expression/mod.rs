use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_while, take_while1};
use nom::character::complete::char;
use nom::combinator::{cut, fail, not, opt};
use nom::error::context;
use nom::multi::{many0, many1, many_till};
use nom::sequence::{pair, tuple};
use proc_macro2::{Group, TokenStream};
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use syn::token::Dot;

use self::ident::{IdentifierOrFunction, IdentifierScope};
use super::template::whitespace;
use super::Res;
use crate::syntax::item::tag_end;
use crate::{Source, State};

mod ident;
mod keyword;

pub(super) use self::ident::{ident, identifier, Identifier};
pub(super) use self::keyword::{keyword, Keyword};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Field<'a> {
    dot: Source<'a>,
    ident_or_fn: IdentifierOrFunction<'a>,
}
impl ToTokens for Field<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let span = self.dot.span();
        let dot = syn::parse2::<Dot>(quote_spanned! {span=> . })
            .expect("Dot should be able to be parsed properly here");

        let ident_or_fn = &self.ident_or_fn;
        tokens.append_all(quote! { #dot #ident_or_fn });
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Expression<'a> {
    Identifier(IdentifierOrFunction<'a>, IdentifierScope),
    String(Source<'a>),
    Number(Source<'a>),
    Bool(bool, Source<'a>),
    // Group(Box<Expression<'a>>),
    Concat(Vec<ExpressionAccess<'a>>, Source<'a>),
    Calc(
        Box<ExpressionAccess<'a>>,
        Operator<'a>,
        Box<ExpressionAccess<'a>>,
    ),
    Prefixed(PrefixOperator<'a>, Box<ExpressionAccess<'a>>),
}

impl ToTokens for Expression<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            Expression::Identifier(identifier, scope) => match &identifier {
                IdentifierOrFunction::Identifier(identifier) => {
                    let span = identifier.source.span();
                    match scope {
                        IdentifierScope::Local => quote_spanned! {span=> #identifier },
                        IdentifierScope::Parent => quote_spanned! {span=> self.#identifier },
                        IdentifierScope::Data => {
                            quote_spanned! {span=> self.oxiplate_extends_data.#identifier }
                        }
                    }
                }
                IdentifierOrFunction::Function(identifier, parens) => {
                    let span = parens.span();
                    let mut parens =
                        Group::new(proc_macro2::Delimiter::Parenthesis, TokenStream::new());
                    parens.set_span(span);

                    let span = identifier.source.span();
                    match scope {
                        IdentifierScope::Local => quote_spanned! {span=> #identifier #parens },
                        IdentifierScope::Parent => {
                            quote_spanned! {span=> self.#identifier #parens }
                        }
                        IdentifierScope::Data => {
                            quote_spanned! {span=> self.oxiplate_extends_data.#identifier #parens }
                        }
                    }
                }
            },
            Expression::Concat(expressions, concat_operator) => {
                let span = concat_operator.span();

                let mut format_tokens = vec![];
                let mut argument_tokens = vec![];
                for expression in expressions {
                    match expression {
                        ExpressionAccess {
                            expression: Expression::String(string),
                            fields,
                        } if fields.is_empty() => {
                            let string = syn::LitStr::new(string.as_str(), string.span());
                            format_tokens.push(quote_spanned! {span=> #string });
                        }
                        _ => {
                            format_tokens.push(quote_spanned! {span=> "{}" });
                            argument_tokens.push(quote!(#expression));
                        }
                    }
                }

                if argument_tokens.is_empty() {
                    return;
                }

                let format_concat_tokens = quote! { concat!(#(#format_tokens),*) };
                format_tokens.clear();

                if argument_tokens.is_empty() {
                    format_concat_tokens
                } else {
                    quote_spanned! {span=> format!(#format_concat_tokens, #(#argument_tokens),*) }
                }
            }
            Expression::Calc(left, operator, right) => quote!(#left #operator #right),
            Expression::Prefixed(operator, expression) => quote!(#operator #expression),
            Expression::String(string) => {
                let string = ::syn::LitStr::new(string.as_str(), string.span());
                quote! {
                    #string
                }
            }
            Expression::Number(number) => {
                let number = ::syn::LitInt::new(number.as_str(), number.span());
                quote! {
                    #number
                }
            }
            Expression::Bool(bool, source) => {
                let bool = ::syn::LitBool::new(*bool, source.span());
                quote! { #bool }
            }
        });
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ExpressionAccess<'a> {
    expression: Expression<'a>,
    fields: Vec<Field<'a>>,
}
impl ToTokens for ExpressionAccess<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expression = &self.expression;
        let fields = &self.fields;
        tokens.append_all(quote! { #expression #(#fields)* });
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Operator<'a> {
    Addition(Source<'a>),
    Subtraction(Source<'a>),
    Multiplication(Source<'a>),
    Division(Source<'a>),
    Remainder(Source<'a>),

    Equal(Source<'a>),
    NotEqual(Source<'a>),
    GreaterThan(Source<'a>),
    LessThan(Source<'a>),
    GreaterThanOrEqual(Source<'a>),
    LessThanOrEqual(Source<'a>),

    Or(Source<'a>),
    And(Source<'a>),
}

impl ToTokens for Operator<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            Operator::Addition(source) => {
                let span = source.span();
                quote_spanned!(span=> +)
            }
            Operator::Subtraction(source) => {
                let span = source.span();
                quote_spanned!(span=> -)
            }
            Operator::Multiplication(source) => {
                let span = source.span();
                quote_spanned!(span=> *)
            }
            Operator::Division(source) => {
                let span = source.span();
                quote_spanned!(span=> /)
            }
            Operator::Remainder(source) => {
                let span = source.span();
                quote_spanned!(span=> %)
            }

            Operator::Equal(source) => {
                let span = source.span();
                quote_spanned!(span=> ==)
            }
            Operator::NotEqual(source) => {
                let span = source.span();
                quote_spanned!(span=> !=)
            }
            Operator::GreaterThan(source) => {
                let span = source.span();
                quote_spanned!(span=> >)
            }
            Operator::LessThan(source) => {
                let span = source.span();
                quote_spanned!(span=> <)
            }
            Operator::GreaterThanOrEqual(source) => {
                let span = source.span();
                quote_spanned!(span=> >=)
            }
            Operator::LessThanOrEqual(source) => {
                let span = source.span();
                quote_spanned!(span=> <=)
            }

            Operator::Or(source) => {
                let span = source.span();
                quote_spanned!(span=> ||)
            }
            Operator::And(source) => {
                let span = source.span();
                quote_spanned!(span=> &&)
            }
        });
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PrefixOperator<'a> {
    Borrow(Source<'a>),
    Dereference(Source<'a>),
    Not(Source<'a>),
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
        });
    }
}

pub(super) fn expression<'a>(
    state: &'a State,
    allow_calc: bool,
    allow_concat: bool,
) -> impl Fn(Source) -> Res<Source, ExpressionAccess> + 'a {
    move |input| {
        let (input, (expression, fields)) = pair(
            alt((
                concat(state, allow_concat),
                calc(state, allow_calc),
                string,
                number,
                bool,
                identifier(state),
                prefixed_expression(state),
            )),
            many0(field()),
        )(input)?;

        Ok((input, ExpressionAccess { expression, fields }))
    }
}

fn field<'a>() -> impl Fn(Source) -> Res<Source, Field> + 'a {
    |input| {
        let (input, (dot, ident, parens)) = tuple((tag("."), &ident, opt(tag("()"))))(input)?;

        let ident_or_fn = if let Some(parens) = parens {
            IdentifierOrFunction::Function(ident, parens)
        } else {
            IdentifierOrFunction::Identifier(ident)
        };

        Ok((input, Field { dot, ident_or_fn }))
    }
}
fn operator(input: Source) -> Res<Source, Operator> {
    let (input, operator) = alt((
        tag("+"),
        tag("-"),
        tag("*"),
        tag("/"),
        tag("%"),
        tag("=="),
        tag("!="),
        tag(">="),
        tag("<="),
        tag(">"),
        tag("<"),
        tag("||"),
        tag("&&"),
    ))(input)?;

    let operator = match operator.as_str() {
        "+" => Operator::Addition(operator),
        "-" => Operator::Subtraction(operator),
        "*" => Operator::Multiplication(operator),
        "/" => Operator::Division(operator),
        "%" => Operator::Remainder(operator),

        "==" => Operator::Equal(operator),
        "!=" => Operator::NotEqual(operator),
        ">" => Operator::GreaterThan(operator),
        "<" => Operator::LessThan(operator),
        ">=" => Operator::GreaterThanOrEqual(operator),
        "<=" => Operator::LessThanOrEqual(operator),

        "||" => Operator::Or(operator),
        "&&" => Operator::And(operator),

        _ => unreachable!("All cases should be covered"),
    };

    Ok((input, operator))
}

/// Parses a bool value: `true` or `false`
fn bool(input: Source) -> Res<Source, Expression> {
    let (input, source) = alt((tag("true"), tag("false")))(input)?;
    let bool = match source.as_str() {
        "true" => true,
        "false" => false,
        _ => unreachable!("All cases should be covered"),
    };

    Ok((input, Expression::Bool(bool, source)))
}

fn number(input: Source) -> Res<Source, Expression> {
    let (input, number) = take_while1(|char: char| char.is_ascii_digit())(input)?;
    Ok((input, Expression::Number(number)))
}
fn string(input: Source) -> Res<Source, Expression> {
    let (input, (opening_hashes, _opening_quote)) =
        pair(take_while(|c| c == '#'), char('"'))(input)?;

    let closing = pair(char('"'), tag(opening_hashes.as_str()));
    let (input, (string, _)) = context(
        r#"String is opened but never closed. The string ending must be a double quote (") followed by the same number of hashes (#) as the string opening."#,
        cut(many_till(take(1u32), closing)),
    )(input)?;
    let (input, _closing_hashes) = tag(opening_hashes.as_str())(input)?;

    let full_string = if let Some(full_string) = string.first() {
        let mut full_string = full_string.clone();
        full_string.range.end = string.last().unwrap().range.end;
        full_string
    } else {
        let mut full_string = opening_hashes.clone();
        full_string.range.start = full_string.range.end;
        full_string
    };
    Ok((input, Expression::String(full_string)))
}
fn concat<'a>(
    state: &'a State,
    allow_concat: bool,
) -> impl Fn(Source) -> Res<Source, Expression> + 'a {
    move |input| {
        if !allow_concat {
            return fail(input);
        }
        let (input, (left, concats)) = tuple((
            expression(state, false, false),
            many1(tuple((
                opt(whitespace),
                tag("~"),
                opt(whitespace),
                context(
                    "Expected an expression",
                    cut(expression(state, true, false)),
                ),
            ))),
        ))(input)?;
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

fn calc<'a>(state: &'a State, allow_calc: bool) -> impl Fn(Source) -> Res<Source, Expression> + 'a {
    move |input| {
        if !allow_calc {
            return fail(input);
        }
        let (input, (left, _leading_whitespace, (), operator, _trailing_whitespace, right)) =
            tuple((
                expression(state, false, false),
                opt(whitespace),
                // End tags like `-}}` and `%}` could be matched by operator; this ensures we can use `cut()` later.
                not(alt((tag_end("}}"), tag_end("%}"), tag_end("#}")))),
                operator,
                opt(whitespace),
                context(
                    "Expected an expression",
                    cut(expression(state, true, false)),
                ),
            ))(input)?;
        Ok((
            input,
            Expression::Calc(Box::new(left), operator, Box::new(right)),
        ))
    }
}
fn prefix_operator(input: Source) -> Res<Source, PrefixOperator> {
    let (input, operator) = alt((tag("&"), tag("*"), tag("!")))(input)?;
    let operator = match operator.as_str() {
        "&" => PrefixOperator::Borrow(operator),
        "*" => PrefixOperator::Dereference(operator),
        "!" => PrefixOperator::Not(operator),
        _ => unreachable!("All cases should be covered"),
    };

    Ok((input, operator))
}
fn prefixed_expression<'a>(state: &'a State) -> impl Fn(Source) -> Res<Source, Expression> + 'a {
    |input| {
        let (input, (prefix_operator, expression)) = tuple((
            prefix_operator,
            context(
                "Expected an expression after prefix operator",
                cut(expression(state, true, true)),
            ),
        ))(input)?;

        Ok((
            input,
            Expression::Prefixed(prefix_operator, Box::new(expression)),
        ))
    }
}

use super::{template::whitespace, Res};
use crate::syntax::item::tag_end;
use crate::{Source, State};
use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_while, take_while1};
use nom::character::complete::char;
use nom::combinator::{cut, fail, not, opt, peek};
use nom::error::context;
use nom::multi::{many0, many_till};
use nom::sequence::{pair, tuple};
use proc_macro2::{Group, TokenStream};
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use syn::token::Dot;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Keyword<'a>(pub Source<'a>);

impl ToTokens for Keyword<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let span = self.0.span();
        let keyword = syn::Ident::new(self.0.as_str(), span);
        tokens.append_all(quote_spanned! {span=> #keyword });
    }

    fn to_token_stream(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.to_tokens(&mut tokens);
        tokens
    }

    fn into_token_stream(self) -> TokenStream
    where
        Self: Sized,
    {
        self.to_token_stream()
    }
}

pub(super) fn keyword<'a>(
    keyword: &'static str,
) -> impl Fn(Source<'a>) -> Res<Source<'a>, Keyword<'a>> + 'a {
    move |input: Source<'a>| {
        let (input, keyword) = tag(keyword)(input)?;
        Ok((input, Keyword(keyword)))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Identifier<'a> {
    pub ident: &'a str,
    pub source: Source<'a>,
}

impl ToTokens for Identifier<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = match self.ident.to_ascii_lowercase().as_str() {
            keyword @ ("self" | "super") => panic!("{keyword} cannot be a raw identifier"),

            // Keywords from <https://doc.rust-lang.org/reference/keywords.html>.
            // Prefix with `r#` so Rust will accept them as idents.
            "abstract" | "as" | "async" | "await" | "become" | "box" | "break" | "const"
            | "continue" | "crate" | "do" | "dyn" | "else" | "enum" | "extern" | "false"
            | "final" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "macro"
            | "macro_rules" | "match" | "mod" | "move" | "mut" | "override" | "priv" | "pub"
            | "ref" | "return" | "static" | "struct" | "trait" | "true" | "try" | "type"
            | "typeof" | "union" | "unsafe" | "unsized" | "use" | "virtual" | "where" | "while"
            | "yield" => syn::Ident::new_raw(self.ident, self.source.span()),

            _ => syn::Ident::new(self.ident, self.source.span()),
        };

        tokens.append_all(quote! { #ident });
    }
}

pub(super) fn ident(input: Source) -> Res<Source, Identifier> {
    // Ignore if it starts with a number
    let (input, _) = peek(take_while1(
        |char: char| matches!(char, 'a'..='z' | 'A'..='Z' | '_'),
    ))(input)?;

    let (input, ident) = cut(take_while1(
        |char: char| matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'),
    ))(input)?;
    Ok((
        input,
        Identifier {
            ident: ident.as_str(),
            source: ident,
        },
    ))
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum IdentifierOrFunction<'a> {
    Identifier(Identifier<'a>),
    Function(Identifier<'a>, Source<'a>),
}
impl ToTokens for IdentifierOrFunction<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            IdentifierOrFunction::Identifier(identifier) => {
                tokens.append_all(quote! { #identifier });
            }
            IdentifierOrFunction::Function(identifier, parens) => {
                let span = parens.span();
                let mut parens =
                    Group::new(proc_macro2::Delimiter::Parenthesis, TokenStream::new());
                parens.set_span(span);

                tokens.append_all(quote! { #identifier #parens });
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum IdentifierScope {
    Local,
    Parent,
    Data,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IdentField<'a> {
    parents: Vec<Identifier<'a>>,
    ident_or_function: IdentifierOrFunction<'a>,
    scope: IdentifierScope,
}

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
                        IdentifierScope::Data => quote_spanned! {span=> self._data.#identifier },
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
                            quote_spanned! {span=> self._data.#identifier #parens }
                        }
                    }
                }
            },
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
            ($source: ident, $op: tt) => {{
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
) -> impl Fn(Source) -> Res<Source, ExpressionAccess> + 'a {
    move |input| {
        let (input, (expression, fields)) = pair(
            alt((
                string,
                number,
                bool,
                calc(state, allow_calc),
                identifier(state),
                prefixed_expression(state),
            )),
            many0(field()),
        )(input)?;

        Ok((input, ExpressionAccess { expression, fields }))
    }
}

fn identifier<'a>(state: &'a State) -> impl Fn(Source) -> Res<Source, Expression> + 'a {
    |input| {
        let (input, (ident, parens)) = pair(&ident, opt(tag("()")))(input)?;

        let ident_str = ident.ident;
        let field = if let Some(parens) = parens {
            IdentifierOrFunction::Function(ident, parens)
        } else {
            IdentifierOrFunction::Identifier(ident)
        };
        let is_extending = input.original.is_extending;
        let is_local = state.local_variables.contains(ident_str);

        Ok((
            input,
            Expression::Identifier(
                field,
                if is_local {
                    IdentifierScope::Local
                } else if is_extending {
                    IdentifierScope::Data
                } else {
                    IdentifierScope::Parent
                },
            ),
        ))
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
    // TODO: Add support for _ separatation
    // TODO: Add support for other number types? (e.g., 0b10011)
    // TODO: Fail on numbers like `0123`
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
fn calc<'a>(state: &'a State, allow_calc: bool) -> impl Fn(Source) -> Res<Source, Expression> + 'a {
    move |input| {
        if !allow_calc {
            return fail(input);
        }
        let (input, (left, _leading_whitespace, (), operator, _trailing_whitespace, right)) =
            tuple((
                expression(state, false),
                opt(whitespace),
                // End tags like `-}}` and `%}` could be matched by operator; this ensures we can use `cut()` later.
                not(alt((tag_end("}}"), tag_end("%}"), tag_end("#}")))),
                operator,
                opt(whitespace),
                context("Expected an expression", cut(expression(state, true))),
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
                cut(expression(state, true)),
            ),
        ))(input)?;

        Ok((
            input,
            Expression::Prefixed(prefix_operator, Box::new(expression)),
        ))
    }
}

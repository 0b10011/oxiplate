use std::collections::HashSet;

use nom::Parser as _;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while, take_while1};
use nom::character::complete::char;
use nom::combinator::{cut, opt};
use nom::error::context;
use nom::multi::many0;
use nom::sequence::preceded;
use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote, quote_spanned};
use syn::Ident;
use syn::spanned::Spanned;

use super::super::expression::expression;
use super::super::{Item, Res};
use super::{Statement, StatementKind};
use crate::syntax::expression::{ExpressionAccess, Identifier, ident};
use crate::syntax::template::{Template, is_whitespace, whitespace};
use crate::{Source, State};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TypeName<'a>(&'a str, Source<'a>);

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Type<'a>(Vec<(TypeName<'a>, Source<'a>)>, TypeName<'a>, Fields<'a>);

impl<'a> Type<'a> {
    pub fn get_variables(&'a self) -> HashSet<&'a str> {
        match self {
            Type(_, _, fields) => fields.get_variables(),
        }
    }
}

impl ToTokens for Type<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for (type_name, separator) in &self.0 {
            let type_name: proc_macro2::TokenStream =
                type_name.0.parse().expect("Should be able to parse type");
            let separator: proc_macro2::TokenStream = separator
                .as_str()
                .parse()
                .expect("Should be able to parse type");
            tokens.append_all(quote! {
                #type_name #separator
            });
        }

        let type_name: proc_macro2::TokenStream =
            self.1.0.parse().expect("Should be able to parse type");

        let fields = &self.2;

        tokens.append_all(quote! {
            #type_name #fields
        });
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum TypeOrIdent<'a> {
    #[allow(dead_code)]
    Type(Box<Type<'a>>),
    Identifier(Identifier<'a>),
}

impl<'a> TypeOrIdent<'a> {
    fn get_variables(&'a self) -> HashSet<&'a str> {
        match self {
            Self::Type(ty) => ty.get_variables(),
            Self::Identifier(ident) => HashSet::from([ident.ident]),
        }
    }
}

impl ToTokens for TypeOrIdent<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Type(ty) => tokens.append_all(quote! { #ty }),
            Self::Identifier(ident) => tokens.append_all(quote! { #ident }),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum IfType<'a> {
    If(ExpressionAccess<'a>),
    IfLet(TypeOrIdent<'a>, Option<ExpressionAccess<'a>>),
}

#[derive(Debug)]
pub(crate) struct If<'a> {
    pub ifs: Vec<(IfType<'a>, Template<'a>)>,
    pub otherwise: Option<Template<'a>>,
    pub is_ended: bool,
}

impl<'a> If<'a> {
    pub fn get_active_variables(&'a self) -> HashSet<&'a str> {
        match self.ifs.last() {
            Some((IfType::If(_), _)) => HashSet::new(),
            Some((IfType::IfLet(ty, _), _)) => ty.get_variables(),
            None => unreachable!("If statements should always have at least one if"),
        }
    }

    pub fn add_item(&mut self, item: Item<'a>) {
        if self.is_ended {
            unreachable!(
                "Should not attempt to add item to `if` statement after statement is ended."
            );
        }

        match item {
            Item::Statement(Statement {
                kind: StatementKind::ElseIf(ElseIf(if_type)),
                source,
            }) => {
                if let Some(ref mut ifs) = self.otherwise {
                    ifs.0.push(Item::CompileError(
                        "`else` previously present in this if statement; expected `endif`"
                            .to_string(),
                        source,
                    ));
                } else {
                    self.ifs.push((if_type, Template(vec![])));
                }
            }
            Item::Statement(Statement {
                kind: StatementKind::Else,
                source,
            }) => {
                if let Some(ref mut ifs) = self.otherwise {
                    ifs.0.push(Item::CompileError(
                        "`else` already present in this if statement; expected `endif`".to_string(),
                        source,
                    ));
                } else {
                    self.otherwise = Some(Template(vec![]));
                }
            }
            Item::Statement(Statement {
                kind: StatementKind::EndIf,
                source: _,
            }) => {
                self.is_ended = true;
            }
            _ => {
                if let Some(template) = &mut self.otherwise {
                    template.0.push(item);
                } else {
                    self.ifs.last_mut().unwrap().1.0.push(item);
                }
            }
        }
    }

    pub(crate) fn to_tokens(&self, state: &State) -> (TokenStream, usize) {
        let mut tokens = TokenStream::new();
        let mut estimated_length = usize::MAX;

        let mut is_elseif = false;
        for (expression, template) in &self.ifs {
            match expression {
                IfType::If(expression) => {
                    let (expression, _expression_length) = expression.to_tokens(state);
                    let (template, template_length) = template.to_tokens(state);
                    estimated_length = estimated_length.min(template_length);
                    if is_elseif {
                        tokens.append_all(quote! { else if #expression { #template } });
                    } else {
                        tokens.append_all(quote! { if #expression { #template } });
                    }
                }
                IfType::IfLet(ty, Some(expression)) => {
                    let (expression, _expression_length) = expression.to_tokens(state);

                    let mut local_variables = ty.get_variables();
                    for value in state.local_variables {
                        local_variables.insert(value);
                    }
                    let branch_state = &State {
                        local_variables: &local_variables,
                        ..*state
                    };
                    let (template, template_length) = template.to_tokens(branch_state);
                    estimated_length = estimated_length.min(template_length);

                    if is_elseif {
                        tokens.append_all(quote! { else if let #ty = #expression { #template } });
                    } else {
                        tokens.append_all(quote! { if let #ty = #expression { #template } });
                    }
                }
                IfType::IfLet(ty, None) => {
                    let vars = ty.get_variables();

                    let mut ident = None;
                    for var in vars {
                        if ident.is_none() {
                            ident = Some(var);
                        } else {
                            ident = None;
                            break;
                        }
                    }

                    let span = ty.span();
                    let ident = if let Some(ident_str) = ident {
                        let ident = Ident::new(ident_str, ty.span());

                        if state.local_variables.contains(ident_str) {
                            quote_spanned! {span=> = &#ident }
                        } else {
                            quote_spanned! {span=> = &self.#ident }
                        }
                    } else {
                        quote! {}
                    };

                    let mut local_variables = ty.get_variables();
                    for value in state.local_variables {
                        local_variables.insert(value);
                    }
                    let branch_state = &State {
                        local_variables: &local_variables,
                        ..*state
                    };
                    let (template, template_length) = template.to_tokens(branch_state);
                    estimated_length = estimated_length.min(template_length);

                    if is_elseif {
                        tokens.append_all(quote! { else if let #ty #ident { #template } });
                    } else {
                        tokens.append_all(quote! { if let #ty #ident { #template } });
                    }
                }
            }

            is_elseif = true;
        }
        if let Some(template) = &self.otherwise {
            let (template, template_length) = template.to_tokens(state);
            estimated_length = estimated_length.min(template_length);
            tokens.append_all(quote! { else { #template } });
        }

        (tokens, estimated_length)
    }
}

impl<'a> From<If<'a>> for StatementKind<'a> {
    fn from(statement: If<'a>) -> Self {
        StatementKind::If(statement)
    }
}

pub(super) fn parse_type_name(input: Source) -> Res<Source, TypeName> {
    let (input, ident) =
        take_while1(|char: char| matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'))
            .parse(input)?;
    Ok((input, TypeName(ident.as_str(), ident)))
}

#[derive(Debug, PartialEq, Eq)]
enum Field<'a> {
    Ident(Identifier<'a>),
    Type {
        ident: Identifier<'a>,
        ty: TypeOrIdent<'a>,
    },
}

impl<'a> Field<'a> {
    fn get_variables(&'a self) -> HashSet<&'a str> {
        match self {
            Self::Ident(Identifier { ident, source: _ }) => HashSet::from([*ident]),
            Self::Type { ident: _, ty } => ty.get_variables(),
        }
    }
}

impl ToTokens for Field<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Ident(ident) => tokens.append_all(quote! { #ident }),
            Self::Type { ident, ty } => {
                let span = ident.span();
                tokens.append_all(quote_spanned! {span=> #ident: #ty });
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Fields<'a> {
    Struct {
        first_field: Field<'a>,
        additional_fields: Vec<(Source<'a>, Field<'a>)>,
    },
    Tuple {
        first_field: TypeOrIdent<'a>,
        additional_fields: Vec<(Source<'a>, TypeOrIdent<'a>)>,
    },
}

impl<'a> Fields<'a> {
    fn get_variables(&'a self) -> HashSet<&'a str> {
        let mut vars: HashSet<&'a str> = HashSet::new();
        match &self {
            Fields::Struct {
                first_field,
                additional_fields,
            } => {
                for var in first_field.get_variables() {
                    vars.insert(var);
                }
                for (_separator, field) in additional_fields {
                    for var in field.get_variables() {
                        vars.insert(var);
                    }
                }
            }
            Fields::Tuple {
                first_field,
                additional_fields,
            } => {
                for var in first_field.get_variables() {
                    vars.insert(var);
                }
                for (_separator, field) in additional_fields {
                    for var in field.get_variables() {
                        vars.insert(var);
                    }
                }
            }
        }
        vars
    }
}

impl ToTokens for Fields<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Fields::Struct {
                first_field,
                additional_fields,
            } => {
                let mut fields = quote! { #first_field };
                for (separator, field) in additional_fields {
                    let span = separator.span();
                    let separator = separator.as_str();
                    let separator = quote_spanned! {span=> #separator };
                    fields.append_all(quote! { #separator #field });
                }
                tokens.append_all(quote! { { #fields } });
            }
            Fields::Tuple {
                first_field,
                additional_fields,
            } => {
                let mut fields = quote! { #first_field };
                for (separator, field) in additional_fields {
                    let span = separator.span();
                    fields.append_all(quote_spanned! {span=> , #field });
                }
                tokens.append_all(quote! { ( #fields ) });
            }
        }
    }
}

fn parse_field(input: Source) -> Res<Source, Field> {
    let (input, (name, value)) = (
        ident,
        opt((
            opt(whitespace),
            tag(":"),
            opt(whitespace),
            parse_type_or_ident,
        )),
    )
        .parse(input)?;

    let field = if let Some((_leading_whitespace, _colon, _trailing_whitespace, ty)) = value {
        Field::Type { ident: name, ty }
    } else {
        Field::Ident(name)
    };

    Ok((input, field))
}

fn parse_struct_fields(input: Source) -> Res<Source, Fields> {
    let (input, (first_field, additional_fields_and_whitespace)) = (
        parse_field,
        many0((opt(whitespace), tag(","), opt(whitespace), parse_field)),
    )
        .parse(input)?;

    let mut additional_fields = Vec::with_capacity(additional_fields_and_whitespace.len());
    for (leading_whitespace, comma, trailing_whitespace, field) in additional_fields_and_whitespace
    {
        let source = if let Some(source) = leading_whitespace {
            source.merge(&comma, "Comma expected after whitespace")
        } else {
            comma.clone()
        }
        .merge_some(
            trailing_whitespace.as_ref(),
            "Whitespace expected after comma",
        );

        additional_fields.push((source, field));
    }

    Ok((
        input,
        Fields::Struct {
            first_field,
            additional_fields,
        },
    ))
}

fn parse_tuple_fields(input: Source) -> Res<Source, Fields> {
    let (input, (first_field, additional_fields_and_whitespace)) = (
        parse_type_or_ident,
        many0((
            opt(whitespace),
            tag(","),
            opt(whitespace),
            parse_type_or_ident,
        )),
    )
        .parse(input)?;

    let mut additional_fields = Vec::with_capacity(additional_fields_and_whitespace.len());
    for (leading_whitespace, comma, trailing_whitespace, field) in additional_fields_and_whitespace
    {
        let source = if let Some(source) = leading_whitespace {
            source.merge(&comma, "Comma expected after whitespace")
        } else {
            comma.clone()
        }
        .merge_some(
            trailing_whitespace.as_ref(),
            "Whitespace expected after comma",
        );

        additional_fields.push((source, field));
    }

    Ok((
        input,
        Fields::Tuple {
            first_field,
            additional_fields,
        },
    ))
}

fn parse_type_or_ident(input: Source) -> Res<Source, TypeOrIdent> {
    let (input, ty) = opt(parse_type).parse(input)?;

    if let Some(ty) = ty {
        return Ok((input, TypeOrIdent::Type(Box::new(ty))));
    }

    let (input, ident) = ident.parse(input)?;

    Ok((input, TypeOrIdent::Identifier(ident)))
}

pub(super) fn parse_type(input: Source) -> Res<Source, Type> {
    let (
        input,
        (
            path_segments,
            type_name,
            _whitespace,
            (_open_brace, _leading_whitespace, (fields, _trailing_whitespace, _close_brace)),
        ),
    ) = (
        many0((parse_type_name, tag("::"))),
        parse_type_name,
        opt(whitespace),
        alt((
            (
                tag("{"),
                opt(whitespace),
                cut((parse_struct_fields, opt(whitespace), tag("}"))),
            ),
            (
                tag("("),
                opt(whitespace),
                cut((parse_tuple_fields, opt(whitespace), tag(")"))),
            ),
        )),
    )
        .parse(input)?;

    Ok((input, Type(path_segments, type_name, fields)))
}

pub(super) fn parse_if(input: Source) -> Res<Source, Statement> {
    let (input, statement_source) = tag("if")(input)?;

    let (input, if_type) = parse_if_generic(input)?;

    Ok((
        input,
        Statement {
            kind: If {
                ifs: vec![(if_type, Template(vec![]))],
                otherwise: None,
                is_ended: false,
            }
            .into(),
            source: statement_source,
        },
    ))
}

fn parse_if_generic(input: Source) -> Res<Source, IfType> {
    // Consume at least one whitespace.
    let (input, _) = take_while1(is_whitespace).parse(input)?;

    let (input, r#let) = cut(opt((tag("let"), take_while1(is_whitespace)))).parse(input)?;

    if r#let.is_some() {
        let (input, ty) =
            context(r#"Expected a type after "let""#, cut(parse_type_or_ident)).parse(input)?;
        let (input, expression) = if ty.get_variables().len() == 1 {
            opt(preceded(
                take_while(is_whitespace),
                preceded(
                    char('='),
                    preceded(
                        take_while(is_whitespace),
                        context(
                            "Expected an expression after `=`",
                            cut(expression(true, true)),
                        ),
                    ),
                ),
            ))
            .parse(input)?
        } else {
            let (input, expression) = preceded(
                take_while(is_whitespace),
                preceded(
                    context("Expected `=`", cut(char('='))),
                    preceded(
                        take_while(is_whitespace),
                        context(
                            "Expected an expression after `=`",
                            cut(expression(true, true)),
                        ),
                    ),
                ),
            )
            .parse(input)?;
            (input, Some(expression))
        };
        Ok((input, IfType::IfLet(ty, expression)))
    } else {
        let (input, output) = context(
            "Expected an expression after `if`",
            cut(expression(true, true)),
        )
        .parse(input)?;
        Ok((input, IfType::If(output)))
    }
}

pub(super) fn parse_elseif(input: Source) -> Res<Source, Statement> {
    let (input, statement_source) = tag("elseif").parse(input)?;

    let (input, if_type) = parse_if_generic(input)?;

    Ok((
        input,
        Statement {
            kind: ElseIf(if_type).into(),
            source: statement_source,
        },
    ))
}

pub(super) fn parse_else(input: Source) -> Res<Source, Statement> {
    let (input, output) = tag("else").parse(input)?;

    Ok((
        input,
        Statement {
            kind: StatementKind::Else,
            source: output,
        },
    ))
}

pub(super) fn parse_endif(input: Source) -> Res<Source, Statement> {
    let (input, output) = tag("endif").parse(input)?;

    Ok((
        input,
        Statement {
            kind: StatementKind::EndIf,
            source: output,
        },
    ))
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, PartialEq, Eq)]
pub struct ElseIf<'a>(IfType<'a>);

impl<'a> From<ElseIf<'a>> for StatementKind<'a> {
    fn from(statement: ElseIf<'a>) -> Self {
        StatementKind::ElseIf(statement)
    }
}

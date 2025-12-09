use std::collections::HashSet;

use nom::Parser as _;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while, take_while1};
use nom::character::complete::char as nom_char;
use nom::combinator::{cut, opt};
use nom::error::context;
use nom::multi::many0;
use nom::sequence::preceded;
use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote, quote_spanned};

use super::super::expression::{bool, char, expression, number, string};
use super::super::{Item, Res};
use super::{Statement, StatementKind};
use crate::syntax::expression::{Expression, ExpressionAccess, Identifier, ident};
use crate::syntax::template::{Template, is_whitespace, whitespace};
use crate::{Source, State};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Type<'a> {
    path_segments: Vec<(Identifier<'a>, Source<'a>)>,
    name: Identifier<'a>,
    fields: Box<Fields<'a>>,
    source: Source<'a>,
}

impl<'a> Type<'a> {
    pub fn get_variables(&'a self) -> HashSet<&'a str> {
        let Type { fields, .. } = self;
        fields.get_variables()
    }

    fn to_tokens(&self, state: &State) -> TokenStream {
        let mut path_segments = TokenStream::new();
        for (type_name, separator) in &self.path_segments {
            let separator: proc_macro2::TokenStream = separator
                .as_str()
                .parse()
                .expect("Should be able to parse type");
            path_segments.append_all(quote! {
                #type_name #separator
            });
        }

        let type_name = &self.name;
        let fields = &self.fields.to_tokens(state);
        quote! {
            #path_segments #type_name #fields
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum DestructuringValue<'a> {
    Type(Type<'a>),
    Identifier(Identifier<'a>),
    Expression(Box<Expression<'a>>),
}

impl<'a> DestructuringValue<'a> {
    fn source(&self) -> Source<'a> {
        match self {
            Self::Type(ty) => ty.source.clone(),
            Self::Identifier(ident) => ident.source.clone(),
            Self::Expression(expression) => expression.source(),
        }
    }

    fn get_variables(&'a self) -> HashSet<&'a str> {
        match self {
            Self::Type(ty) => ty.get_variables(),
            Self::Identifier(ident) => HashSet::from([ident.ident]),
            Self::Expression(_expression) => HashSet::new(),
        }
    }

    fn to_tokens(&self, state: &State) -> TokenStream {
        match self {
            Self::Type(ty) => {
                let ty = ty.to_tokens(state);
                quote! { #ty }
            }
            Self::Identifier(ident) => quote! { #ident },
            Self::Expression(expression) => expression.to_tokens(state).0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum TypeOrIdent<'a> {
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

    fn to_tokens(&self, state: &State) -> TokenStream {
        match self {
            Self::Type(ty) => {
                let ty = ty.to_tokens(state);
                quote! { #ty }
            }
            Self::Identifier(ident) => quote! { #ident },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum IfType<'a> {
    If(ExpressionAccess<'a>),
    IfLet(TypeOrIdent<'a>, ExpressionAccess<'a>),
}

#[derive(Debug)]
pub(crate) struct If<'a> {
    pub ifs: Vec<(IfType<'a>, Template<'a>)>,
    pub otherwise: Option<Template<'a>>,
    pub is_ended: bool,
}

impl<'a> If<'a> {
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
                IfType::IfLet(ty, expression) => {
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

                    let ty = ty.to_tokens(state);

                    if is_elseif {
                        tokens.append_all(quote! { else if let #ty = #expression { #template } });
                    } else {
                        tokens.append_all(quote! { if let #ty = #expression { #template } });
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

fn parse_type_fields(input: Source) -> Res<Source, Fields> {
    let (input, fields) = opt(alt((parse_struct_fields, parse_tuple_fields))).parse(input)?;

    Ok((input, fields.unwrap_or(Fields::Unit)))
}

#[derive(Debug, PartialEq, Eq)]
enum Field<'a> {
    Ident(Identifier<'a>),
    Type {
        ident: Identifier<'a>,
        ty: DestructuringValue<'a>,
        source: Source<'a>,
    },
}

impl<'a> Field<'a> {
    fn source(&self) -> &Source<'a> {
        match self {
            Self::Ident(ident) => &ident.source,
            Self::Type { source, .. } => source,
        }
    }

    fn get_variables(&'a self) -> HashSet<&'a str> {
        match self {
            Self::Ident(Identifier { ident, source: _ }) => HashSet::from([*ident]),
            Self::Type { ty, .. } => ty.get_variables(),
        }
    }

    fn to_tokens(&self, state: &State) -> TokenStream {
        match self {
            Self::Ident(ident) => quote! { #ident },
            Self::Type { ident, ty, source } => {
                let span = source.span();
                let ty = ty.to_tokens(state);
                quote_spanned! {span=> #ident: #ty }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Fields<'a> {
    Struct {
        first_field: Field<'a>,
        additional_fields: Vec<(Source<'a>, Field<'a>)>,
        source: Source<'a>,
    },
    Tuple {
        first_field: DestructuringValue<'a>,
        additional_fields: Vec<(Source<'a>, DestructuringValue<'a>)>,
        source: Source<'a>,
    },
    Unit,
}

impl<'a> Fields<'a> {
    fn source(&self) -> Option<&Source<'a>> {
        match self {
            Self::Struct { source, .. } | Self::Tuple { source, .. } => Some(source),
            Self::Unit => None,
        }
    }

    fn get_variables(&'a self) -> HashSet<&'a str> {
        let mut vars: HashSet<&'a str> = HashSet::new();
        match &self {
            Fields::Struct {
                first_field,
                additional_fields,
                ..
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
                ..
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
            Fields::Unit => (),
        }
        vars
    }

    fn to_tokens(&self, state: &State) -> TokenStream {
        match self {
            Fields::Struct {
                first_field,
                additional_fields,
                source,
            } => {
                let mut fields = first_field.to_tokens(state);
                for (separator, field) in additional_fields {
                    let span = separator.span();
                    let field = field.to_tokens(state);
                    fields.append_all(quote_spanned! {span=> , #field });
                }
                let span = source.span();
                quote_spanned! {span=> { #fields } }
            }
            Fields::Tuple {
                first_field,
                additional_fields,
                source,
            } => {
                let mut fields = first_field.to_tokens(state);
                for (separator, field) in additional_fields {
                    let span = separator.span();
                    let field = field.to_tokens(state);
                    fields.append_all(quote_spanned! {span=> , #field });
                }
                let span = source.span();
                quote_spanned! {span=> ( #fields ) }
            }
            Fields::Unit => TokenStream::new(),
        }
    }
}

fn parse_field(input: Source) -> Res<Source, Field> {
    let (input, (name, value)) = (
        context("Expected field name", ident),
        opt((
            opt(whitespace),
            context("Expected `:`", tag(":")),
            opt(whitespace),
            context("Expected type or ident", cut(parse_type_or_ident_or_value)),
        )),
    )
        .parse(input)?;

    let field = if let Some((leading_whitespace, colon, trailing_whitespace, ty)) = value {
        let source = name
            .source
            .clone()
            .merge_some(
                leading_whitespace.as_ref(),
                "Whitespace expected after name",
            )
            .merge(&colon, "Colon expected after whitespace")
            .merge_some(
                trailing_whitespace.as_ref(),
                "Whitespace expected after colon",
            )
            .merge(&ty.source(), "Type expected after whitespace");

        Field::Type {
            ident: name,
            ty,
            source,
        }
    } else {
        Field::Ident(name)
    };

    Ok((input, field))
}

fn parse_struct_fields(input: Source) -> Res<Source, Fields> {
    let (
        input,
        (
            leading_whitespace,
            open_brace,
            middle_whitespace,
            (first_field, additional_fields_and_whitespace, trailing_whitespace, close_brace),
        ),
    ) = (
        opt(whitespace),
        tag("{"),
        opt(whitespace),
        context(
            "Expect fields followed by `}`",
            cut((
                parse_field,
                many0((opt(whitespace), tag(","), opt(whitespace), parse_field)),
                opt(whitespace),
                tag("}"),
            )),
        ),
    )
        .parse(input)?;

    let mut source = if let Some(leading_whitespace) = leading_whitespace {
        leading_whitespace.merge(&open_brace, "Open brace expected after whitespace")
    } else {
        open_brace
    }
    .merge_some(middle_whitespace.as_ref(), "Whitespace expected after `{`")
    .merge(
        first_field.source(),
        "First field expected after whitespace",
    );

    let mut additional_fields = Vec::with_capacity(additional_fields_and_whitespace.len());
    for (leading_whitespace, comma, trailing_whitespace, field) in additional_fields_and_whitespace
    {
        let comma_source = if let Some(source) = leading_whitespace {
            source.merge(&comma, "Comma expected after whitespace")
        } else {
            comma.clone()
        }
        .merge_some(
            trailing_whitespace.as_ref(),
            "Whitespace expected after comma",
        );

        source = source
            .merge(
                &comma_source,
                "Comma and whitespace expected after whitespace",
            )
            .merge(field.source(), "Field expected after whitespace");

        additional_fields.push((comma_source, field));
    }

    source = source
        .merge_some(
            trailing_whitespace.as_ref(),
            "Whitespace expected after last field",
        )
        .merge(&close_brace, "Closing brace expected after whitespace");

    Ok((
        input,
        Fields::Struct {
            first_field,
            additional_fields,
            source,
        },
    ))
}

fn parse_tuple_fields(input: Source) -> Res<Source, Fields> {
    let (
        input,
        (
            leading_whitespace,
            open_brace,
            middle_whitespace,
            (first_field, additional_fields_and_whitespace, trailing_whitespace, close_brace),
        ),
    ) = (
        opt(whitespace),
        tag("("),
        opt(whitespace),
        context(
            "Expect fields followed by `)`",
            cut((
                parse_type_or_ident_or_value,
                many0((
                    opt(whitespace),
                    tag(","),
                    opt(whitespace),
                    parse_type_or_ident_or_value,
                )),
                opt(whitespace),
                tag(")"),
            )),
        ),
    )
        .parse(input)?;

    let mut source = if let Some(leading_whitespace) = leading_whitespace {
        leading_whitespace.merge(&open_brace, "Open brace expected after whitespace")
    } else {
        open_brace
    }
    .merge_some(middle_whitespace.as_ref(), "Whitespace expected after `{`")
    .merge(
        &first_field.source(),
        "First field expected after whitespace",
    );

    let mut additional_fields = Vec::with_capacity(additional_fields_and_whitespace.len());
    for (leading_whitespace, comma, trailing_whitespace, field) in additional_fields_and_whitespace
    {
        let comma_source = if let Some(source) = leading_whitespace {
            source.merge(&comma, "Comma expected after whitespace")
        } else {
            comma.clone()
        }
        .merge_some(
            trailing_whitespace.as_ref(),
            "Whitespace expected after comma",
        );

        source = source
            .merge(
                &comma_source,
                "Comma and whitespace expected after whitespace",
            )
            .merge(&field.source(), "Field expected after whitespace");

        additional_fields.push((comma_source, field));
    }

    source = source
        .merge_some(
            trailing_whitespace.as_ref(),
            "Whitespace expected after last field",
        )
        .merge(&close_brace, "Closing brace expected after whitespace");

    Ok((
        input,
        Fields::Tuple {
            first_field,
            additional_fields,
            source,
        },
    ))
}

fn parse_type_or_ident_or_value(input: Source) -> Res<Source, DestructuringValue> {
    let (input, expression) = opt(alt((char, string, number, bool))).parse(input)?;
    if let Some(expression) = expression {
        return Ok((input, DestructuringValue::Expression(Box::new(expression))));
    }

    let (input, value) = parse_type_or_ident.parse(input)?;

    Ok(match value {
        TypeOrIdent::Identifier(ident) => (input, DestructuringValue::Identifier(ident)),
        TypeOrIdent::Type(ty) => (input, DestructuringValue::Type(*ty)),
    })
}

fn parse_type_or_ident(input: Source) -> Res<Source, TypeOrIdent> {
    let (input, ty) = parse_type.parse(input)?;

    match (ty.path_segments.len(), &*ty.fields) {
        (0, Fields::Unit) => Ok((input, TypeOrIdent::Identifier(ty.name))),
        _ => Ok((input, TypeOrIdent::Type(Box::new(ty)))),
    }
}

pub(super) fn parse_type(input: Source) -> Res<Source, Type> {
    let (input, path_segments) = many0((ident, tag("::"))).parse(input)?;

    let mut parser = (ident, parse_type_fields);

    let (input, (type_name, fields)) = if path_segments.is_empty() {
        parser.parse(input)?
    } else {
        context("Expected type name and optional fields", cut(parser)).parse(input)?
    };

    let mut source: Option<Source> = None;
    for (_, segment_source) in &path_segments {
        source = Some(source.map_or(segment_source.clone(), |source| {
            source.merge(segment_source, "Segment expected after previous segment")
        }));
    }
    let source = source
        .map_or(type_name.source.clone(), |source| {
            source.merge(&type_name.source, "Type name expected after path segments")
        })
        .merge_some(fields.source(), "Fields expected after type name");

    Ok((
        input,
        Type {
            path_segments,
            name: type_name,
            fields: Box::new(fields),
            source,
        },
    ))
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
        let (input, expression) = preceded(
            take_while(is_whitespace),
            preceded(
                context("Expected `=`", cut(nom_char('='))),
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

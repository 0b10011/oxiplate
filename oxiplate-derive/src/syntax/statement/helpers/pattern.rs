use std::collections::HashSet;

use nom::Parser as _;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{cut, opt};
use nom::error::context;
use nom::multi::{many0, many1};
use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote, quote_spanned};

use crate::syntax::Res;
use crate::syntax::expression::{Expression, Identifier, bool, char, ident, number, string};
use crate::syntax::template::whitespace;
use crate::{Source, State};

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

pub fn parse_type(input: Source) -> Res<Source, Type> {
    let (input, path) = opt(Path::parse).parse(input)?;

    if let Some(path) = path {
        let (input, fields) = parse_type_fields.parse(input)?;

        let ty = match fields {
            Some(Fields::Struct(fields)) => {
                let source = path
                    .source()
                    .clone()
                    .merge(&fields.source, "Fields should follow path");
                Type {
                    variant: TypeVariant::Struct {
                        path,
                        fields: Some(Box::new(fields)),
                    },
                    source,
                }
            }
            Some(Fields::Tuple(fields)) => {
                let source = path
                    .source()
                    .clone()
                    .merge(&fields.source, "Fields should follow path");
                Type {
                    variant: TypeVariant::Tuple {
                        path: Some(path),
                        fields: Box::new(fields),
                    },
                    source,
                }
            }
            None => {
                let source = path.source().clone();
                Type {
                    variant: TypeVariant::Struct { path, fields: None },
                    source,
                }
            }
        };

        Ok((input, ty))
    } else {
        let (input, fields) = parse_tuple_fields.parse(input)?;

        let source = fields.source.clone();

        Ok((
            input,
            Type {
                variant: TypeVariant::Tuple {
                    path: None,
                    fields: Box::new(fields),
                },
                source,
            },
        ))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Type<'a> {
    variant: TypeVariant<'a>,
    source: Source<'a>,
}

impl<'a> Type<'a> {
    pub fn source(&self) -> &Source<'a> {
        &self.source
    }

    pub fn get_variables(&'a self) -> HashSet<&'a str> {
        match &self.variant {
            TypeVariant::Struct {
                fields: Some(fields),
                ..
            } => fields.get_variables(),
            TypeVariant::Struct { fields: None, .. } => HashSet::new(),
            TypeVariant::Tuple { fields, .. } => fields.get_variables(),
        }
    }

    pub fn to_tokens(&self, state: &State) -> TokenStream {
        match &self.variant {
            TypeVariant::Struct {
                path:
                    Path {
                        segments,
                        name,
                        source: _,
                    },
                fields,
            } => {
                let mut path_segments = TokenStream::new();
                for (type_name, separator) in segments {
                    let separator: proc_macro2::TokenStream = separator
                        .as_str()
                        .parse()
                        .expect("Should be able to parse type");
                    path_segments.append_all(quote! {
                        #type_name #separator
                    });
                }

                let fields = if let Some(fields) = fields {
                    fields.to_tokens(state)
                } else {
                    quote! {}
                };
                quote! {
                    #path_segments #name #fields
                }
            }
            TypeVariant::Tuple { path, fields } => {
                let (path_segments, type_name) = if let Some(Path {
                    segments,
                    name,
                    source: _,
                }) = path
                {
                    let mut path_segments = TokenStream::new();
                    for (type_name, separator) in segments {
                        let separator: proc_macro2::TokenStream = separator
                            .as_str()
                            .parse()
                            .expect("Should be able to parse type");
                        path_segments.append_all(quote! {
                            #type_name #separator
                        });
                    }

                    (path_segments, quote! { #name })
                } else {
                    (quote! {}, quote! {})
                };

                let fields = fields.to_tokens(state);
                quote! {
                    #path_segments #type_name #fields
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum TypeVariant<'a> {
    Struct {
        path: Path<'a>,
        fields: Option<Box<StructFields<'a>>>,
    },
    Tuple {
        path: Option<Path<'a>>,
        fields: Box<TupleFields<'a>>,
    },
}

#[derive(Debug, PartialEq, Eq)]
struct Path<'a> {
    segments: Vec<(Identifier<'a>, Source<'a>)>,
    name: Identifier<'a>,
    source: Source<'a>,
}

impl<'a> Path<'a> {
    pub fn parse(input: Source<'a>) -> Res<Source<'a>, Self> {
        let (input, path) = opt((
            many1((ident, opt(whitespace), tag("::"), opt(whitespace))),
            context("Expected type name and optional fields", cut(ident)),
        ))
        .parse(input)?;

        let mut segments = vec![];
        if let Some((segments_with_whitespace, name)) = path {
            let mut source: Option<Source<'a>> = None;
            for (segment, leading_whitespace, colons, trailing_whitespace) in
                segments_with_whitespace
            {
                if let Some(existing_source) = source {
                    source = Some(
                        existing_source
                            .merge(&segment.source, "Segment expected")
                            .merge_some(
                                leading_whitespace.as_ref(),
                                "Whitespace expected after segment",
                            )
                            .merge(&colons, "Colons expected after whitespace")
                            .merge_some(
                                trailing_whitespace.as_ref(),
                                "Whitespace expected after colons",
                            ),
                    );
                } else {
                    source = Some(
                        segment
                            .source
                            .clone()
                            .merge_some(
                                leading_whitespace.as_ref(),
                                "Whitespace expected after segment",
                            )
                            .merge(&colons, "Colons expected after whitespace")
                            .merge_some(
                                trailing_whitespace.as_ref(),
                                "Whitespace expected after colons",
                            ),
                    );
                }

                segments.push((segment, colons));
            }

            let source = source
                .expect("Source should already exist because at least one segment is required")
                .merge(&name.source, "Name expected after segments");

            Ok((
                input,
                Self {
                    segments,
                    name,
                    source,
                },
            ))
        } else {
            let (input, name) = ident.parse(input)?;
            let source = name.source.clone();
            Ok((
                input,
                Self {
                    segments: vec![],
                    name,
                    source,
                },
            ))
        }
    }

    pub fn source<'b>(&'b self) -> &'b Source<'a> {
        &self.source
    }
}

fn parse_type_fields(input: Source) -> Res<Source, Option<Fields>> {
    let (input, fields) =
        opt(alt((parse_maybe_struct_fields, parse_maybe_tuple_fields))).parse(input)?;

    Ok((input, fields))
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
    Struct(StructFields<'a>),
    Tuple(TupleFields<'a>),
}

#[derive(Debug, PartialEq, Eq)]
struct StructFields<'a> {
    first_field: Field<'a>,
    additional_fields: Vec<(Source<'a>, Field<'a>)>,
    source: Source<'a>,
}

impl<'a> StructFields<'a> {
    fn get_variables(&'a self) -> HashSet<&'a str> {
        let mut vars: HashSet<&'a str> = HashSet::new();
        for var in self.first_field.get_variables() {
            vars.insert(var);
        }
        for (_separator, field) in &self.additional_fields {
            for var in field.get_variables() {
                vars.insert(var);
            }
        }
        vars
    }

    fn to_tokens(&self, state: &State) -> TokenStream {
        let mut fields = self.first_field.to_tokens(state);
        for (separator, field) in &self.additional_fields {
            let span = separator.span();
            let field = field.to_tokens(state);
            fields.append_all(quote_spanned! {span=> , #field });
        }
        let span = self.source.span();
        quote_spanned! {span=> { #fields } }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct TupleFields<'a> {
    first_field: DestructuringValue<'a>,
    additional_fields: Vec<(Source<'a>, DestructuringValue<'a>)>,
    source: Source<'a>,
}

impl<'a> TupleFields<'a> {
    fn get_variables(&'a self) -> HashSet<&'a str> {
        let mut vars: HashSet<&'a str> = HashSet::new();
        for var in self.first_field.get_variables() {
            vars.insert(var);
        }
        for (_separator, field) in &self.additional_fields {
            for var in field.get_variables() {
                vars.insert(var);
            }
        }
        vars
    }

    fn to_tokens(&self, state: &State) -> TokenStream {
        let mut fields = self.first_field.to_tokens(state);
        for (separator, field) in &self.additional_fields {
            let span = separator.span();
            let field = field.to_tokens(state);
            fields.append_all(quote_spanned! {span=> , #field });
        }
        let span = self.source.span();
        quote_spanned! {span=> ( #fields ) }
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

fn parse_maybe_struct_fields(input: Source) -> Res<Source, Fields> {
    let (input, fields) = parse_struct_fields.parse(input)?;

    Ok((input, Fields::Struct(fields)))
}

fn parse_struct_fields(input: Source) -> Res<Source, StructFields> {
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
        StructFields {
            first_field,
            additional_fields,
            source,
        },
    ))
}

fn parse_maybe_tuple_fields(input: Source) -> Res<Source, Fields> {
    let (input, fields) = parse_tuple_fields.parse(input)?;

    Ok((input, Fields::Tuple(fields)))
}

fn parse_tuple_fields(input: Source) -> Res<Source, TupleFields> {
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
        TupleFields {
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

    match ty.variant {
        TypeVariant::Struct {
            path:
                Path {
                    segments,
                    name,
                    source: _,
                },
            fields: None,
        } if segments.is_empty() => Ok((input, TypeOrIdent::Identifier(name))),
        _ => Ok((input, TypeOrIdent::Type(Box::new(ty)))),
    }
}

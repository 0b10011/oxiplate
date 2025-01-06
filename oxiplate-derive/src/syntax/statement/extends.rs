use std::fmt;

use super::super::expression::keyword;
use super::super::Res;
use super::{Statement, StatementKind};
use crate::syntax::template::{is_whitespace, Template};
use crate::syntax::Item;
use crate::Source;
use nom::bytes::complete::take_while1;
use nom::bytes::complete::{escaped, is_not, tag};
use nom::character::complete::one_of;
use nom::combinator::cut;
use nom::error::context;
use nom::sequence::tuple;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::Type;

pub struct Extends<'a> {
    is_extending: bool,
    data_type: Type,
    blocks: Vec<String>,
    path: Source<'a>,
    template: Template<'a>,
}

impl fmt::Debug for Extends<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Extends")
            // .field("data_type", &"UNSUPPORTED_SORRY")
            .field("blocks", &self.blocks)
            .field("path", &self.path)
            .field("template", &self.template)
            .finish()
    }
}

impl<'a> Extends<'a> {
    pub(crate) fn add_item(&mut self, mut item: Item<'a>) {
        #[allow(clippy::match_same_arms)]
        match &mut item {
            // Comments are fine to keep
            Item::Comment => self.template.0.push(item),

            // Compile errors must be kept
            Item::CompileError(_, _) => self.template.0.push(item),

            // Whitespace should be ignored
            Item::Whitespace(_) => (),

            // Block statements are allowed, but other statements should fail
            Item::Statement(Statement {
                kind: StatementKind::Block(_),
                ..
            }) => self.template.0.push(item),
            Item::Statement(_) => unimplemented!("Statements are not allowed here. Only comments, whitespace, and blocks are allowed."),

            // No static text or writs allowed
            Item::Static(_) => unimplemented!("Text is not allowed here. Only comments, whitespace, and blocks are allowed."),
            Item::Writ(_) => unimplemented!("Writs are not allowed here. Only comments, whitespace, and blocks are allowed."),
        }
    }
}

impl<'a> From<Extends<'a>> for StatementKind<'a> {
    fn from(statement: Extends<'a>) -> Self {
        StatementKind::Extends(statement)
    }
}

impl ToTokens for Extends<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Extends { path, template, .. } = self;

        let path = path.as_str();
        let path = ::std::path::PathBuf::from(
            ::std::env::var("CARGO_MANIFEST_DIR_OVERRIDE")
                .or(::std::env::var("CARGO_MANIFEST_DIR"))
                .unwrap(),
        )
        .join(option_env!("OXIP_TEMPLATE_DIR").unwrap_or("templates"))
        .join(path);
        let path = path.to_string_lossy();

        let data_type = &self.data_type;
        // FIXME: Should also include local vars here I think
        let mut inherited_blocks = vec![];
        let mut new_blocks = vec![];
        for item in &self.template.0 {
            if let Item::Statement(Statement {
                kind: StatementKind::Block(block),
                ..
            }) = item
            {
                if self.blocks.contains(&block.name.ident.to_string()) {
                    inherited_blocks.push(&block.name);
                } else {
                    new_blocks.push(&block.name);
                }
            }
        }
        if self.is_extending {
            tokens.append_all(quote! {
                #template
                #[derive(::oxiplate_derive::Oxiplate)]
                #[oxiplate_extends = include_str!(#path)]
                struct ExtendingTemplate<'a, F>
                where
                    F: Fn(&mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
                {
                    _data: &'a #data_type,
                    #(#inherited_blocks: &'a F,)*
                    #(#new_blocks: &'a F,)*
                }

                let template = ExtendingTemplate {
                    _data: &self._data,
                    #(#inherited_blocks: &self.#inherited_blocks,)*
                    #(#new_blocks: &#new_blocks,)*
                };
            });
        } else {
            tokens.append_all(quote! {
                #template
                #[derive(::oxiplate_derive::Oxiplate)]
                #[oxiplate_extends = include_str!(#path)]
                struct Template<'a, F>
                where
                    F: Fn(&mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
                {
                    // FIXME: Need to pass #extending and #extending_generics down to next level (type alias doesn't help because generics need to be passed sometimes)
                    _data: &'a #data_type,
                    #(#inherited_blocks: &'a F,)*
                    #(#new_blocks: &'a F,)*
                }

                let template = Template {
                    _data: self,
                    #(#inherited_blocks: &#inherited_blocks,)*
                    #(#new_blocks: &#new_blocks,)*
                };
            });
        }
        tokens.append_all(quote! {
            write!(f, "{}", template)?;
        });
    }
}

pub(super) fn parse_extends(input: Source) -> Res<Source, Statement> {
    let (input, _extends_keyword) = keyword("extends")(input)?;

    let (input, (_, _, path, _)) = cut(tuple((
        context("Expected space after 'extends'", take_while1(is_whitespace)),
        context(r#"Expected ""#, tag(r#"""#)),
        context(
            "Expected path to the template to extend",
            escaped(is_not(r#"""#), '\\', one_of(r#"""#)),
        ),
        context(r#"Expected ""#, tag(r#"""#)),
    )))(input)?;

    let is_extending = input.original.is_extending;
    let data_type = input.original.data_type.clone();
    let blocks = input.original.blocks.clone();

    Ok((
        input,
        Statement {
            kind: Extends {
                is_extending,
                data_type,
                blocks,
                path: path.clone(),
                template: Template(vec![]),
            }
            .into(),
            source: path,
        },
    ))
}

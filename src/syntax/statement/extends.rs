use std::fmt;

use super::super::expression::keyword;
use super::super::Res;
use super::{Statement, StatementKind};
use crate::syntax::template::is_whitespace;
use crate::syntax::Item;
use crate::Source;
use nom::bytes::complete::take_while1;
use nom::bytes::complete::{escaped, is_not, tag};
use nom::character::complete::one_of;
use nom::combinator::cut;
use nom::error::context;
use nom::sequence::tuple;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::Generics;

pub struct Extends<'a> {
    extending: Ident,
    extending_generics: Generics,
    blocks: Vec<String>,
    path: Source<'a>,
    items: Vec<Item<'a>>,
}

impl<'a> fmt::Debug for Extends<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Extends")
            .field("extending", &self.extending)
            // .field("extending_generics", &"UNSUPPORTED_SORRY")
            .field("blocks", &self.blocks)
            .field("path", &self.path)
            .field("items", &self.items)
            .finish()
    }
}

impl<'a> Extends<'a> {
    pub(crate) fn add_item(&mut self, mut item: Item<'a>) {
        match &mut item {
            // Comments are fine to keep
            Item::Comment => self.items.push(item),

            // Compile errors must be kept
            Item::CompileError(_, _) => self.items.push(item),

            // Whitespace should be ignored
            Item::Whitespace(_) => (),

            // Block statements are allowed, but other statements should fail
            Item::Statement(Statement {
                kind: StatementKind::Block(_),
                ..
            }) => self.items.push(item),
            Item::Statement(_) => todo!(),

            // No static text or writs allowed
            Item::Static(_) => todo!(),
            Item::Writ(_) => todo!(),
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
        let Extends { path, items, .. } = self;
        let path = path.as_str();
        let extending = &self.extending;
        let extending_generics = &self.extending_generics;
        let mut parent_blocks = vec![];
        let mut blocks = vec![];
        for item in &self.items {
            match item {
                Item::Statement(Statement {
                    kind: StatementKind::Block(block),
                    ..
                }) => {
                    if self.blocks.contains(&block.name.0.to_string()) {
                        parent_blocks.push(&block.name);
                    } else {
                        blocks.push(&block.name);
                    }
                }
                _ => (),
            }
        }
        tokens.append_all(quote! {
            #(#items)*
            #[derive(::oxiplate::Oxiplate)]
            #[oxiplate_extends = include_str!(#path)]
            struct Template<'a, F>
            where
                F: Fn(&mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
            {
                _data: &'a #extending #extending_generics,
                #(#parent_blocks: F,)*
                #(#blocks: F,)*
            }
            let template = Template {
                _data: self,
                #(self.#parent_blocks,)*
                #(#blocks,)*
            };
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

    let extending = input.original.ident.clone();
    let extending_generics = input.original.generics.clone();
    let blocks = input.original.blocks.clone();

    Ok((
        input,
        Statement {
            kind: Extends {
                extending,
                extending_generics,
                blocks,
                path: path.clone(),
                items: vec![],
            }
            .into(),
            source: path,
        },
    ))
}

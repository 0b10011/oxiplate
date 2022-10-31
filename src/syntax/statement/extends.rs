use super::super::expression::keyword;
use super::super::Res;
use super::{Statement, StatementKind};
use crate::syntax::Item;
use crate::syntax::expression::Keyword;
use crate::syntax::template::is_whitespace;
use crate::Source;
use nom::bytes::complete::take_while1;
use nom::bytes::complete::{escaped, is_not, tag};
use nom::character::complete::one_of;
use nom::combinator::cut;
use nom::error::context;
use nom::sequence::tuple;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug)]
pub struct Extends<'a> {
    extends_keyword: Keyword<'a>,
    path: Source<'a>,
    items: Vec<Item<'a>>,
}

impl<'a> Extends<'a> {
    pub(crate) fn add_item(&mut self, item: Item<'a>) {
        match item {
            // Comments are fine to keep
            Item::Comment => self.items.push(item),

            // Compile errors must be kept
            Item::CompileError(_, _) => self.items.push(item),

            // Whitespace should be ignored
            Item::Whitespace(_) => (),

            // Block statements are allowed, but other statements should fail
            Item::Statement(Statement { kind: StatementKind::Block(_), ..}) => self.items.push(item),
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
        tokens.append_all(quote! {
            #(#items)*
            #[derive(::oxiplate::Oxiplate)]
            #[oxiplate = include_str!(#path)]
            struct Template<F>
            where
                F: Fn(&mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
            {
                content: F,
            }
            let template = Template {
                content: content,
            };
            write!(f, "{}", template)?;
        });
    }
}

pub(super) fn parse_extends(input: Source) -> Res<Source, Statement> {
    let (input, extends_keyword) = keyword("extends")(input)?;

    let (input, (_, _, path, _)) = cut(tuple((
        context("Expected space after 'extends'", take_while1(is_whitespace)),
        context(r#"Expected ""#, tag(r#"""#)),
        context(
            "Expected path to the template to extend",
            escaped(is_not(r#"""#), '\\', one_of(r#"""#)),
        ),
        context(r#"Expected ""#, tag(r#"""#)),
    )))(input)?;

    Ok((
        input,
        Statement {
            kind: Extends {
                extends_keyword,
                path: path.clone(),
                items: vec![],
            }
            .into(),
            source: path,
        },
    ))
}

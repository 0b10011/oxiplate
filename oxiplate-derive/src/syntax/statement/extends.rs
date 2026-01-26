use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote, quote_spanned};
use syn::LitStr;

use super::{Statement, StatementKind, StaticType};
use crate::syntax::expression::{KeywordParser, String};
use crate::syntax::parser::{Parser as _, cut};
use crate::syntax::template::Template;
use crate::syntax::{Item, Res};
use crate::tokenizer::TokenSlice;
use crate::{BuiltTokens, State};

#[derive(Debug)]
pub struct Extends<'a> {
    blocks: HashMap<&'a str, (Template<'a>, Option<Template<'a>>)>,
    path: String<'a>,
    template: Template<'a>,
}

impl<'a> Extends<'a> {
    pub(crate) fn add_item(&mut self, item: Item<'a>) {
        #[allow(clippy::match_same_arms)]
        match item {
            // Comments are fine to keep
            Item::Comment(_) => self.template.0.push(item),

            // Compile errors must be kept
            Item::CompileError { .. } => self.template.0.push(item),

            // Whitespace should be ignored
            Item::Whitespace(_) => (),
            Item::Static(_, StaticType::Whitespace) => (),

            // Block statements are allowed, but other statements should fail
            Item::Statement(Statement {
                kind: StatementKind::Block(block),
                ..
            }) => {
                self.blocks
                    .insert(block.name.as_str(), (block.prefix, block.suffix));
            }
            Item::Statement(statement) => self.template.0.push(Item::CompileError {
                message: "Only block statements are allowed here, along with comments and \
                          whitespace."
                    .to_owned(),
                error_source: statement.source.clone(),
                consumed_source: statement.source.clone(),
            }),

            // No static text or writs allowed
            Item::Static(text, StaticType::Text) => {
                self.template.0.push(Item::CompileError {
                    message: "Text is not allowed here. Only comments, whitespace, and block \
                              statements are allowed."
                        .to_owned(),
                    error_source: text.1.clone(),
                    consumed_source: text.1.clone(),
                });
            }
            Item::Writ(writ) => {
                self.template.0.push(Item::CompileError {
                    message: "Writs are not allowed here. Only comments, whitespace, and block \
                              statements are allowed."
                        .to_owned(),
                    error_source: writ.source().clone(),
                    consumed_source: writ.source().clone(),
                });
            }
        }
    }

    pub(crate) fn to_tokens<'b: 'a>(&self, state: &mut State<'b>) -> BuiltTokens {
        let span = self.path.source().span_token();
        let path = LitStr::new(self.path.as_str(), span);

        #[cfg(feature = "oxiplate")]
        let oxiplate = quote_spanned! {span=> ::oxiplate::Oxiplate };
        #[cfg(not(feature = "oxiplate"))]
        let oxiplate = quote_spanned! {span=> ::oxiplate_derive::Oxiplate };

        let (template, _template_length) = &self.template.to_tokens(state);
        let mut tokens: TokenStream = quote! { #template };

        let template_to_extend = quote_spanned! {span=>
            #[derive(#oxiplate)]
            #[oxiplate_extends = #path]
            struct Template {}
        };

        let mut block_stack = state.blocks.clone();
        let mut blocks = HashMap::new();
        for (name, block) in &self.blocks {
            state.local_variables.push_stack();
            let prefix = block.0.to_tokens(state);
            state.local_variables.pop_stack();

            state.local_variables.push_stack();
            let suffix = block.1.as_ref().map(|suffix| suffix.to_tokens(state));
            state.local_variables.pop_stack();

            blocks.insert(*name, (prefix, suffix));
        }
        block_stack.push_back(&blocks);

        let (template, estimated_length) =
            crate::oxiplate_internal(template_to_extend.into(), &block_stack);
        let template: TokenStream = template.into();

        tokens.append_all(quote! {
            #template
        });
        (tokens, estimated_length)
    }
}

impl<'a> From<Extends<'a>> for StatementKind<'a> {
    fn from(statement: Extends<'a>) -> Self {
        StatementKind::Extends(statement)
    }
}

pub(super) fn parse_extends(tokens: TokenSlice) -> Res<Statement> {
    let (tokens, (extends_keyword, path)) = (
        KeywordParser::new("extends"),
        cut(
            "Expected string containing path to template to extend",
            String::parse,
        ),
    )
        .parse(tokens)?;

    let source = extends_keyword
        .source()
        .clone()
        .merge(path.source(), "Path expected after `\"`");

    Ok((
        tokens,
        Statement {
            kind: Extends {
                blocks: HashMap::new(),
                path,
                template: Template(vec![]),
            }
            .into(),
            source,
        },
    ))
}

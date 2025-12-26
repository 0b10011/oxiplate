use std::collections::HashSet;

use nom::Parser as _;
use nom::bytes::complete::tag;
use nom::combinator::cut;
use nom::error::context;
use proc_macro::{Diagnostic, Level};
use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote};

use super::super::expression::{Identifier, Keyword, expression, keyword};
use super::super::{Item, Res};
use super::{State, Statement, StatementKind};
use crate::Source;
use crate::syntax::expression::ExpressionAccess;
use crate::syntax::template::{Template, whitespace};

#[derive(Debug)]
pub struct For<'a> {
    #[allow(clippy::struct_field_names)]
    for_keyword: Keyword<'a>,
    ident: Identifier<'a>,
    in_keyword: Keyword<'a>,
    expression: ExpressionAccess<'a>,
    template: Template<'a>,
    otherwise: Option<Template<'a>>,
    pub(super) is_ended: bool,
}

impl<'a> For<'a> {
    pub(crate) fn add_item(&mut self, item: Item<'a>) {
        if self.is_ended {
            Diagnostic::spanned(
                item.source().span().unwrap(),
                Level::Error,
                "Internal Oxiplate error: Attempted to add item to ended `for` statement.",
            )
            .help("Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Attempted+to+add+item+to+ended+for+statement")
            .help("Include template that caused the issue and the associated note.")
            .emit();
            unreachable!("Internal Oxiplate error. See previous error for more information.");
        }

        match item {
            Item::Statement(Statement {
                kind: StatementKind::Else,
                source,
            }) => {
                if let Some(ref mut ifs) = self.otherwise {
                    ifs.0.push(Item::CompileError {
                        message: "`else` previously present in this `for` statement; expected \
                                  `endfor`"
                            .to_string(),
                        error_source: source.clone(),
                        consumed_source: source,
                    });
                } else {
                    self.otherwise = Some(Template(vec![]));
                }
            }
            Item::Statement(Statement {
                kind: StatementKind::EndFor,
                ..
            }) => {
                self.is_ended = true;
            }
            _ => {
                if let Some(otherwise) = &mut self.otherwise {
                    otherwise.0.push(item);
                } else {
                    self.template.0.push(item);
                }
            }
        }
    }

    pub(crate) fn get_active_variables(&self) -> HashSet<&'a str> {
        HashSet::from([self.ident.as_str()])
    }

    pub fn to_tokens(&self, state: &State) -> (TokenStream, usize) {
        let mut tokens = TokenStream::new();
        let mut estimated_length = 0;

        let For {
            for_keyword,
            ident,
            in_keyword,
            expression,
            template,
            otherwise,
            is_ended: _,
        } = self;

        let (expression, _expression_length) = expression.to_tokens(state);

        let mut local_variables = self.get_active_variables();
        for value in state.local_variables {
            local_variables.insert(value);
        }
        let loop_state = &State {
            local_variables: &local_variables,
            ..*state
        };
        let (template, template_length) = template.to_tokens(loop_state);

        // Loops will very likely run at least twice.
        estimated_length += template_length * 2;

        if let Some(otherwise) = otherwise {
            let (otherwise, otherwise_length) = otherwise.to_tokens(state);
            estimated_length = estimated_length.min(otherwise_length);
            tokens.append_all(quote! {
                {
                    let mut loop_ran = false;
                    #for_keyword #ident #in_keyword #expression {
                        loop_ran = true;
                        #template
                    }
                    if !loop_ran {
                        #otherwise
                    }
                }
            });
        } else {
            tokens.append_all(quote! { #for_keyword #ident #in_keyword #expression { #template } });
        }

        (tokens, estimated_length)
    }
}

impl<'a> From<For<'a>> for StatementKind<'a> {
    fn from(statement: For<'a>) -> Self {
        StatementKind::For(statement)
    }
}

pub(super) fn parse_for(input: Source) -> Res<Source, Statement> {
    let (input, for_keyword) = keyword("for").parse(input)?;

    let (input, (for_whitespace, ident, ident_whitespace, in_keyword, in_whitespace, expression)) =
        cut((
            context("Expected space after 'for'", whitespace),
            context("Expected an identifier", Identifier::parse),
            context("Expected space after identifier", whitespace),
            context("Expected 'in'", keyword("in")),
            context("Expected space after 'in'", whitespace),
            context(
                "Expected an expression that is iterable",
                expression(true, true),
            ),
        ))
        .parse(input)?;

    let source = for_keyword
        .0
        .clone()
        .merge(&for_whitespace, "Whitespace expected after `for`")
        .merge(ident.source(), "Ident expected after whitespace")
        .merge(&ident_whitespace, "Whitespace expected after ident")
        .merge(&in_keyword.0, "`in` expected after whitespace")
        .merge(&in_whitespace, "Whitespace expected after `in`")
        .merge(&expression.source(), "Expression expected after whitespace");

    Ok((
        input,
        Statement {
            kind: For {
                for_keyword,
                ident,
                in_keyword,
                expression,
                template: Template(vec![]),
                otherwise: None,
                is_ended: false,
            }
            .into(),
            source,
        },
    ))
}

pub(super) fn parse_endfor(input: Source) -> Res<Source, Statement> {
    let (input, output) = tag("endfor").parse(input)?;

    Ok((
        input,
        Statement {
            kind: StatementKind::EndFor,
            source: output,
        },
    ))
}

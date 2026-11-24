use super::{State, Statement, StatementKind};
use crate::template::parser::Item;
use crate::template::parser::expression::{
    Cow, Expression, Filter, Identifier, Keyword, KeywordParser, PathExprSegment, PathIdentSegment,
    PathInExpression,
};
use crate::template::parser::prelude::*;
use crate::template::parser::template::Template;
use crate::template::parser::writ::{Escaper, EscaperType, Writ};

#[derive(Debug)]
pub(crate) struct FilterBlock<'a> {
    filter_keyword: Keyword<'a>,
    escaper: Escaper<'a>,
    filter_expression: Expression<'a>,
    template: Template<'a>,
    is_ended: bool,
}

impl<'a> FilterBlock<'a> {
    pub(in crate::template::parser) fn parser() -> FilterBlockParser<'a> {
        FilterBlockParser {
            phantom_data: PhantomData,
        }
    }

    pub(in crate::template::parser) fn end_parser() -> EndFilterBlockParser<'a> {
        EndFilterBlockParser {
            phantom_data: PhantomData,
        }
    }

    pub(in crate::template::parser) fn is_ended(&self) -> bool {
        self.is_ended
    }

    pub(crate) fn add_item(&mut self, item: Item<'a>) {
        if self.is_ended {
            internal_error!(
                item.source().span_token().unwrap(),
                "Attempted to add item to ended `filter` statement",
            );
        }

        match item {
            Item::Statement(Statement {
                kind: StatementKind::EndFilter,
                ..
            }) => {
                self.is_ended = true;
            }
            Item::Statement(Statement { kind, source }) if !kind.expected_in_statements() => {
                self.template.0.push(Item::CompileError {
                    message: "Unexpected statement in `filter` statement; is an `endfilter` \
                              statement missing?"
                        .to_string(),
                    error_source: source.clone(),
                    consumed_source: source,
                });
            }
            _ => {
                self.template.0.push(item);
            }
        }
    }
}

impl<'a> ToTokensWithMutState<'a> for FilterBlock<'a> {
    fn to_tokens_with_mut_state<'b: 'a>(&'a self, state: &mut State<'b>) -> BuiltTokens {
        let FilterBlock {
            filter_keyword,
            escaper,
            filter_expression,
            template,
            is_ended: _,
        } = self;

        state.local_variables.push_stack();
        let previous_in_filter_block = state.in_filter_block;
        state.in_filter_block = true;

        let (template, estimated_length) = template.to_tokens_with_mut_state(state);
        let (filter_expression, _estimated_length) = filter_expression.to_tokens_with_state(state);

        state.in_filter_block = previous_in_filter_block;
        state.local_variables.pop_stack();

        let escaper_type: EscaperType = match Writ::escaper_type(Some(escaper), state) {
            Ok(escaper_type) => escaper_type,
            Err(tokens) => return tokens,
        };
        let _ = escaper_type;

        let span = filter_keyword.source().span_token();
        let text = quote_spanned! {span=> {
            // Temporary variable to hold the built template
            // to write to the actual formatter.
            let template_string = {
                // Temporary formatter for the template to write to.
                let mut oxiplate_formatter_owned = alloc::string::String::with_capacity(#estimated_length);
                let oxiplate_formatter = &mut oxiplate_formatter_owned;

                #template

                oxiplate_formatter_owned
            };

            // Run the filters against the built template
            // and write to the formatter.
            #filter_expression
        }};

        let (tokens, estimated_length) = match escaper_type {
            EscaperType::Default => Writ::escaper_default(state, span, &text, estimated_length),
            EscaperType::Specified(group, group_span, escaper) => Writ::escaper_specified(
                state,
                &group,
                group_span,
                escaper,
                span,
                &text,
                estimated_length,
            ),
            EscaperType::Raw => Writ::escaper_raw(&text, estimated_length),
        };

        (quote_spanned! {span=> #tokens; }, estimated_length)
    }
}

impl<'a> From<FilterBlock<'a>> for StatementKind<'a> {
    fn from(statement: FilterBlock<'a>) -> Self {
        StatementKind::Filter(statement)
    }
}

pub(in crate::template::parser) struct FilterBlockParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for FilterBlockParser<'a> {
    type TokenKind = TokenKind;
    type Output = Statement<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, (filter_keyword, escaper_group, escaper, expression_callbacks)): (
            _,
            (_, Option<(_, Source)>, _, _),
        ) = (
            KeywordParser::new("filter"),
            ignore_recoverable_errors((Identifier::parser(), into(take(TokenKind::Period)))),
            cut("Expected escaper or escaper group", Identifier::parser()),
            cut("Expected filters", many1(Filter::parser())),
        )
            .parse(tokens)?;

        let escaper_source = if let Some((escaper_group, dot)) = &escaper_group {
            escaper_group
                .source()
                .clone()
                .merge(dot, "Dot expected after escaper group")
                .merge(escaper.source(), "Escaper name expected after group")
        } else {
            escaper.source().clone()
        };

        let template_string_source = escaper_source;

        // Build `oxiplate_formatter` as an expression
        // and wrap it in the filter calls.
        let mut filter_expression = Expression::Cow(Cow::new(
            filter_keyword.source().to_owned(),
            Box::new(Expression::Path(PathInExpression::new(
                None,
                PathExprSegment::new(
                    PathIdentSegment::TemplateString(template_string_source),
                    None,
                ),
                vec![],
            ))),
        ));
        for callback in expression_callbacks {
            filter_expression = callback(filter_expression);
        }

        filter_expression.fix_precedence();

        filter_expression.merge_joinable();

        let escaper = Escaper::new(
            escaper_group.map(|(escaper_group, _dot)| escaper_group),
            escaper,
        );

        let source = filter_expression.source();

        Ok((
            tokens,
            Statement {
                kind: FilterBlock {
                    filter_keyword,
                    escaper,
                    filter_expression,
                    template: Template(Vec::new()),
                    is_ended: false,
                }
                .into(),
                source,
            },
        ))
    }
}

pub(in crate::template::parser) struct EndFilterBlockParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for EndFilterBlockParser<'a> {
    type TokenKind = TokenKind;
    type Output = Statement<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, output) = KeywordParser::new("endfilter").parse(tokens)?;

        Ok((
            tokens,
            Statement {
                kind: StatementKind::EndFilter,
                source: output.source().clone(),
            },
        ))
    }
}

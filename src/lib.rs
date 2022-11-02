#![feature(proc_macro_diagnostic)]
#![feature(proc_macro_expand)]
#![doc(issue_tracker_base_url = "https://github.com/0b10011/Oxiplate/issues/")]
#![doc(test(no_crate_inject))]
#![doc(test(attr(deny(warnings))))]
#![doc = include_str!("../README.md")]

mod syntax;

use nom::Compare;
use nom::InputIter;
use nom::InputLength;
use nom::InputTake;
use nom::Needed;
use nom::Offset;
use nom::Slice;
use nom::UnspecializedInput;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::Literal;
use proc_macro2::Span;
use quote::quote;
use quote::ToTokens;
use std::ops::Range;
use std::ops::RangeFrom;
use std::ops::RangeTo;
use std::path::PathBuf;
use std::str::CharIndices;
use std::str::Chars;
use syn::spanned::Spanned;
use syn::{Attribute, Data, DeriveInput, Fields};

#[derive(Debug)]
pub(crate) struct SourceOwned {
    ident: Ident,
    blocks: Vec<String>,
    code: String,
    literal: Literal,
    span_hygiene: Span,
    origin: Option<PathBuf>,
    is_extending: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct Source<'a> {
    original: &'a SourceOwned,
    range: Range<usize>,
}

impl<'a> Source<'a> {
    pub fn as_str(&self) -> &'a str {
        &self.original.code[self.range.clone()]
    }

    pub fn span(&self) -> Span {
        let mut start = self.range.start;
        let end = self.range.end;
        if start == end && start > 1 {
            start -= 1;
        }
        self.original
            .literal
            .subspan(Range {
                start: start + 1,
                end: end + 1,
            })
            .unwrap_or_else(proc_macro2::Span::call_site)
            .resolved_at(self.original.span_hygiene)
    }
}

impl<'a> Slice<RangeFrom<usize>> for Source<'a> {
    fn slice(&self, new_range: RangeFrom<usize>) -> Self {
        Source {
            original: self.original,
            range: Range {
                start: self.range.start + new_range.start,
                end: self.range.end,
            },
        }
    }
}

impl<'a> Slice<RangeTo<usize>> for Source<'a> {
    fn slice(&self, new_range: RangeTo<usize>) -> Self {
        Source {
            original: self.original,
            range: Range {
                start: self.range.start,
                end: self.range.start + new_range.end,
            },
        }
    }
}

impl<'a> ToTokens for Source<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let text = self.as_str();
        let span = self
            .original
            .literal
            .subspan(self.range.clone())
            .expect("No subspan found");
        tokens.append_all(quote::quote_spanned! {span=> write!(f, "{}", #text)?;});
    }
}

impl<'a> PartialEq<Source<'a>> for Source<'a> {
    fn eq(&self, other: &Source) -> bool {
        self.range == other.range
            && self.original.origin == other.original.origin
            && self.original.code == other.original.code
    }
}

impl<'a> Eq for Source<'a> {}

impl<'a> PartialEq<char> for Source<'a> {
    fn eq(&self, char: &char) -> bool {
        self.as_str().len() == 1 && char == &self.as_str().chars().next().unwrap()
    }
}

impl<'a> Compare<&Source<'a>> for Source<'a> {
    fn compare(&self, other_source: &Source) -> nom::CompareResult {
        self.as_str().compare(other_source.as_str())
    }

    fn compare_no_case(&self, other_source: &Source) -> nom::CompareResult {
        self.as_str().compare_no_case(other_source.as_str())
    }
}

impl<'a> Compare<&str> for Source<'a> {
    fn compare(&self, string: &str) -> nom::CompareResult {
        self.as_str().compare(string)
    }

    fn compare_no_case(&self, string: &str) -> nom::CompareResult {
        self.as_str().compare_no_case(string)
    }
}

impl<'a> InputIter for Source<'a> {
    type Item = char;
    type Iter = CharIndices<'a>;
    type IterElem = Chars<'a>;

    #[inline]
    fn iter_indices(&self) -> Self::Iter {
        self.as_str().iter_indices()
    }

    #[inline]
    fn iter_elements(&self) -> Self::IterElem {
        self.as_str().iter_elements()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.as_str().position(predicate)
    }

    #[inline]
    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        self.as_str().slice_index(count)
    }
}

impl<'a> InputTake for Source<'a> {
    #[inline]
    fn take(&self, count: usize) -> Self {
        let end = self.range.start + count;
        if end > self.range.end {
            panic!("End greater than end of string");
        }
        Source {
            original: self.original,
            range: Range {
                start: self.range.start,
                end,
            },
        }
    }

    // return byte index
    #[inline]
    fn take_split(&self, count: usize) -> (Self, Self) {
        let end = self.range.start + count;
        if end > self.range.end {
            panic!("End greater than end of string");
        }

        (
            Source {
                original: self.original,
                range: Range {
                    start: end,
                    end: self.range.end,
                },
            },
            Source {
                original: self.original,
                range: Range {
                    start: self.range.start,
                    end,
                },
            },
        )
    }
}

impl<'a> InputLength for Source<'a> {
    fn input_len(&self) -> usize {
        self.as_str().input_len()
    }
}

impl<'a> InputLength for &Source<'a> {
    fn input_len(&self) -> usize {
        self.as_str().input_len()
    }
}

impl<'a> Offset for Source<'a> {
    fn offset(&self, offset: &Self) -> usize {
        self.as_str().offset(offset.as_str())
    }
}

impl<'a> UnspecializedInput for Source<'a> {}

impl<'a> Iterator for Source<'a> {
    type Item = Source<'a>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        todo!()
    }
}

#[proc_macro_derive(Oxiplate, attributes(oxiplate, oxiplate_extends))]
pub fn oxiplate(input: TokenStream) -> TokenStream {
    match parse(input) {
        Ok(token_stream) => token_stream,
        Err(err) => err.to_compile_error().into(),
    }
}

fn parse(input: TokenStream) -> Result<TokenStream, syn::Error> {
    let input = syn::parse(input).unwrap();
    let DeriveInput {
        attrs,
        ident,
        data,
        generics,
        ..
    } = &input;

    let mut field_names: Vec<&syn::Ident> = Vec::new();
    match data {
        Data::Struct(ref struct_item) => {
            if let Fields::Named(fields) = &struct_item.fields {
                for field in &fields.named {
                    match &field.ident {
                        Some(name) => field_names.push(name),
                        None => field.span().unwrap().error("Expected a named field").emit(),
                    }
                }
            }
        }
        _ => {
            return Err(syn::Error::new(input.span(), "Expected a struct"));
        }
    };

    let source = get_source(ident, data, attrs)?;
    let source = Source {
        original: &source,
        range: Range {
            start: 0,
            end: source.code.len(),
        },
    };
    let template = syntax::parse(source);

    let where_clause = &generics.where_clause;
    let expanded = quote! {
        impl #generics std::fmt::Display for #ident #generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #template
                Ok(())
            }
        }
    };

    Ok(TokenStream::from(expanded))
}

fn get_source(
    ident: &Ident,
    data: &Data,
    attrs: &Vec<Attribute>,
) -> Result<SourceOwned, syn::Error> {
    let invalid_attribute_message = r#"Must provide either an external or internal template:
External: #[oxiplate = include_str!("./relative/path/to/template/from/current/file.txt.oxip")]
Internal: #[oxiplate = "{{ your_var }}"]"#;
    for attr in attrs {
        let is_extending = attr.path.is_ident("oxiplate_extends");
        if attr.path.is_ident("oxiplate") || is_extending {
            // Parse out the `=` and expression to it can be expanded.
            let parser = |input: syn::parse::ParseStream| {
                input.parse::<syn::Token![=]>()?;
                input.parse::<syn::Expr>()
            };
            let input = syn::parse::Parser::parse2(parser, attr.tokens.clone())?;

            // Change the `syn::Expr` into a `proc_macro2::TokenStream`
            let span = input.span();
            let input = quote::quote_spanned!(span=> #input);

            // Change the `proc_macro2::TokenStream` to a `proc_macro::TokenStream`
            let input = proc_macro::TokenStream::from(input);

            // Expand any macros, or fallback to the unexpanded input
            let input = input.expand_expr();
            if input.is_err() {
                return Err(syn::Error::new(span, invalid_attribute_message));
            }
            let input = input.unwrap();

            // Parse the string and token out of the expanded expression
            let parser = |input: syn::parse::ParseStream| input.parse::<syn::Lit>();
            let (code, literal) = match syn::parse::Parser::parse(parser, input)? {
                syn::Lit::Str(code) => (code.value(), code.token()),
                _ => Err(syn::Error::new(attr.span(), invalid_attribute_message))?,
            };

            let mut blocks = vec![];
            if is_extending {
                match data {
                    Data::Struct(ref struct_item) => {
                        if let Fields::Named(fields) = &struct_item.fields {
                            for field in &fields.named {
                                match &field.ident {
                                    Some(name) => {
                                        if name.to_string() != "_data"
                                            && name.to_string() != "_blocks"
                                        {
                                            blocks.push(name.to_string());
                                        }
                                    }
                                    None => {
                                        field.span().unwrap().error("Expected a named field").emit()
                                    }
                                }
                            }
                        }
                    }
                    _ => unreachable!("Should have checked this doesn't happen already"),
                }
            }

            // Return the source
            return Ok(SourceOwned {
                ident: ident.clone(),
                blocks,
                code,
                literal,
                span_hygiene: span,
                origin: None,
                is_extending,
            });
        }
    }

    unimplemented!();
}

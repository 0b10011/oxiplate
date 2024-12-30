#![feature(proc_macro_diagnostic)]
#![feature(proc_macro_expand)]
#![doc(issue_tracker_base_url = "https://github.com/0b10011/Oxiplate/issues/")]
#![doc(test(no_crate_inject))]
#![doc(test(attr(deny(warnings))))]
#![doc = include_str!("../README.md")]
// Clippy groups
#![warn(clippy::cargo, clippy::pedantic)]
// Clippy rules
#![allow(
    // rustfmt doesn't format `assert!()` :(
    clippy::manual_assert,
)]

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
use proc_macro2::Literal;
use proc_macro2::Span;
use quote::quote;
use quote::ToTokens;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fmt;
use std::fs;
use std::iter::Enumerate;
use std::ops::Range;
use std::ops::RangeFrom;
use std::ops::RangeTo;
use std::path;
use std::path::PathBuf;
use std::str::CharIndices;
use std::str::Chars;
use syn::spanned::Spanned;
use syn::Expr;
use syn::ExprLit;
use syn::Lit;
use syn::MetaNameValue;
use syn::Type;
use syn::{Attribute, Data, DeriveInput, Fields};

pub(crate) struct SourceOwned {
    data_type: Type,
    blocks: Vec<String>,
    code: String,
    literal: Literal,
    span_hygiene: Span,
    origin: Option<PathBuf>,
    is_extending: bool,
}

impl fmt::Debug for SourceOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SourceOwned")
            // .field("ident", &"UNSUPPORTED_SORRY")
            .field("blocks", &self.blocks)
            .field("code", &self.code)
            .field("literal", &self.literal)
            .field("span_hygiene", &self.span_hygiene)
            .field("origin", &self.origin)
            .field("is_extending", &self.is_extending)
            .finish_non_exhaustive()
    }
}

pub(crate) struct State<'a> {
    local_variables: &'a HashSet<&'a str>,
    config: &'a Config,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct EscaperGroup {
    escaper: String,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    #[serde(default)]
    default_escaper_group: Option<String>,
    #[serde(default)]
    escaper_groups: HashMap<String, EscaperGroup>,
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

        // Customize the range to map properly to the literal.
        let mut range = Range { start, end };

        if self.original.origin.is_some() {
            return self
                .original
                .literal
                .subspan(range)
                .unwrap_or_else(proc_macro2::Span::call_site)
                .resolved_at(self.original.span_hygiene);
        }

        let literal = format!("{}", self.original.literal);
        let mut chars = literal.chars().enumerate();

        let hash_count = Self::parse_open(&mut chars, &mut range);
        Self::parse_interior(&mut chars, &mut range, hash_count);

        self.original
            .literal
            .subspan(range)
            .unwrap_or_else(proc_macro2::Span::call_site)
            .resolved_at(self.original.span_hygiene)
    }

    fn update_range(range: &mut Range<usize>, pos: usize) {
        if range.start >= pos {
            range.start += 1;
        }
        if range.end >= pos {
            range.end += 1;
        }
    }

    fn parse_open(chars: &mut Enumerate<Chars<'_>>, range: &mut Range<usize>) -> Option<usize> {
        let (pos, char) = chars.next().expect("Unexpected end of string");
        match char {
            'r' => (),
            '"' => {
                Self::update_range(range, pos);
                return None;
            }
            _ => panic!("Expected 'r' or '\"', found: {char}"),
        }

        Self::update_range(range, pos);

        let mut hash_count = 0;
        for (pos, char) in chars.by_ref() {
            match char {
                '#' => hash_count += 1,
                '"' => {
                    Self::update_range(range, pos);
                    break;
                }
                _ => panic!("Expected '#' or '\"'; found: {char}"),
            }
            Self::update_range(range, pos);
        }

        Some(hash_count)
    }

    fn parse_ascii_escape(chars: &mut Enumerate<Chars<'_>>) {
        // https://doc.rust-lang.org/reference/tokens.html#ascii-escapes
        // Up to 0x7F
        match chars.next().expect("Unexpected end of string") {
            (_pos, '0'..='7') => (),
            (_pos, char) => panic!("Expected [0-7]; found: {char}"),
        }
        match chars.next().expect("Unexpected end of string") {
            (_pos, '0'..='9' | 'a'..='f' | 'A'..='F') => (),
            (_pos, char) => panic!("Expected [0-9a-f]; found: {char}"),
        }
    }

    fn parse_unicode_escape(chars: &mut Enumerate<Chars<'_>>, range: &mut Range<usize>) {
        let mut unicode_chars_parsed = -1;
        let mut unicode_code = String::new();
        loop {
            let (pos, char) = chars.next().expect("Unexpected end of string");
            Self::update_range(range, pos);
            match (unicode_chars_parsed, char) {
                (-1, '{') => {
                    unicode_chars_parsed += 1;
                    continue;
                }
                (0..=3, '0'..='9' | 'a'..='f' | 'A'..='F') => {
                    unicode_chars_parsed += 1;
                    unicode_code = format!("{unicode_code}{char}");
                    continue;
                }
                (1..=4, '}') => {
                    let code = u32::from_str_radix(&unicode_code, 16).expect("Should be a u32");
                    let char = char::from_u32(code).expect("Should be a unicode char");
                    let byte_count = char.to_string().len();
                    if range.start >= pos {
                        range.start -= byte_count - 1;
                    }
                    if range.end >= pos {
                        range.end -= byte_count - 1;
                    }
                    return;
                }
                (-1, _) => panic!("Expected {}; found {char}", '{'),
                (0, _) => panic!("Expected a hex character (0-9a-f)]; found {char}"),
                (1..=3, _) => panic!(
                    "Expected a hex character (0-9a-f) or {}]; found {char}",
                    '{'
                ),
                (4, _) => panic!("Expected {}; found {char}", '}'),
                (_, _) => unreachable!(
                    "All possible cases should be covered; found {} with count {}",
                    char, unicode_chars_parsed
                ),
            }
        }
    }

    fn parse_escape(chars: &mut Enumerate<Chars<'_>>, range: &mut Range<usize>) {
        let (_pos, char) = chars.next().expect("Unexpected end of string");
        match char {
            // https://doc.rust-lang.org/reference/tokens.html#quote-escapes
            // https://doc.rust-lang.org/reference/tokens.html#ascii-escapes
            '\'' | '"' | 'n' | 'r' | 't' | '\\' | '0' => (),
            // https://doc.rust-lang.org/reference/tokens.html#ascii-escapes
            'x' => Self::parse_ascii_escape(chars),
            // https://doc.rust-lang.org/reference/tokens.html#unicode-escapes
            'u' => Self::parse_unicode_escape(chars, range),
            _ => panic!(
                "Expected ', \", n, r, t, \\, 0, x, or {}; found: {}",
                '{', char
            ),
        }
    }

    fn parse_interior(
        chars: &mut Enumerate<Chars<'_>>,
        range: &mut Range<usize>,
        hash_count: Option<usize>,
    ) {
        while let Some((pos, char)) = chars.next() {
            match (char, hash_count) {
                ('"', _) => return,
                ('\\', None) => {
                    Self::update_range(range, pos);
                    Self::parse_escape(chars, range);
                }
                _ => (),
            }
        }
    }
}

impl Slice<RangeFrom<usize>> for Source<'_> {
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

impl Slice<RangeTo<usize>> for Source<'_> {
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

impl ToTokens for Source<'_> {
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

impl Eq for Source<'_> {}

impl PartialEq<char> for Source<'_> {
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

impl Compare<&str> for Source<'_> {
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

impl InputTake for Source<'_> {
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

impl InputLength for Source<'_> {
    fn input_len(&self) -> usize {
        self.as_str().input_len()
    }
}

impl InputLength for &Source<'_> {
    fn input_len(&self) -> usize {
        self.as_str().input_len()
    }
}

impl Offset for Source<'_> {
    fn offset(&self, offset: &Self) -> usize {
        self.as_str().offset(offset.as_str())
    }
}

impl UnspecializedInput for Source<'_> {}

impl<'a> Iterator for Source<'a> {
    type Item = Source<'a>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        todo!()
    }
}

#[proc_macro_derive(Oxiplate, attributes(oxiplate, oxiplate_inline, oxiplate_extends))]
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

    let data_type = quote! { #ident #generics };
    let config_path =
        path::Path::new(&env::current_dir().expect("Failed to get current directory"))
            .join("oxiplate.toml");
    let config: Config = if let Ok(toml) = fs::read_to_string(config_path.clone()) {
        toml::from_str(&toml).expect("Failed to parse oxiplate.toml")
    } else {
        Config {
            default_escaper_group: None,
            escaper_groups: HashMap::new(),
        }
    };
    let state = State {
        local_variables: &HashSet::new(),
        config: &config,
    };
    let source = parse_attributes(syn::parse2(data_type)?, data, attrs)?;
    let source = Source {
        original: &source,
        range: Range {
            start: 0,
            end: source.code.len(),
        },
    };
    let template = syntax::parse(&state, source);

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

fn parse_attributes(
    data_type: Type,
    data: &Data,
    attrs: &Vec<Attribute>,
) -> Result<SourceOwned, syn::Error> {
    let invalid_attribute_message = r#"Must provide either an external or internal template:
External: #[oxiplate = "/path/to/template/from/templates/directory.txt.oxip"]
Internal: #[oxiplate_inline = "{{ your_var }}"]"#;
    for attr in attrs {
        let is_inline = attr.path().is_ident("oxiplate_inline");
        let is_extending = attr.path().is_ident("oxiplate_extends");
        if attr.path().is_ident("oxiplate") || is_inline || is_extending {
            return parse_attribute(
                data_type,
                data,
                attr,
                is_inline,
                is_extending,
                invalid_attribute_message,
            );
        }
    }

    unimplemented!();
}

fn parse_attribute(
    mut data_type: Type,
    data: &Data,
    attr: &Attribute,
    is_inline: bool,
    is_extending: bool,
    invalid_attribute_message: &str,
) -> Result<SourceOwned, syn::Error> {
    let (span, input, origin) = parse_source_tokens(attr, is_inline, is_extending);

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
                                if *name == "_data" {
                                    data_type = field.ty.clone();
                                } else {
                                    blocks.push(name.to_string());
                                }
                            }
                            None => {
                                field.span().unwrap().error("Expected a named field").emit();
                            }
                        }
                    }
                }
            }
            _ => unreachable!("Should have checked this doesn't happen already"),
        }
    }

    // Return the source
    Ok(SourceOwned {
        data_type,
        blocks,
        code,
        literal,
        span_hygiene: span,
        origin,
        is_extending,
    })
}

fn parse_source_tokens(
    attr: &Attribute,
    is_inline: bool,
    is_extending: bool,
) -> (Span, proc_macro2::TokenStream, Option<PathBuf>) {
    if is_inline || is_extending {
        let syn::Meta::NameValue(MetaNameValue {
            path: _,
            eq_token: _,
            value: input,
        }) = attr.meta.clone()
        else {
            todo!("need to handle when non-name-value data is provided");
        };
        // Change the `syn::Expr` into a `proc_macro2::TokenStream`
        let span = input.span();
        return (span, quote::quote_spanned!(span=> #input), None);
    }

    let syn::Meta::NameValue(MetaNameValue {
        path: _,
        eq_token: _,
        value: Expr::Lit(ExprLit {
            attrs: _,
            lit: Lit::Str(path),
        }),
    }) = attr.meta.clone()
    else {
        todo!("need to handle when non-name-value data is provided");
    };
    let templates_dir = PathBuf::from(option_env!("OXIP_TEMPLATE_DIR").unwrap_or("templates"));
    let root = PathBuf::from(
        ::std::env::var("CARGO_MANIFEST_DIR_OVERRIDE")
            .or(::std::env::var("CARGO_MANIFEST_DIR"))
            .unwrap(),
    );

    // Path::join() doesn't play well with absolute paths (for our use case).
    let templates_dir_root = root.join(templates_dir.clone());
    if !templates_dir_root.starts_with(root) {
        panic!("OXIP_TEMPLATE_DIR must be a relative path; example: 'templates' instead of '/templates'. Provided: {}", templates_dir.display());
    }

    // Path::join() doesn't play well with absolute paths (for our use case).
    let full_path = templates_dir_root.join(path.value());
    if !full_path.starts_with(templates_dir_root) {
        panic!("Template path must be a relative path; example 'template.oxip' instead of '/template.oxip'. Provided: {}", path.value());
    }
    let span = path.span();
    let path = syn::LitStr::new(&full_path.to_string_lossy(), span);

    // Change the `syn::Expr` into a `proc_macro2::TokenStream`
    (
        span,
        quote::quote_spanned!(span=> include_str!(#path)),
        Some(full_path),
    )
}

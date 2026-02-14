#[cfg(feature = "config")]
mod parser;
#[cfg(feature = "config")]
mod tokenizer;

use std::collections::HashMap;
use std::env;
#[cfg(feature = "config")]
use std::fs;
use std::ops::Deref;
use std::path::PathBuf;

#[cfg(feature = "config")]
use proc_macro2::Span;
#[cfg(feature = "config")]
use syn::LitStr;

#[cfg(feature = "config")]
use self::tokenizer::TokenKind;
#[cfg(feature = "config")]
use crate::parser::Error;
#[cfg(feature = "config")]
use crate::{Source, SourceOwned};

#[cfg(feature = "config")]
type Token<'a> = crate::tokenizer::Token<'a, TokenKind>;
#[cfg(feature = "config")]
type TokenSlice<'a> = crate::tokenizer::TokenSlice<'a, TokenKind>;

#[cfg(not(feature = "config"))]
pub fn read_config() -> Result<Config, syn::Error> {
    Ok(Config::default())
}

/// Read the user-defined `/oxiplate.toml` if possible,
/// otherwise generate a default `Config`.
#[cfg(feature = "config")]
pub fn read_config() -> Result<Config, syn::Error> {
    use crate::config::parser::parse;
    use crate::config::tokenizer::tokens_and_eof;
    use crate::tokenizer::TokenSlice;

    let path = config_path();
    let code = if let Ok(toml) = fs::read_to_string(path.clone()) {
        LitStr::new(&toml, Span::mixed_site())
    } else {
        return Ok(Config::default());
    };

    let span = Span::mixed_site();
    let origin = Some(path);
    let owned_source = SourceOwned::new(&code, span, origin);
    let source = Source::new(&owned_source);

    let (tokens, eof) = tokens_and_eof(source);
    let tokens = TokenSlice::new(&tokens, &eof);
    let (_tokens, config) = parse(tokens).map_err(|err| convert_error(&err))?;
    Ok(config)
}

#[cfg(feature = "config")]
fn convert_error(error: &Error) -> syn::Error {
    match error {
        Error::Recoverable { message, .. } | Error::Unrecoverable { message, .. } => {
            syn::Error::new(
                Span::mixed_site(),
                format!("Failed to parse `/oxiplate.toml`: {message}"),
            )
        }
        Error::Multiple(errors) => {
            let Some(error) = errors.first() else {
                unreachable!("`Error::Multiple` should always contain at least one error");
            };

            convert_error(error)
        }
    }
}

/// Build the path to the user-defined `/oxiplate.toml`.
pub fn config_path() -> PathBuf {
    let root = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR_OVERRIDE")
            .or(env::var("CARGO_MANIFEST_DIR"))
            .expect("`CARGO_MANFIEST_DIR` should be present for setting up state"),
    );
    root.join("oxiplate.toml")
}

/// Macro configuration.
#[cfg_attr(not(feature = "_unreachable"), derive(Default))]
pub(crate) struct Config {
    /// The escaper group to use
    /// when one cannot be inferred from the template's file extension.
    pub(crate) fallback_escaper_group: Option<String>,

    /// List of valid escaper groups,
    /// with the key being the name that is used in templates
    /// and the value being a path to the enum.
    #[cfg_attr(not(feature = "_oxiplate"), allow(dead_code))]
    pub(crate) escaper_groups: HashMap<String, EscaperGroup>,

    /// Whether to require escapers to be explicitly specified,
    /// or to fallback to the default escaper of a group.
    pub(crate) require_specifying_escaper: bool,

    /// Whether to attempt to infer the escaper group
    /// from the template's file extension.
    #[cfg_attr(not(feature = "_oxiplate"), allow(dead_code))]
    pub(crate) infer_escaper_group_from_file_extension: InferEscaperGroupFromFileExtension,

    #[cfg_attr(not(feature = "_oxiplate"), allow(dead_code))]
    pub(crate) optimized_renderer: OptimizedRenderer,
}

#[cfg(feature = "_unreachable")]
impl Default for Config {
    fn default() -> Self {
        Self {
            fallback_escaper_group: Some("raw".to_string()),
            escaper_groups: HashMap::default(),
            require_specifying_escaper: Default::default(),
            infer_escaper_group_from_file_extension: InferEscaperGroupFromFileExtension::default(),
            optimized_renderer: OptimizedRenderer::default(),
        }
    }
}

/// Escaper group defined in the configuration.
#[cfg_attr(not(feature = "_oxiplate"), allow(dead_code))]
#[derive(Clone)]
pub(crate) struct EscaperGroup {
    pub(crate) escaper: String,
}

pub(crate) struct InferEscaperGroupFromFileExtension(bool);

impl Default for InferEscaperGroupFromFileExtension {
    fn default() -> Self {
        Self(true)
    }
}

impl Deref for InferEscaperGroupFromFileExtension {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
pub(crate) struct OptimizedRenderer(bool);

impl OptimizedRenderer {
    pub(super) fn unoptimized() -> Self {
        OptimizedRenderer(false)
    }
}

impl Default for OptimizedRenderer {
    fn default() -> Self {
        #[cfg(feature = "_oxiplate")]
        return Self(true);

        #[cfg(not(feature = "_oxiplate"))]
        Self(false)
    }
}

impl Deref for OptimizedRenderer {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

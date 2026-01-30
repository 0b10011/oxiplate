mod tokenizer;

use std::collections::HashMap;
use std::ops::Deref;
use std::path::PathBuf;
use std::{env, fs};

use proc_macro2::Span;
use syn::LitStr;

use self::tokenizer::TokenKind;
use crate::{Source, SourceOwned};

type Token<'a> = crate::tokenizer::Token<'a, TokenKind>;
type TokenSlice<'a> = crate::tokenizer::TokenSlice<'a, TokenKind>;

mod parser;

#[cfg(not(feature = "config"))]
pub fn read_config(_input: &DeriveInput) -> Result<Config, syn::Error> {
    Ok(Config::default())
}

/// Read the user-defined `/oxiplate.toml` if possible,
/// otherwise generate a default `Config`.
#[cfg(feature = "config")]
pub fn read_config() -> Result<Config, syn::Error> {
    use crate::config::parser::parse;
    use crate::config::tokenizer::tokens_and_eof;
    use crate::parser::Error;
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
    let (_tokens, config) = parse(tokens).map_err(|err| {
        syn::Error::new(
            Span::mixed_site(),
            match err {
                Error::Recoverable { message, .. } | Error::Unrecoverable { message, .. } => {
                    format!("Error parsing `/oxiplate.toml`: {message}")
                }
                Error::Multiple { .. } => "Error parsing `/oxiplate.toml`".to_string(),
            },
        )
    })?;
    Ok(config)
}

/// Build the path to the user-defined `/oxiplate.toml`.
fn config_path() -> PathBuf {
    let root = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR_OVERRIDE")
            .or(env::var("CARGO_MANIFEST_DIR"))
            .expect("`CARGO_MANFIEST_DIR` should be present for setting up state"),
    );
    root.join("oxiplate.toml")
}

/// Macro configuration.
#[cfg_attr(not(feature = "unreachable"), derive(Default))]
pub(crate) struct Config {
    /// The escaper group to use
    /// when one cannot be inferred from the template's file extension.
    pub(crate) fallback_escaper_group: Option<String>,

    /// List of valid escaper groups,
    /// with the key being the name that is used in templates
    /// and the value being a path to the enum.
    #[cfg_attr(not(feature = "oxiplate"), allow(dead_code))]
    pub(crate) escaper_groups: HashMap<String, EscaperGroup>,

    /// Whether to require escapers to be explicitly specified,
    /// or to fallback to the default escaper of a group.
    pub(crate) require_specifying_escaper: bool,

    /// Whether to attempt to infer the escaper group
    /// from the template's file extension.
    #[cfg_attr(not(feature = "oxiplate"), allow(dead_code))]
    pub(crate) infer_escaper_group_from_file_extension: InferEscaperGroupFromFileExtension,

    #[cfg_attr(not(feature = "oxiplate"), allow(dead_code))]
    pub(crate) optimized_renderer: OptimizedRenderer,
}

#[cfg(feature = "unreachable")]
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
#[cfg_attr(not(feature = "oxiplate"), allow(dead_code))]
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
        #[cfg(feature = "oxiplate")]
        return Self(true);

        #[cfg(not(feature = "oxiplate"))]
        Self(false)
    }
}

impl Deref for OptimizedRenderer {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

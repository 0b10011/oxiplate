use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::{env, fs};

use proc_macro2::TokenStream;
#[cfg(feature = "config")]
use serde::Deserialize;
use syn::spanned::Spanned;
use syn::DeriveInput;

#[cfg(all(feature = "built-in-escapers", not(feature = "oxiplate")))]
compile_error!(
    "The `built-in-escapers` feature only works when using the `oxiplate` library rather than \
     `oxiplate-derive` directly."
);

/// Build a `Config` from Oxiplate's defaults
/// and the user-defined `/oxiplate.toml`.
pub(crate) fn build_config(input: &DeriveInput) -> Result<Config, syn::Error> {
    #[cfg(not(feature = "config"))]
    if fs::exists(config_path()).unwrap_or(false) {
        return Err(syn::Error::new(
            input.span(),
            r#"`/oxiplate.toml` exists, but the "config" feature is turned off. Either delete/rename `/oxiplate.toml`, or turn the "config" feature on."#,
        ));
    }

    #[cfg(not(feature = "built-in-escapers"))]
    return read_config(input);

    #[cfg(feature = "built-in-escapers")]
    read_config_and_add_built_in_escapers(input)
}

/// Build a `Config` from the user-defined `/oxiplate.toml`
/// and add all built-in escapers that have not been overridden.
#[cfg(feature = "built-in-escapers")]
fn read_config_and_add_built_in_escapers(input: &DeriveInput) -> Result<Config, syn::Error> {
    let mut config = read_config(input)?;

    // Add built-in escapers to the user-defined escapers
    let built_in_escapers = [
        ("html", "::oxiplate::escapers::html::HtmlEscaper"),
        ("md", "::oxiplate::escapers::markdown::MarkdownEscaper"),
        ("json", "::oxiplate::escapers::json::JsonEscaper"),
    ];
    for (name, path) in built_in_escapers {
        if config.escaper_groups.contains_key(name) {
            continue;
        }

        config.escaper_groups.insert(
            name.to_string(),
            EscaperGroup {
                escaper: path.to_string(),
            },
        );
    }

    Ok(config)
}

/// Generate a default `Config`.
#[cfg(not(feature = "config"))]
fn read_config(_input: &DeriveInput) -> Result<Config, syn::Error> {
    Ok(Config::default())
}

/// Read the user-defined `/oxiplate.toml` if possible,
/// otherwise generate a default `Config`.
#[cfg(feature = "config")]
fn read_config(input: &DeriveInput) -> Result<Config, syn::Error> {
    if let Ok(toml) = fs::read_to_string(config_path().clone()) {
        toml::from_str(&toml).map_err(|error| {
            syn::Error::new(
                input.span(),
                format!("Failed to parse `/oxiplate.toml`: {}", error.message()),
            )
        })
    } else {
        Ok(Config::default())
    }
}

/// Build the path to the user-defined `/oxiplate.toml`.
fn config_path() -> PathBuf {
    let root = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR_OVERRIDE")
            .or(env::var("CARGO_MANIFEST_DIR"))
            .unwrap(),
    );
    root.join("oxiplate.toml")
}

/// Macro state containing the configuration and any local variables.
pub(crate) struct State<'a> {
    pub(crate) local_variables: &'a HashSet<&'a str>,
    pub(crate) config: &'a Config,
    pub(crate) inferred_escaper_group: Option<&'a EscaperGroup>,
    pub(crate) blocks: &'a HashMap<&'a str, (TokenStream, Option<TokenStream>)>,
}

/// Macro configuration.
#[cfg_attr(feature = "config", derive(Deserialize))]
#[cfg_attr(feature = "config", serde(deny_unknown_fields))]
pub(crate) struct Config {
    /// The escaper group to use
    /// when one cannot be inferred from the template's file extension.
    #[cfg_attr(feature = "config", serde(default))]
    pub(crate) fallback_escaper_group: Option<String>,

    /// List of valid escaper groups,
    /// with the key being the name that is used in templates
    /// and the value being a path to the enum.
    #[cfg_attr(not(feature = "oxiplate"), allow(dead_code))]
    #[cfg_attr(feature = "config", serde(default))]
    pub(crate) escaper_groups: HashMap<String, EscaperGroup>,

    /// Whether to require escapers to be explicitly specified,
    /// or to fallback to the default escaper of a group.
    #[cfg_attr(feature = "config", serde(default))]
    pub(crate) require_specifying_escaper: bool,

    /// Whether to attempt to infer the escaper group
    /// from the template's file extension.
    #[cfg_attr(not(feature = "oxiplate"), allow(dead_code))]
    #[cfg_attr(feature = "config", serde(default))]
    pub(crate) infer_escaper_group_from_file_extension: bool,

    #[cfg_attr(not(feature = "oxiplate"), allow(dead_code))]
    #[cfg_attr(feature = "config", serde(default))]
    pub(crate) optimized_renderer: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fallback_escaper_group: None,
            escaper_groups: HashMap::new(),
            require_specifying_escaper: false,
            infer_escaper_group_from_file_extension: true,
            optimized_renderer: true,
        }
    }
}

/// Escaper group defined in the configuration.
#[cfg_attr(not(feature = "oxiplate"), allow(dead_code))]
#[cfg_attr(feature = "config", derive(Deserialize))]
#[cfg_attr(feature = "config", serde(deny_unknown_fields))]
pub(crate) struct EscaperGroup {
    pub(crate) escaper: String,
}

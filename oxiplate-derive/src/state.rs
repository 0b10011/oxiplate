use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::{env, fs};

#[cfg(feature = "config")]
use serde::Deserialize;
use syn::spanned::Spanned;
use syn::DeriveInput;

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
}

/// Macro configuration.
#[cfg_attr(not(feature = "built-in-escapers"), derive(Default))]
#[cfg_attr(feature = "config", derive(Deserialize))]
#[cfg_attr(feature = "config", serde(deny_unknown_fields))]
pub(crate) struct Config {
    #[cfg_attr(feature = "config", serde(default))]
    pub(crate) default_escaper_group: Option<String>,
    #[cfg_attr(feature = "config", serde(default))]
    pub(crate) escaper_groups: HashMap<String, EscaperGroup>,
}

#[cfg(feature = "built-in-escapers")]
impl Default for Config {
    fn default() -> Self {
        Self {
            default_escaper_group: Some("html".to_string()),
            escaper_groups: HashMap::new(),
        }
    }
}

/// Escaper group defined in the configuration.
#[cfg_attr(feature = "config", derive(Deserialize))]
#[cfg_attr(feature = "config", serde(deny_unknown_fields))]
pub(crate) struct EscaperGroup {
    pub(crate) escaper: String,
}

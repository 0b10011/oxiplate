use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::{env, fs};

#[cfg(feature = "config")]
use serde::Deserialize;
use syn::spanned::Spanned;
use syn::DeriveInput;

#[cfg(not(feature = "config"))]
pub(crate) fn build_config(input: &DeriveInput) -> Result<Config, syn::Error> {
    if fs::exists(config_path()).unwrap_or(false) {
        Err(syn::Error::new(
            input.span(),
            r#"`/oxiplate.toml` exists, but the "config" feature is turned off. Either delete/rename `/oxiplate.toml`, or turn the "config" feature on."#,
        ))
    } else {
        Ok(default_config())
    }
}

#[cfg(feature = "config")]
pub(crate) fn build_config(input: &DeriveInput) -> Result<Config, syn::Error> {
    if let Ok(toml) = fs::read_to_string(config_path().clone()) {
        toml::from_str(&toml).map_err(|error| {
            syn::Error::new(
                input.span(),
                format!("Failed to parse `/oxiplate.toml`: {}", error.message()),
            )
        })
    } else {
        Ok(default_config())
    }
}

fn config_path() -> PathBuf {
    let root = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR_OVERRIDE")
            .or(env::var("CARGO_MANIFEST_DIR"))
            .unwrap(),
    );
    root.join("oxiplate.toml")
}

fn default_config() -> Config {
    Config {
        default_escaper_group: None,
        escaper_groups: HashMap::new(),
    }
}

pub(crate) struct State<'a> {
    pub(crate) local_variables: &'a HashSet<&'a str>,
    pub(crate) config: &'a Config,
}

#[cfg_attr(feature = "config", derive(Deserialize, Default))]
#[cfg_attr(feature = "config", serde(deny_unknown_fields))]
pub(crate) struct Config {
    #[cfg_attr(feature = "config", serde(default))]
    pub(crate) default_escaper_group: Option<String>,
    #[cfg_attr(feature = "config", serde(default))]
    pub(crate) escaper_groups: HashMap<String, EscaperGroup>,
}

#[cfg_attr(feature = "config", derive(Deserialize))]
#[cfg_attr(feature = "config", serde(deny_unknown_fields))]
pub(crate) struct EscaperGroup {
    pub(crate) escaper: String,
}

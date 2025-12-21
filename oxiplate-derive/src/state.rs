use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Deref;
use std::path::PathBuf;
use std::{env, fs};

#[cfg(feature = "config")]
use serde::Deserialize;
use syn::DeriveInput;
use syn::spanned::Spanned;

use crate::syntax::Template;

#[cfg(all(feature = "built-in-escapers", not(feature = "oxiplate")))]
compile_error!(
    "The `built-in-escapers` feature only works when using the `oxiplate` library rather than \
     `oxiplate-derive` directly."
);

/// Build a `Config` from Oxiplate's defaults
/// and the user-defined `/oxiplate.toml`.
pub(crate) fn build_config(input: &DeriveInput) -> Result<Config, (syn::Error, OptimizedRenderer)> {
    #[cfg(not(feature = "config"))]
    if fs::exists(config_path()).unwrap_or(false) {
        return Err((
            syn::Error::new(
                input.span(),
                r#"`/oxiplate.toml` exists, but the "config" feature is turned off. Either delete/rename `/oxiplate.toml`, or turn the "config" feature on."#,
            ),
            OptimizedRenderer::unoptimized(),
        ));
    }

    #[cfg(not(feature = "built-in-escapers"))]
    let config = read_config(input);

    #[cfg(feature = "built-in-escapers")]
    let config = read_config_and_add_built_in_escapers(input);

    let config = config.map_err(|err| (err, OptimizedRenderer::unoptimized()))?;

    if let Some(ref fallback_escaper_group) = config.fallback_escaper_group
        && fallback_escaper_group != "raw"
        && !config
            .escaper_groups
            .contains_key(fallback_escaper_group.as_str())
    {
        return Err((
            syn::Error::new(
                input.span(),
                format!(
                    "The `fallback_escaper_group` that was provided (`{fallback_escaper_group}`) \
                     does not match any of the `escaper_groups` specified in `/oxiplate.toml`. \
                     Fix `fallback_escaper_group` or add the missing group to `escaper_groups`."
                ),
            ),
            config.optimized_renderer,
        ));
    }

    Ok(config)
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
#[derive(Clone)]
pub(crate) struct State<'a> {
    pub(crate) local_variables: &'a HashSet<&'a str>,
    pub(crate) config: &'a Config,
    pub(crate) inferred_escaper_group: Option<(&'a str, &'a EscaperGroup)>,

    /// Default escaper group for a template.
    /// Overrides any inferred escaping group that's already set.
    pub(crate) default_escaper_group: Option<(&'a str, &'a EscaperGroup)>,

    /// Flag to track when setting the default escaper group fails.
    /// Because this can change which escapers are available,
    /// it can result in a different error for every writ that escapes values.
    /// This allows for reducing it to a single error per template.
    pub(crate) failed_to_set_default_escaper_group: &'a bool,
    pub(crate) blocks:
        &'a VecDeque<&'a HashMap<&'a str, (&'a Template<'a>, Option<&'a Template<'a>>)>>,
    pub(crate) has_content: &'a bool,
}

#[cfg_attr(feature = "config", derive(Deserialize))]
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

#[cfg_attr(feature = "config", derive(Deserialize))]
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

/// Macro configuration.
#[cfg_attr(feature = "config", derive(Deserialize))]
#[cfg_attr(feature = "config", serde(deny_unknown_fields))]
#[cfg_attr(not(feature = "unreachable"), derive(Default))]
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
    pub(crate) infer_escaper_group_from_file_extension: InferEscaperGroupFromFileExtension,

    #[cfg_attr(not(feature = "oxiplate"), allow(dead_code))]
    #[cfg_attr(feature = "config", serde(default))]
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
#[cfg_attr(feature = "config", derive(Deserialize))]
#[cfg_attr(feature = "config", serde(deny_unknown_fields))]
pub(crate) struct EscaperGroup {
    pub(crate) escaper: String,
}

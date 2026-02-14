use std::collections::{HashMap, HashSet, VecDeque};
#[cfg(not(feature = "config"))]
use std::fs;

use syn::DeriveInput;
use syn::spanned::Spanned;

use crate::BuiltTokens;
#[cfg(not(feature = "config"))]
use crate::config::config_path;
use crate::config::{Config, EscaperGroup, OptimizedRenderer, read_config};

#[cfg(all(feature = "built-in-escapers", not(feature = "_oxiplate")))]
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
    let config = read_config();

    #[cfg(feature = "built-in-escapers")]
    let config = read_config_and_add_built_in_escapers();

    let config = config.map_err(|err| (err, OptimizedRenderer::unoptimized()))?;

    if let Some(ref fallback_escaper_group) = config.fallback_escaper_group {
        if fallback_escaper_group != "raw"
            && !config
                .escaper_groups
                .contains_key(fallback_escaper_group.as_str())
        {
            return Err((
                syn::Error::new(
                    input.span(),
                    format!(
                        "The `fallback_escaper_group` that was provided \
                         (`{fallback_escaper_group}`) does not match any of the `escaper_groups` \
                         specified in `/oxiplate.toml`. Fix `fallback_escaper_group` or add the \
                         missing group to `escaper_groups`."
                    ),
                ),
                config.optimized_renderer,
            ));
        }
    }

    Ok(config)
}

/// Build a `Config` from the user-defined `/oxiplate.toml`
/// and add all built-in escapers that have not been overridden.
#[cfg(feature = "built-in-escapers")]
fn read_config_and_add_built_in_escapers() -> Result<Config, syn::Error> {
    let mut config = read_config()?;

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

/// Local variables available for usage within templates.
pub(crate) struct LocalVariables {
    /// Currently active variables.
    active: HashSet<String>,

    /// Groups of variables added in each statement/branch.
    stack: Vec<Vec<String>>,
}

impl LocalVariables {
    /// Create a new instance of local variables.
    pub fn new() -> Self {
        Self {
            active: HashSet::new(),
            stack: vec![vec![]],
        }
    }

    /// Whether the provided variable name exists as a local variable.
    #[must_use]
    pub fn contains(&self, var: &str) -> bool {
        self.active.contains(var)
    }

    /// Add one or more variables to the current stack.
    pub fn add(&mut self, vars: HashSet<String>) {
        let existing: Vec<String> = vars
            .difference(&self.active)
            .collect::<HashSet<&String>>()
            .into_iter()
            .map(ToOwned::to_owned)
            .collect();
        let Some(stack) = self.stack.last_mut() else {
            unreachable!("Attempted to add variable to stack, but no stack present");
        };
        for var in existing {
            stack.push(var);
        }
        self.active.extend(vars);
    }

    /// Push a stack for a new statement or branch.
    pub fn push_stack(&mut self) {
        self.stack.push(vec![]);
    }

    /// Pop the latest stack of variables
    /// and remove all variables added in it.
    pub fn pop_stack(&mut self) {
        let Some(vars) = self.stack.pop() else {
            unreachable!("Attempted to pop stack, but no stack remained");
        };

        for var in vars {
            self.active.remove(&var);
        }
    }
}

/// Macro state containing the configuration and any local variables.
pub(crate) struct State<'a> {
    /// Storage for local variable names when building tokens.
    pub(crate) local_variables: LocalVariables,
    pub(crate) config: Config,
    pub(crate) inferred_escaper_group: Option<(String, EscaperGroup)>,

    /// Default escaper group for a template.
    /// Overrides any inferred escaping group that's already set.
    pub(crate) default_escaper_group: Option<(String, EscaperGroup)>,

    /// Flag to track when setting the default escaper group fails.
    /// Because this can change which escapers are available,
    /// it can result in a different error for every writ that escapes values.
    /// This allows for reducing it to a single error per template.
    pub(crate) failed_to_set_default_escaper_group: bool,
    pub(crate) blocks: &'a VecDeque<&'a HashMap<&'a str, (BuiltTokens, Option<BuiltTokens>)>>,
    pub(crate) has_content: bool,
}

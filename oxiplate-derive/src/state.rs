use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::PathBuf;

pub(crate) fn build_config() -> Config {
    let root = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR_OVERRIDE")
            .or(env::var("CARGO_MANIFEST_DIR"))
            .unwrap(),
    );
    let config_path = root.join("oxiplate.toml");
    if let Ok(toml) = fs::read_to_string(config_path.clone()) {
        toml::from_str(&toml).expect("Failed to parse oxiplate.toml")
    } else {
        Config {
            default_escaper_group: None,
            escaper_groups: HashMap::new(),
        }
    }
}

pub(crate) struct State<'a> {
    pub(crate) local_variables: &'a HashSet<&'a str>,
    pub(crate) config: &'a Config,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    #[serde(default)]
    pub(crate) default_escaper_group: Option<String>,
    #[serde(default)]
    pub(crate) escaper_groups: HashMap<String, EscaperGroup>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct EscaperGroup {
    pub(crate) escaper: String,
}

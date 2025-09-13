use anyhow::{Context, Result};
use jsonc_parser::parse_to_serde_value;
use serde::Deserialize;

use std::{
    env, fs,
    path::{Path, PathBuf},
};

#[derive(Deserialize, Clone, Debug)]
pub struct Configuration {
    pub width: i32,
    
    pub query_height: i32,
}

pub fn config_home_path() -> PathBuf {
    if let Ok(path) = env::var("XDG_CONFIG_HOME") {
        if !path.is_empty() {
            return path.into();
        }
    }

    let home = env::var("HOME").expect("HOME environment variable not set");
    PathBuf::from(home).join(".config")
}

pub fn from_env_or_home() -> Result<Configuration> {
    let path = config_home_path().join("runex").join("config.jsonc");

    return from_file(&path);
}

pub fn from_file(path: &Path) -> Result<Configuration> {
    let contents = fs::read_to_string(path)
        .with_context(|| format!("Failed to read configuration file at {}", path.display()))?;

    let raw = parse_to_serde_value(&contents, &Default::default())?
        .ok_or_else(|| anyhow::anyhow!("Invalid configuration file"))?;

    let config: Configuration =
        serde_json::from_value(raw).context("Failed to deserialize configuration file")?;

    Ok(config)
}

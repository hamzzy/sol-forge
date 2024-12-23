use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};
use std::{path::PathBuf, fs};
use toml;

 #[derive(Serialize, Deserialize, Debug)]
pub struct AnchorforgeConfig {
    pub project_name: String,
    pub program_keypair_path: Option<String>,
    pub cluster_url: Option<String>,
}

pub fn get_config() -> Result<AnchorforgeConfig> {
   let config_path = PathBuf::from("anchorforge.toml");
    if !config_path.exists() {
        anyhow::bail!("Can't find config file {}", config_path.display());
    }
    let config_str = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config file at {}", config_path.display()))?;

   let config: AnchorforgeConfig = toml::from_str(&config_str)
        .with_context(|| format!("Failed to parse config file at {}", config_path.display()))?;
    Ok(config)
}
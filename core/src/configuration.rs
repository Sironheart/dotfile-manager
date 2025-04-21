use anyhow::{Result, anyhow};
use serde::de::DeserializeOwned;

pub fn generate_config<T>(content: &str, extension: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    match extension {
        "yml" | "yaml" => Ok(serde_yml::from_str(content)?),
        "json" | "json5" => Ok(serde_json::from_str(content)?),
        "toml" => Ok(toml::from_str(content)?),
        _ => Err(anyhow!("Cannot parse this file format!")),
    }
}

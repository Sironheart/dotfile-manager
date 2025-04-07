use anyhow::{Context, Result, anyhow};
use serde::de::DeserializeOwned;
use std::{fs, path::Path};

pub fn generate_config<T>(path: &Path) -> Result<T>
where
    T: DeserializeOwned,
{
    if path.is_dir() {
        return Err(anyhow!(
            "This command needs a single file, that is not a directory!"
        ));
    }

    let config_content: String = fs::read_to_string(path)
        .with_context(|| format!("Cannot read file at path {}", path.display()))?;

    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .with_context(|| "Could not read extension from file or path");

    return match extension? {
        "yml" | "yaml" => Ok(serde_yml::from_str(&config_content)?),
        "json" | "json5" => Ok(serde_json::from_str(&config_content)?),
        "toml" => Ok(toml::from_str(&config_content)?),
        _ => Err(anyhow!("Cannot parse this file format!")),
    };
}

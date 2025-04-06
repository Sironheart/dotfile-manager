use super::model::ParsedConfiguration;
use super::projects::configure_project_base_path;
use anyhow::{Context, Result, anyhow};
use std::fs;
use std::path::PathBuf;

pub fn setup(path: &PathBuf) -> Result<()> {
    let config = generate_config(path)?;

    configure_project_base_path(&config.project_config.clone())?;

    println!("{config:?}");

    Ok(())
}

fn generate_config(path: &PathBuf) -> Result<ParsedConfiguration> {
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

    match extension? {
        "yml" | "yaml" => Ok(serde_yml::from_str(&config_content)?),
        "json" | "json5" => Ok(serde_json::from_str(&config_content)?),
        "toml" => Ok(toml::from_str(&config_content)?),
        _ => Err(anyhow!("Cannot parse this file format!")),
    }
}

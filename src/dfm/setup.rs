use super::model::Config;
use anyhow::{Result, anyhow};
use std::fs;
use std::path::PathBuf;

pub fn setup(path: PathBuf) -> Result<()> {
    let config = generate_config(path)

    println!("{:?}", config?);

    Ok(())
}

fn generate_config(path: PathBuf) -> Result<Config> {
    let config_content: String = fs::read_to_string(&path).expect("");

    let extension = path.extension().expect("").to_str().unwrap();

    match extension {
        "yml" | "yaml" => Ok(serde_yaml::from_str(&config_content).unwrap()),
        "json" | "json5" => Ok(serde_json::from_str(&config_content).unwrap()),
        "toml" => Ok(toml::from_str(&config_content).unwrap()),
        _ => Err(anyhow!("Cannot parse this file format!")),
    }
}

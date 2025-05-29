pub mod basic_config;
pub mod configuration;
pub mod git;
pub mod projects;

extern crate anyhow;
extern crate serde;
extern crate shellexpand;

use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;
use basic_config::BasicConfigContent;
use std::fs;
use std::path::Path;
use std::vec::Vec;

pub fn setup(
    path: &Path,
    force: bool,
    additional_modules: Vec<Box<dyn SetupAdapter>>,
) -> Result<()> {
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
        .ok_or(anyhow!("Could not read extension"))?;

    let config = configuration::generate_config::<BasicConfigContent>(
        &add_force_field_to_config(force, extension, &config_content),
        extension,
    )
    .map_err(|_| {
        println!("Could not serialize");
        anyhow!("Well, shit!")
    })?;

    projects::configure_project_base_path(&config.base.clone())?;

    for module in additional_modules {
        module
            .exec(&config_content, extension, &config)
            .with_context(|| "Something went terribly wrong")?;
    }

    Ok(())
}

fn add_force_field_to_config(force: bool, extension: &str, original_content: &str) -> String {
    match extension {
        "yaml" => format!("force: {force}\n{original_content}"),
        "toml" => format!("force = {force}\n{original_content}"),
        "json" => format!(
            r#"{{ "force": {}, {} }}"#,
            force,
            original_content.trim().trim_matches('{').trim_matches('}'),
        ),
        _ => todo!("Not a supported extension."),
    }
}

pub trait SetupAdapter {
    fn exec(
        &self,
        config_string: &str,
        config_extension: &str,
        base_config: &BasicConfigContent,
    ) -> Result<()>;
}

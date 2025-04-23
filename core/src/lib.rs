pub mod basic_config;
pub mod configuration;
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

pub fn setup(path: &Path, additional_modules: Vec<Box<dyn SetupAdapter>>) -> Result<()> {
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
        .ok_or(anyhow!("Could not read extension "))?;

    let config = configuration::generate_config::<BasicConfigContent>(&config_content, extension)?;

    projects::configure_project_base_path(&config.base.clone())?;

    for module in additional_modules {
        module
            .exec(&config_content, extension, &config)
            .with_context(|| "Something went terribly wrong")?;
    }
    Ok(())
}

pub trait SetupAdapter {
    fn exec(
        &self,
        config_string: &str,
        config_extension: &str,
        base_config: &BasicConfigContent,
    ) -> Result<()>;
}

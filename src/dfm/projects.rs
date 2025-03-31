use super::model::ProjectConfig;
use anyhow::{Context, Ok, Result};
use std::{fs, path::Path};

pub fn configure_project_base_path(config: ProjectConfig) -> Result<()> {
    if !Path::exists(&config.base_path) {
        fs::create_dir_all::<_>(&config.base_path.canonicalize()?)
            .with_context(|| format!("Could not create path"))?
    }

    return Ok(());
}

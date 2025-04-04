use super::model::ProjectConfig;
use anyhow::{Context, Error};
use std::{fs, path::Path};

pub fn configure_project_base_path(config: &ProjectConfig) -> Result<(), Error> {
    let base_path = match config.base_path.is_absolute() {
        true => &config.base_path.canonicalize()?,
        false => &config.base_path,
    };
    if !Path::try_exists(&base_path)? {
        fs::create_dir_all(base_path).with_context(|| format!("Could not create project paths"))?;
    }

    return Ok(());
}

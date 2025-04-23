use super::basic_config::BaseConfig;
use anyhow::{Context, Error};
use std::{fs, path::Path};

pub fn configure_project_base_path(config: &BaseConfig) -> Result<(), Error> {
    let base_path = if config.base_path.is_absolute() {
        &config.base_path.canonicalize()?
    } else {
        &config.base_path
    };
    if !Path::try_exists(base_path)? {
        fs::create_dir_all(base_path).with_context(|| "Could not create project paths")?;
    }

    Ok(())
}

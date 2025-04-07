pub mod basic_config;
pub mod configuration;
pub mod projects;

extern crate anyhow;
extern crate serde;
extern crate shellexpand;

use anyhow::Result;
use basic_config::BasicConfigContent;
use serde::de::DeserializeOwned;
use std::path::Path;

pub fn setup(path: &Path) -> Result<()> {
    let config = configuration::generate_config::<BasicConfigContent>(path)?;

    projects::configure_project_base_path(&config.base.clone())?;

    println!("{config:?}");

    Ok(())
}

pub trait ConfigurationAdapter<T>
where
    T: DeserializeOwned,
{
    fn new(path: &Path) -> Self;
    fn is_responsible(&self) -> bool;
    fn execute(&self);
}

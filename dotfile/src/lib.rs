extern crate serde;

use anyhow::Result;
use core::{
    SetupAdapter,
    basic_config::{BasicConfigContent, deserialize_and_resolve_path},
};
use serde::Deserialize;
use std::{
    fs::{self, File},
    io::{ErrorKind, Write},
    path::PathBuf,
};

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DotfileConfiguration {
    pub(crate) files: Option<Vec<DotfileDefinition>>,
    pub(crate) _nvim: String,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DotfileDefinition {
    #[serde(deserialize_with = "deserialize_and_resolve_path")]
    path: PathBuf,
    content: String,
}

pub struct DotfileSetup {}

impl SetupAdapter for DotfileSetup {
    fn exec(
        &self,
        config_string: &str,
        config_extension: &str,
        _base_config: &BasicConfigContent,
    ) -> Result<()> {
        let config: DotfileConfiguration =
            core::configuration::generate_config(config_string, config_extension)?;

        config.create_files();
        println!("{:?}", config._nvim);

        Ok(())
    }
}

impl DotfileConfiguration {
    fn create_files(&self) {
        self.files
            .iter()
            .flatten()
            .for_each(|f| f.create_system_file());
    }
}

impl DotfileDefinition {
    fn create_system_file(&self) {
        if let Some(parent) = self.path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let mut f = match File::create_new(&self.path) {
            Ok(f) => f,
            Err(err) => {
                match err.kind() {
                    ErrorKind::AlreadyExists => println!(
                        "{:?} already exists. Delete or backup the current version and try again.",
                        self.path
                    ),
                    _ => println!("{err:?}"),
                }

                return;
            }
        };

        let _ = f.write_all(self.content.as_bytes());

        println!("created {:?}", self.path)
    }
}

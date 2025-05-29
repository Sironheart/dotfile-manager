extern crate serde;
extern crate shellexpand;

use anyhow::Result;
use core::{
    SetupAdapter,
    basic_config::{BasicConfigContent, deserialize_and_resolve_path},
    git::GitModule,
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
    pub(crate) nvim: Option<String>,
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
        base_config: &BasicConfigContent,
    ) -> Result<()> {
        let config: DotfileConfiguration =
            core::configuration::generate_config(config_string, config_extension)?;

        config.create_files(base_config);
        config.create_program_files(base_config)?;

        Ok(())
    }
}

impl DotfileConfiguration {
    fn create_files(&self, base_config: &BasicConfigContent) {
        self.files
            .iter()
            .flatten()
            .for_each(|f| f.create_system_file(base_config));
    }

    fn create_program_files(&self, base_config: &BasicConfigContent) -> Result<()> {
        let nvim = match &self.nvim {
            Some(nvim) => nvim,
            None => return Ok(()), // probably not a nvim user... which is wrong in other than
                                   // we're able to handle right here.
        };

        let target_path = format!(
            "{}/{}",
            &base_config
                .base
                .base_path
                .to_str()
                .expect("this should have already been validated to exist."),
            GitModule::get_project_path(nvim, base_config)
                .expect("Malformed git resource mentioned!"),
        );

        GitModule::git_clone(nvim, &target_path)?;

        Ok(())
    }
}

impl DotfileDefinition {
    fn create_system_file(&self, base_config: &BasicConfigContent) {
        self.path.parent().map(fs::create_dir_all);

        let _ = if base_config.force {
            File::create(&self.path)
        } else {
            File::create_new(&self.path).map_err(|err| {
                match err.kind() {
                    ErrorKind::AlreadyExists => println!(
                        "{:?} already exists. Delete or backup the current version and try again.",
                        self.path
                    ),
                    _ => eprintln!("{err:?}"),
                }
                err
            })
        }
        .map(|mut f| {
            f.write_all(self.content.as_bytes()).ok();

            println!("created {:?}", self.path);
        });
    }
}

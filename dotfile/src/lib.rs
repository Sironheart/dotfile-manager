extern crate serde;
extern crate shellexpand;

use anyhow::{Context, Result};
use core::{
    SetupAdapter,
    basic_config::{BasicConfigContent, deserialize_and_resolve_path},
    git::GitModule,
};
use serde::Deserialize;
use std::{
    fs::{self, File},
    io::{ErrorKind, Write},
    os::unix::fs as unix_fs,
    path::{Path, PathBuf},
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
        self.handle_nvim_files(base_config).with_context(|| {
            tracing::info!(
                "There was an error during nvim configuration. Please check whether this is acceptable on your system"
            );

            "Could not create the nvim settings"
        })?;

        Ok(())
    }

    fn handle_nvim_files(&self, base_config: &BasicConfigContent) -> Result<()> {
        let nvim = match &self.nvim {
            Some(nvim) => nvim,
            None => return Ok(()), // probably not a nvim user... which is wrong in other ways
                                   // than we're able to handle right here.
        };

        let target_path = format!(
            "{}/{}",
            &base_config
                .base
                .base_path
                .to_str()
                .with_context(|| "this should have already been validated to exist.")?,
            GitModule::get_project_path(nvim, base_config)
                .with_context(|| "Malformed git resource mentioned!")?,
        );

        let path = Path::new(&target_path);

        if base_config.force && Path::try_exists(path)? {
            fs::remove_dir_all(path).with_context(|| "couldn't delete {path:}")?
        }

        GitModule::git_clone(nvim, &target_path)?;

        let target_nvim_dir = shellexpand::full("~/.config/nvim")
            .with_context(|| "Wasn't able to resolve to the home `.config` dir.")?
            .into_owned();

        if Path::try_exists(Path::new(&target_nvim_dir))? {
            fs::remove_file(&target_nvim_dir)?;
        }

        unix_fs::symlink(&target_path, target_nvim_dir)?;

        tracing::info!("Nvim is now setup");

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
                    ErrorKind::AlreadyExists => tracing::info!(
                        "{:?} already exists. Delete or backup the current version and try again.",
                        self.path
                    ),
                    _ => tracing::error!("{err:?}"),
                }
                err
            })
        }
        .map(|mut f| {
            f.write_all(self.content.as_bytes()).ok();

            tracing::debug!("created {:?}", self.path);
        });
    }
}

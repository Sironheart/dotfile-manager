extern crate serde;

use core::{
    ConfigurationAdapter,
    basic_config::{BasicConfigContent, deserialize_and_resolve_path},
    configuration::generate_config,
};
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DotfileConfiguration {
    pub base_configuration: BasicConfigContent,
    pub(crate) files: Option<Vec<DotfileDefinition>>,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DotfileDefinition {
    #[serde(deserialize_with = "deserialize_and_resolve_path")]
    _path: PathBuf,
    _content: String,
}

impl std::ops::Deref for DotfileConfiguration {
    type Target = BasicConfigContent;

    fn deref(&self) -> &Self::Target {
        &self.base_configuration
    }
}

pub struct DotfileAdapter {
    pub dotfile_configuration: Option<DotfileConfiguration>,
}

impl ConfigurationAdapter<DotfileConfiguration> for DotfileAdapter {
    fn new(path: &Path) -> Self {
        let Ok(config) = generate_config::<DotfileConfiguration>(path) else {
            return Self {
                dotfile_configuration: None,
            };
        };

        Self {
            dotfile_configuration: Some(config),
        }
    }

    fn is_responsible(&self) -> bool {
        match &self.dotfile_configuration {
            Some(val) => val.files.iter().count() != 0,
            _ => false,
        }
    }

    fn execute(&self) {
        todo!()
    }
}

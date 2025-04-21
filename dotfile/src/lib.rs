extern crate serde;

use anyhow::Result;
use core::{
    SetupAdapter,
    basic_config::{BasicConfigContent, deserialize_and_resolve_path},
};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DotfileConfiguration {
    pub(crate) _files: Option<Vec<DotfileDefinition>>,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DotfileDefinition {
    #[serde(deserialize_with = "deserialize_and_resolve_path")]
    _path: PathBuf,
    _content: String,
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

        println!("{config:?}");

        Ok(())
    }
}

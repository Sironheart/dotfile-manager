use serde::{Deserialize, Deserializer};
use shellexpand::full;
use std::default::Default;
use std::path::PathBuf;

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParsedConfiguration {
    pub project_config: ProjectConfig,
    pub _nvim_config: Option<String>,
    pub _files: Option<Vec<DotfileDefinition>>,
    pub _macos: Option<MacosConfig>,
    pub _linux: Option<LinuxConfig>,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectConfig {
    #[serde(deserialize_with = "deserialize_and_resolve_path")]
    pub base_path: PathBuf,
    pub _use_git_source_path: bool,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DotfileDefinition {
    #[serde(deserialize_with = "deserialize_and_resolve_path")]
    _path: PathBuf,
    _content: String,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MacosConfig {
    _brew: HomebrewDefinition,
    _config_opts: MacosOpts,
}
#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomebrewDefinition {
    _package: Option<Vec<String>>,
    _casks: Option<Vec<String>>,
    _tap: Option<Vec<String>>,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MacosOpts {}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinuxConfig {}

fn deserialize_and_resolve_path<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let Ok(expanded_path) = full(&s) else { todo!() };
    Ok(PathBuf::from(expanded_path.into_owned()))
}

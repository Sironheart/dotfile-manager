use serde::Deserialize;
use std::default::Default;
use std::path::PathBuf;

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub project_config: ProjectConfig,
    pub nvim_config: Option<String>,
    pub files: Option<Vec<DotfileDefinition>>,
    pub macos: Option<MacosConfig>,
    pub linux: Option<LinuxConfig>,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectConfig {
    pub base_path: PathBuf,
    pub use_git_source_path: bool,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DotfileDefinition {
    path: PathBuf,
    content: String,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MacosConfig {
    brew: HomebrewDefinition,
    config_opts: MacosOpts,
}
#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomebrewDefinition {
    package: Option<Vec<String>>,
    casks: Option<Vec<String>>,
    tap: Option<Vec<String>>,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MacosOpts {}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinuxConfig {}

use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    project_config: ProjectConfig,
    nvim_config: Option<String>,
    files: Option<Vec<DotfileDefinition>>,
    macos: Option<MacosConfig>,
    linux: Option<LinuxConfig>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ProjectConfig {
    base_path: PathBuf,
    use_git_source_path: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DotfileDefinition {
    path: PathBuf,
    content: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MacosConfig {
    brew: HomebrewDefinition,
    config_opts: MacosOpts,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct HomebrewDefinition {
    package: Option<Vec<String>>,
    casks: Option<Vec<String>>,
    tap: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MacosOpts {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LinuxConfig {}

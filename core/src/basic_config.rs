use serde::{Deserialize, Deserializer};
use shellexpand::full;
use std::default::Default;
use std::path::PathBuf;

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BasicConfigContent {
    pub base: BaseConfig,
    pub force: bool,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BaseConfig {
    #[serde(deserialize_with = "deserialize_and_resolve_path")]
    pub base_path: PathBuf,
    pub use_git_source_path: bool,
}

pub fn deserialize_and_resolve_path<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let Ok(expanded_path) = full(&s) else { todo!() };
    Ok(PathBuf::from(expanded_path.into_owned()))
}

use std::fs;

use git2::Repository;

use crate::basic_config::BasicConfigContent;

pub struct GitModule {}

impl GitModule {
    pub fn git_clone(source_path: &str, target_folder: &str) -> anyhow::Result<()> {
        if let Some((parent, _)) = target_folder.rsplit_once("/") {
            fs::create_dir_all(parent)?;
        };

        Repository::clone(source_path, target_folder)
            .map(|repo| {
                tracing::info!("cloned repository: {:?}", repo.workdir());
            })
            .unwrap_or_else(|err| {
                tracing::warn!("Git clone failed with following error: {}", err.message());
            });

        Ok(())
    }

    pub fn get_project_path(source_path: &str, base_config: &BasicConfigContent) -> Option<String> {
        let source_path = source_path.trim_end_matches(".git");

        if base_config.base.use_git_source_path {
            Some(
                source_path
                    .rsplit("@")
                    .next()?
                    .rsplit("//")
                    .next()?
                    .replace(":", "/")
                    .to_lowercase()
                    .to_string(),
            )
        } else {
            Some(
                source_path
                    .rsplit("/")
                    .next()
                    .map(|source| source.to_string())?,
            )
        }
    }
}

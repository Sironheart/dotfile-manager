use std::{fs, path::Path};

use git2::Repository;

use crate::basic_config::BasicConfigContent;

pub struct GitModule {}

impl GitModule {
    pub fn git_clone(source_path: &str, target_folder: &str) -> anyhow::Result<()> {
        if let Some((parent, _)) = target_folder.rsplit_once("/") {
            fs::create_dir_all(parent)?;
        };

        if fs::exists(target_folder)? {
            tracing::info!(
                "the path \"{}\" already exists. If you'd like to overwrite this path use the `--force` flag.",
                target_folder
            );
            return Ok(());
        }

        Repository::clone(source_path, target_folder)
            .map(|repo| {
                tracing::debug!(
                    "cloned repository: {:?}",
                    repo.workdir().unwrap_or_else(|| Path::new(source_path))
                );
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
                    .rsplit("@") // remove everything before an `@`, like in most ssh clone URLs
                    .next()?
                    .rsplit("//") // do remove the stuff in front of `//`, like in most https URLs
                    .next()?
                    .replace(":", "/") // in git urls somehow hostnames and folders are split using `:`
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

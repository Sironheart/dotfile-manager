use std::fs;

use git2::Repository;

use crate::basic_config::BasicConfigContent;

pub struct GitModule {}

impl GitModule {
    pub fn git_clone(source_path: &str, target_folder: &str) -> anyhow::Result<()> {
        if let Some((parent, _)) = target_folder.rsplit_once("/") {
            fs::create_dir_all(parent)?;
        };

        let repo = match Repository::clone(source_path, target_folder) {
            Ok(repository) => repository,
            Err(err) => {
                println!("Git clone failed with following error: {}", err.message());
                return Ok(());
            }
        };

        println!("cloned repository: {:?}", repo.workdir());

        Ok(())
    }

    pub fn get_project_path(source_path: &str, base_config: &BasicConfigContent) -> Option<String> {
        let source_path = source_path.trim_end_matches(".git");
        let source = if base_config.base._use_git_source_path {
            Some(
                source_path
                    .rsplit("@")
                    .next()?
                    .rsplit("//")
                    .next()?
                    .replace(":", "/")
                    .to_lowercase(),
            )
        } else {
            return match source_path.rsplit("/").next() {
                Some(str) => Some(str.to_string()),
                None => return None,
            };
        }?;

        Some(source.to_string())
    }
}

use anyhow::{Result, anyhow};
use cmd_lib::{run_cmd, run_fun};
use core::SetupAdapter;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct MacosConfiguration {
    macos: Option<MacosDefinition>,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct MacosDefinition {
    brew: Option<HomebrewDefinition>,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct HomebrewDefinition {
    packages: Option<HashSet<String>>,
}

pub struct MacSetup {}

impl SetupAdapter for MacSetup {
    fn exec(
        &self,
        config_string: &str,
        config_extension: &str,
        base_config: &core::basic_config::BasicConfigContent,
    ) -> Result<()> {
        let config: MacosConfiguration =
            core::configuration::generate_config(config_string, config_extension)?;

        config.setup_macos(base_config).ok();

        tracing::debug!("{:?}", config);

        Ok(())
    }
}

impl MacosConfiguration {
    fn setup_macos(&self, _base_config: &core::basic_config::BasicConfigContent) -> Result<()> {
        let macos = match &self.macos {
            Some(macos) => macos,
            None => return Ok(()),
        };

        macos.setup_homebrew()?;

        Ok(())
    }
}

impl MacosDefinition {
    fn setup_homebrew(&self) -> Result<()> {
        if !self.is_homebrew_installed() {
            tracing::error!(
                "You will need to install homebrew manually first. Please take a look at https://brew.sh/ for that"
            );

            return Err(anyhow!("Homebrew not installed!"));
        }

        tracing::debug!("homebrew is already installed!");

        if let Some(brew) = &self.brew {
            if let Some(packages) = &brew.packages {
                // std::env::set_var("HOMEBREW_NO_INSTALLED_DEPENDENTS_CHECK", "1");

                let installed: HashSet<String> = run_fun!(brew leaves)?
                    .split("\n")
                    .map(|s| s.to_string())
                    .collect();

                let to_remove: Vec<String> = installed
                    .difference(packages)
                    .map(|s| s.to_owned())
                    .collect();

                if !to_remove.is_empty() {
                    run_cmd!(
                        HOMEBREW_NO_INSTALLED_DEPENDENTS_CHECK=1 brew remove -f -q --formula $[to_remove]
                    )?;
                }

                let packages: Vec<String> = packages.iter().map(|s| s.to_string()).collect();

                if !packages.is_empty() {
                    run_cmd!(
                            HOMEBREW_NO_INSTALLED_DEPENDENTS_CHECK=1 brew install -q --formula $[packages]
                    )?;
                }
            }
        }

        tracing::info!("successfully installed all packages");

        Ok(())
    }

    fn is_homebrew_installed(&self) -> bool {
        run_cmd!(brew help >> /dev/null).is_ok()
    }
}

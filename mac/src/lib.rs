extern crate anyhow;
extern crate core;
extern crate serde;
extern crate tracing;

use anyhow::{Result, anyhow};
use cmd_lib::run_fun;
use core::SetupAdapter;
use serde::Deserialize;
use std::{process::Command, time::Duration};

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
    packages: Option<Vec<String>>,
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
        if !self.is_xcode_installed() {
            self.install_xcode()?;

            let mut wait_duration: Duration = Duration::from_secs(0);

            while !self.is_xcode_installed() {
                wait_duration += Duration::from_secs(5);
                std::thread::sleep(wait_duration);

                if wait_duration > Duration::from_secs(60) {
                    return Err(anyhow!(
                        "Homebrew wasn't installed successfully. Consider installing xcode-cli by yourself"
                    ));
                }
            }
        }

        if !self.is_homebrew_installed() {
            self.install_homebrew()?;
        }

        tracing::debug!("homebrew is already installed!");

        if let Some(brew) = &self.brew {
            if let Some(packages) = &brew.packages {
                let packages = packages.join(" ");
                run_fun!(brew install -q ${packages})?;
            }
        }

        tracing::info!("successfully installed all packages");

        Ok(())
    }

    fn is_xcode_installed(&self) -> bool {
        Command::new("xcode-select")
            .arg("-p")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn install_xcode(&self) -> Result<()> {
        let status = Command::new("xcode-select").arg("--install").status()?;

        tracing::info!("{:?}", status);
        todo!()
    }

    fn is_homebrew_installed(&self) -> bool {
        Command::new("brew")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn install_homebrew(&self) -> Result<()> {
        Command::new("/bin/bash")
            .args([
                "-c",
                "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)",
            ])
            .output()
            .map(|o| o.status.success())?;

        Ok(())
    }
}

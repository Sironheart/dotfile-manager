use anyhow::Result;
use core::SetupAdapter;
use std::env::consts;

pub struct MacSetup {}

impl SetupAdapter for MacSetup {
    fn exec(
        &self,
        _config_string: &str,
        _config_extension: &str,
        _base_config: &core::basic_config::BasicConfigContent,
    ) -> Result<()> {
        match consts::OS {
            "ios" | "macos" | "apple" => {}
            _ => return Ok(()),
        }

        Ok(())
    }
}

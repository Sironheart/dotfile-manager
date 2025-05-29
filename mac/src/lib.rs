use anyhow::Result;
use core::SetupAdapter;

pub struct MacSetup {}

impl SetupAdapter for MacSetup {
    fn exec(
        &self,
        _config_string: &str,
        _config_extension: &str,
        _base_config: &core::basic_config::BasicConfigContent,
    ) -> Result<()> {
        println!("This is a macos setup!");

        Ok(())
    }
}

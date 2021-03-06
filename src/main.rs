#[cfg(not(debug_assertions))]
#[macro_use]
extern crate human_panic;

#[cfg(debug_assertions)]
extern crate better_panic;

use utils::app_config::AppConfig;
use utils::error::Result;
use utils::log;

fn main() -> Result<()> {
    // Human Panic. Only enabled when *not* debugging.
    #[cfg(not(debug_assertions))]
    {
        setup_panic!();
    }

    // Better Panic. Only enabled *when* debugging.
    #[cfg(debug_assertions)]
    {
        better_panic::install();
    }

    // Setup Logging
    log::setup_logging()?;

    // Initialize Configuration
    let config_contents = include_str!("resources/default_config.toml");
    AppConfig::init(Some(config_contents))?;

    // Match Commands
    cli::cli_match()
}

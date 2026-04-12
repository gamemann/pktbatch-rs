mod batch;
mod cli;
mod config;
mod logger;
mod tech;
mod util;

mod context;

use anyhow::{Result, anyhow};

use crate::{cli::base::Cli, config::base::Config};

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments.
    let cli = Cli::parse();

    // Load configuration from file.
    let cfg = Config::load_from_file(&cli.args.config)
        .map_err(|e| anyhow!("Failed to load configuration: {}", e))?;

    if cli.args.list_cfg {
        cfg.list();

        return Ok(());
    }

    Ok(())
}

mod batch;
mod cli;
mod config;
mod logger;
mod tech;
mod util;

mod context;

use anyhow::{Result, anyhow};

use crate::{
    batch::base::Batch,
    cli::base::Cli,
    config::base::Config,
    context::ContextData,
    logger::{base::Logger, level::LogLevel},
    tech::base::TechBase,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments.
    let cli = Cli::parse();

    // Load configuration from file.
    let cfg = Config::load_from_file(&cli.args.config)
        .map_err(|e| anyhow!("Failed to load configuration: {}", e))?;

    // Check if we should list and exit.
    if cli.args.list_cfg {
        cfg.list();

        return Ok(());
    }

    // Initialize the logger.
    let logger_cfg = cfg.logger.clone();

    let logger = Logger::new(
        logger_cfg.level.unwrap_or_default(),
        logger_cfg.path,
        logger_cfg.path_is_file,
        logger_cfg.date_format_file,
        logger_cfg.date_format_line,
    );

    logger
        .log_msg(LogLevel::Trace, "Logger initialized...")
        .ok();

    // Create the batch.
    logger
        .log_msg(LogLevel::Trace, "Initializing batch...")
        .ok();

    let batch = Batch::new(
        cfg.batch.batches.iter().map(|b| b.clone().into()).collect(),
        cfg.batch
            .ovr_opts
            .clone()
            .map(|o| o.try_into())
            .transpose()
            .map_err(|e| {
                logger
                    .log_msg(
                        LogLevel::Fatal,
                        &format!("Failed to convert batch override options: {}", e),
                    )
                    .ok();

                anyhow!("Failed to convert batch override options: {}", e)
            })?,
    );

    // Create the tech.
    logger.log_msg(LogLevel::Trace, "Initializing tech...").ok();

    let tech: TechBase = cfg.tech.try_into().map_err(|e| {
        logger
            .log_msg(
                LogLevel::Fatal,
                &format!("Failed to initialize tech (conversion with config): {}", e),
            )
            .ok();

        anyhow!("Failed to initialize tech (conversion with config): {}", e)
    })?;

    // Now we need to initialize the global context.
    logger
        .log_msg(LogLevel::Trace, "Initializing context...")
        .ok();

    let ctx = ContextData::new(cfg, logger, cli, tech, batch);

    // Shadow vars.
    let cfg = &ctx.cfg;
    let logger = &ctx.logger;
    let tech = &ctx.tech;
    let batch = &ctx.batch;

    batch
        .read()
        .await
        .start_batches(ctx.clone())
        .await
        .map_err(|e| {
            logger
                .blocking_read()
                .log_msg(LogLevel::Fatal, &format!("Failed to start batches: {}", e))
                .ok();

            anyhow!("Failed to start batches: {}", e)
        })?;

    Ok(())
}

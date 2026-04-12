use crate::{
    config::{base::Config, tech::Tech},
    logger::level::LogLevel,
};

impl Config {
    /// Lists the current configuration in a human-readable format.
    pub fn list(&self) {
        println!("Listing config settings...");

        println!();

        let logger = &self.logger;

        println!("Logger Settings:");
        println!("  Log Level: {:?}", logger.level.unwrap_or(LogLevel::Info));
        println!("  Log Path: {}", logger.path.as_deref().unwrap_or("N/A"));
        println!("  Log Path is File: {}", logger.path_is_file);
        println!(
            "  Log Date Format (File): {:?}",
            logger.date_format_file.as_deref().unwrap_or("N/A")
        );
        println!("  Log Date Format (Line): {:?}", logger.date_format_line);

        println!();

        let tech = &self.tech;

        match tech {
            Tech::AfXdp(opts) => {
                println!("Tech Settings: AF_XDP");
                println!(
                    "  Queue ID: {}",
                    opts.queue_id
                        .map_or("AUTO".to_string(), |id| id.to_string())
                );
                println!("  Need Wakeup: {}", opts.need_wakeup);
                println!("  Shared UMEM: {}", opts.shared_umem);
                println!("  Batch Size: {}", opts.batch_size);
                println!("  Zero Copy: {}", opts.zero_copy);
            }
        }

        println!();

        let batch = &self.batch;

        println!("Batch Settings:");
        println!("  Number of Batches: {}", batch.batches.len());

        if let Some(overrides) = &batch.ovr_opts {
            println!("  Overrides:");
            if let Some(iface) = &overrides.iface {
                println!("    Interface: {}", iface);
            }
        }

        for (i, batch_data) in batch.batches.iter().enumerate() {
            println!("  Batch {}:", i + 1);
            println!("    Name: {}", batch_data.name.as_deref().unwrap_or("N/A"));
            println!("    Interface: {:?}", batch_data.iface);
            println!()
        }
    }
}

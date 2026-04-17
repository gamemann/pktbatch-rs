use anyhow::Result;

use crate::{
    batch::data::BatchData,
    context::Context,
    util::{net::get_src_mac_addr, sys::get_cpu_count},
};

impl BatchData {
    pub fn exec(&self, ctx: Context, id: u16) -> Result<()> {
        // Retrieve the number of threads we should create.
        let thread_cnt = if self.thread_cnt > 0 {
            self.thread_cnt
        } else {
            get_cpu_count() as u16
        };

        // Prepare block handles.
        let mut block_hdl = Vec::new();

        // Spawn threads.
        for i in 0..thread_cnt {
            let ctx = ctx.clone();
            let data = self.clone();

            let cfg = &ctx.cfg;

            let hdl = std::thread::spawn(move || {
                // We'll want to clone immutable data here so that we aren't waiting for locks from shared threads (hurts performance).
                let tech = ctx.tech.blocking_read().clone();
                let logger = ctx.logger.blocking_read().clone();
                let batch = self.clone();

                logger
                    .log_msg(
                        LogLevel::Info,
                        &format!(
                            "Starting batch execution (batch_id={}, thread_id={})",
                            id, i
                        ),
                    )
                    .ok();

                // We need to retrieve the interface name.
                let if_name = if let Some(name) = tech.if_name {
                    name
                } else {
                    // We can use our util function to get the interface name from the source IP.
                    let src_ip = batch
                        .batches
                        .first()
                        .and_then(|b| b.opt_ip.src.as_ref())
                        .and_then(|src_vec| src_vec.first())
                        .ok_or_else(|| anyhow!("No source IP found to derive interface name"))?;

                    get_ifname_from_src_ip(src_ip)
                        .ok_or_else(|| anyhow!("Could not find interface for IP {}", src_ip))?
                };

                // Determine MAC addresses now.
                let src_mac = if let Some(mac) = batch.opt_eth {
                    mac
                } else {
                    match get_src_mac_addr(if_name) {
                        Ok(mac) => mac,
                        Err(e) => {
                            logger
                                .log_msg(
                                    LogLevel::Error,
                                    &format!(
                                        "Failed to get source MAC address for interface {}: {}",
                                        if_name, e
                                    ),
                                )
                                .ok();
                            return; // Now this exits the thread closure!
                        }
                    }
                };
            });

            if self.wait_for_finish {
                block_hdl.push(hdl);
            }
        }

        let logger = &ctx.logger;

        // Wait for threads to finish if needed.
        for hdl in block_hdl {
            hdl.join().map_err(|e| {
                logger
                    .blocking_read()
                    .log_msg(LogLevel::Error, &format!("Batch thread panicked: {:?}", e))
                    .ok();

                anyhow!("Batch thread panicked when joining: {:?}", e)
            })?;
        }

        Ok(())
    }
}

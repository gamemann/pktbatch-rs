use std::sync::{Arc, atomic::AtomicBool};

use anyhow::{Result, anyhow};

use crate::{
    batch::{data::BatchData, ovr_opts::OvrOpts},
    config::batch::Batch as BatchCfg,
    context::Context,
    logger::level::LogLevel,
};

#[derive(Clone, Default)]
pub struct BatchBase {
    pub cur_batch_id: Option<u16>,
    pub ovr_opts: Option<OvrOpts>,

    pub batches: Vec<BatchData>,
}

pub type Batch = BatchBase;

impl BatchBase {
    pub fn new(batches: Vec<BatchData>, ovr_opts: Option<OvrOpts>) -> Self {
        Self {
            cur_batch_id: None, // internal state
            ovr_opts,
            batches,
        }
    }

    pub async fn start_batches(
        &mut self,
        ctx: Context,
        running: Arc<AtomicBool>,
        iface_fb: Option<String>,
    ) -> Result<()> {
        let logger = &ctx.logger;

        for (i, batch) in self.batches.iter_mut().enumerate() {
            match batch
                .exec(ctx.clone(), i as u16, running.clone(), iface_fb.clone())
                .await
            {
                Ok(_) => (),
                Err(e) => {
                    logger
                        .read()
                        .await
                        .log_msg(
                            LogLevel::Error,
                            &format!("Failed to execute batch {}: {}", i, e),
                        )
                        .ok();

                    return Err(anyhow!("Failed to execute batch {}: {}", i, e));
                }
            }
        }

        Ok(())
    }
}

impl From<BatchCfg> for BatchBase {
    fn from(cfg: BatchCfg) -> Self {
        Self::new(
            cfg.batches.into_iter().map(BatchData::from).collect(),
            cfg.ovr_opts.unwrap().try_into().ok(),
        )
    }
}

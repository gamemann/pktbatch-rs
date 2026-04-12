use anyhow::Result;

use crate::{
    batch::{data::BatchData, ovr_opts::OvrOpts},
    config::batch::Batch as BatchCfg,
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

    pub fn start(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
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

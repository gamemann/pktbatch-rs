pub mod data;
pub mod ovr_opts;

use serde::{Deserialize, Serialize};

use crate::config::batch::{data::BatchData, ovr_opts::BatchOverrideOpts};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Batch {
    #[serde(rename = "overrides")]
    pub ovr_opts: Option<BatchOverrideOpts>,
    pub batches: Vec<BatchData>,
}

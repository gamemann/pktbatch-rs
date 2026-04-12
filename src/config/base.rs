use serde::{Deserialize, Serialize};

use crate::config::{batch::Batch, logger::Logger, tech::Tech};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ConfigBase {
    pub logger: Logger,
    pub tech: Tech,

    pub batch: Batch,
}

pub type Config = ConfigBase;

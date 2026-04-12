pub mod afxdp;

use serde::{Deserialize, Serialize};

use crate::config::tech::afxdp::TechAfXdpOpts;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "opts")]
pub enum Tech {
    AfXdp(TechAfXdpOpts),
}

impl Default for Tech {
    fn default() -> Self {
        Self::AfXdp(TechAfXdpOpts::default())
    }
}

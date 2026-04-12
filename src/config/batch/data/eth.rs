use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct EthOpts {
    pub src_mac: Option<String>,
    pub dst_mac: Option<String>,
}

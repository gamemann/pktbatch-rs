use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct UdpOpts {
    pub src_port: Option<u16>,
    pub dst_port: Option<u16>,
    pub do_csum: bool,
}

impl Default for UdpOpts {
    fn default() -> Self {
        UdpOpts {
            src_port: None,
            dst_port: None,
            do_csum: true,
        }
    }
}

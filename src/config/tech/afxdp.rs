use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct TechAfXdpOpts {
    pub if_name: Option<String>,
    pub queue_id: Option<u16>,
    pub need_wakeup: bool,
    pub shared_umem: bool,
    pub batch_size: u32,
    pub zero_copy: bool,
}

impl Default for TechAfXdpOpts {
    fn default() -> Self {
        Self {
            queue_id: None,
            need_wakeup: false,
            shared_umem: false,
            if_name: None,
            batch_size: 64,
            zero_copy: false, // true is best for performance, but it requires a supported driver and kernel version, so we default to false for better compatibility
        }
    }
}

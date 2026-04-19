pub mod eth;
pub mod ip;
pub mod payload;
pub mod protocol;

use serde::{Deserialize, Serialize};

use crate::config::batch::data::{
    eth::EthOpts, ip::IpOpts, payload::PayloadOpts, protocol::ProtocolOpts,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct BatchData {
    pub name: Option<String>,

    pub iface: Option<String>,

    pub wait_for_finish: bool,

    pub max_pkt: Option<u64>,
    pub max_byt: Option<u64>,

    pub pps: Option<u64>,
    pub bps: Option<u64>,

    pub duration: Option<u64>,
    pub send_interval: Option<u64>,

    pub thread_cnt: Option<u16>,

    pub opt_eth: Option<EthOpts>,
    pub opt_ip: Option<IpOpts>,

    pub opt_protocol: ProtocolOpts,

    pub opt_payload: PayloadOpts,
}

impl Default for BatchData {
    fn default() -> Self {
        BatchData {
            name: None,
            iface: None,
            wait_for_finish: false,
            max_pkt: None,
            max_byt: None,
            pps: None,
            bps: None,
            duration: None,
            send_interval: None,
            thread_cnt: Some(1),
            opt_eth: None,
            opt_ip: Default::default(),
            opt_protocol: ProtocolOpts::Tcp(Default::default()),
            opt_payload: Default::default(),
        }
    }
}

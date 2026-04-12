pub mod icmp;
pub mod tcp;
pub mod udp;

use serde::{Deserialize, Serialize};

use crate::config::batch::data::protocol::{icmp::IcmpOpts, tcp::TcpOpts, udp::UdpOpts};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "opts")]
pub enum ProtocolOpts {
    Tcp(TcpOpts),
    Udp(UdpOpts),
    Icmp(IcmpOpts),
}

impl Default for ProtocolOpts {
    fn default() -> Self {
        ProtocolOpts::Tcp(TcpOpts::default())
    }
}

use crate::config::batch::data::protocol::udp::UdpOpts;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProtocolUdp {
    pub src_port: Option<u16>,
    pub dst_port: Option<u16>,
}

impl From<UdpOpts> for ProtocolUdp {
    fn from(cfg: UdpOpts) -> Self {
        Self {
            src_port: cfg.src_port,
            dst_port: cfg.dst_port,
        }
    }
}

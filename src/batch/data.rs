pub mod eth;
pub mod exec;
pub mod ip;
pub mod payload;
pub mod protocol;

use crate::{
    batch::data::{eth::EthOpts, ip::IpOpts, payload::Payload, protocol::Protocol},
    config::batch::data::BatchData as BatchDataCfg,
    util::sys::get_cpu_count,
};

#[derive(Debug, Clone, Default)]
pub struct BatchData {
    pub id: u16,
    pub name: Option<String>,

    pub iface: Option<String>,

    pub wait_for_finish: bool,

    pub max_pkt: Option<u64>,
    pub max_byt: Option<u64>,

    pub pps: Option<u64>,
    pub bps: Option<u64>,

    pub duration: Option<u64>,
    pub send_interval: Option<u64>,

    pub thread_cnt: u16,

    pub protocol: Protocol,

    pub opt_eth: Option<EthOpts>,
    pub opt_ip: IpOpts,

    pub payload: Option<Payload>,

    pub state_static_pkt: Option<Vec<u8>>,
    pub state_static_payload: Option<Vec<u8>>,
}

impl BatchData {
    pub fn new(
        id: u16,
        name: Option<String>,
        iface: Option<String>,
        wait_for_finish: bool,
        max_pkt: Option<u64>,
        max_byt: Option<u64>,
        pps: Option<u64>,
        bps: Option<u64>,
        duration: Option<u64>,
        send_interval: Option<u64>,
        thread_cnt: u16,
        opt_eth: Option<EthOpts>,
        opt_ip: IpOpts,
        protocol: Protocol,
        payload: Option<Payload>,
    ) -> Self {
        Self {
            id,
            name,
            iface,
            wait_for_finish,
            max_pkt,
            max_byt,
            pps,
            bps,
            duration,
            send_interval,
            thread_cnt,
            opt_eth,
            opt_ip,
            protocol,
            payload,
            state_static_pkt: None,
            state_static_payload: None,
        }
    }
}

impl From<BatchDataCfg> for BatchData {
    fn from(cfg: BatchDataCfg) -> Self {
        // Retrieve thread count.
        // We use core count if none is specified.
        let thread_cnt = cfg.thread_cnt.unwrap_or(get_cpu_count() as u16).max(1);

        Self::new(
            0,
            cfg.name,
            cfg.iface,
            cfg.wait_for_finish,
            cfg.max_pkt,
            cfg.max_byt,
            cfg.pps,
            cfg.bps,
            cfg.duration,
            cfg.send_interval,
            thread_cnt,
            cfg.opt_eth.unwrap().try_into().ok(),
            cfg.opt_ip.unwrap_or_default().into(),
            cfg.opt_protocol.into(),
            cfg.opt_payload.try_into().ok(),
        )
    }
}

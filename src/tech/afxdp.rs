pub mod opt;
pub mod socket;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::{Result, anyhow};
use async_trait::async_trait;

use crate::{
    config::tech::{Tech as TechCfg, afxdp::TechAfXdpOpts as AfXdpOptsCfg},
    context::Context,
    logger::level::LogLevel,
    tech::{
        afxdp::{
            opt::AfXdpOpts,
            socket::{XskTxConfig, XskTxSocket, XskUmem},
        },
        ext::TechExt,
    },
    util::{get_ifname_from_src_ip, sys::get_cpu_count},
};

#[derive(Clone, Default)]
pub struct TechAfXdp {
    pub opts: AfXdpOpts,
    pub sockets: Arc<HashMap<u16, Mutex<XskTxSocket>>>,
}

pub struct AfXdpData {
    pub socket: Mutex<XskTxSocket>,
}

#[async_trait]
impl TechExt for TechAfXdp {
    type Tech = TechAfXdp;
    type Opts = AfXdpOpts;
    type TechData = AfXdpData;

    fn new(opts: Self::Opts) -> Self {
        TechAfXdp {
            opts,
            sockets: Arc::new(HashMap::new()),
        }
    }

    fn get(&self) -> &Self::Tech {
        self
    }

    fn get_mut(&mut self) -> &mut Self::Tech {
        self
    }

    async fn init(&mut self, ctx: Context) -> Result<()> {
        // We need to determine the number of sockets to create.
        let sock_cnt = self
            .opts
            .sock_cnt
            .unwrap_or_else(|| get_cpu_count().max(1) as u16);

        let cfg = &ctx.cfg;

        // Create tech from config.
        let tech = match cfg.read().await.tech.clone() {
            TechCfg::AfXdp(opts) => AfXdpOptsCfg::from(opts.clone()),
        };

        // We need to retrieve the interface name.
        let if_name = if let Some(name) = tech.if_name {
            name
        } else {
            // We can use our util function to get the interface name from the source IP.
            let batch_guard = ctx.batch.read().await;

            let src_ip = batch_guard
                .batches
                .first()
                .and_then(|b| b.opt_ip.src.as_ref())
                .and_then(|src_vec| src_vec.first())
                .ok_or_else(|| anyhow!("No source IP found to derive interface name"))?;

            match get_ifname_from_src_ip(src_ip) {
                Ok(name) => name,
                Err(e) => {
                    return Err(anyhow!(
                        "Could not find interface for IP '{}': {}",
                        src_ip,
                        e
                    ));
                }
            }
        };

        // Create hash map for sockets.
        let mut sock_map = HashMap::new();

        // Check if we need to create umem.
        let shared_umem = if self.opts.shared_umem {
            Some(
                XskUmem::new(&XskTxConfig::from(self.opts.clone())).map_err(|e| {
                    anyhow!("Failed to create shared UMEM for AF_XDP sockets: {}", e)
                })?,
            )
        } else {
            None
        };

        ctx.logger
            .read()
            .await
            .log_msg(
                LogLevel::Info,
                &format!(
                    "Creating {} AF_XDP socket(s) on interface '{}'...",
                    sock_cnt, if_name,
                ),
            )
            .ok();

        for i in 0..sock_cnt {
            let queue_id = self.opts.queue_id.unwrap_or(i);
            let t_id = i + 1;

            ctx.logger
                .read()
                .await
                .log_msg(
                    LogLevel::Trace,
                    &format!(
                        "Creating AF_XDP socket #{} on interface '{}' with queue ID {}...",
                        t_id, if_name, queue_id
                    ),
                )
                .ok();

            // Create XSK socket.
            let mut xsk_cfg = XskTxConfig::from(self.opts.clone());

            xsk_cfg.if_name = if_name.clone();
            xsk_cfg.queue_id = queue_id;

            match XskTxSocket::new(xsk_cfg, shared_umem.as_ref()) {
                Ok(sock) => {
                    ctx.logger
                        .read()
                        .await
                        .log_msg(
                            LogLevel::Info,
                            &format!("Successfully created AF_XDP socket #{}.", t_id),
                        )
                        .ok();

                    sock_map.insert(i, Mutex::new(sock));
                }
                Err(e) => {
                    ctx.logger
                        .read()
                        .await
                        .log_msg(
                            LogLevel::Error,
                            &format!("Failed to create AF_XDP socket #{} :: {}", t_id, e),
                        )
                        .ok();
                }
            }
        }

        self.sockets = Arc::new(sock_map);

        Ok(())
    }

    fn pkt_send(&mut self, _ctx: Context, pkt: &[u8], data: Self::TechData) -> Result<()> {
        let mut sock = data.socket.lock().unwrap();

        sock.send_repeated(pkt)
            .map_err(|e| anyhow!("failed to send packet: {}", e))?;

        Ok(())
    }
}

impl From<AfXdpOptsCfg> for TechAfXdp {
    fn from(afxdp: AfXdpOptsCfg) -> Self {
        Self {
            sockets: Arc::new(HashMap::new()),
            opts: AfXdpOpts::new(
                afxdp.queue_id,
                afxdp.need_wakeup,
                afxdp.shared_umem,
                afxdp.batch_size,
                afxdp.zero_copy,
                afxdp.sock_cnt,
            ),
        }
    }
}

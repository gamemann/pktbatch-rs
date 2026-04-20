use anyhow::{Result, anyhow};
use pnet::packet::tcp::TcpFlags;
use serde::{Deserialize, Serialize};

use crate::{
    batch::data::{
        BatchData,
        payload::Payload,
        protocol::{Protocol, icmp::IcmpOpts, tcp::TcpOpts, udp::UdpOpts},
    },
    cli::arg::Args,
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct BatchOverrideOpts {
    pub iface: Option<String>,
}

pub fn apply_first_batch_overrides(batch: &mut BatchData, args: &Args) -> Result<bool> {
    let mut overridden = false;

    if let Some(iface) = &args.ovr_iface {
        batch.iface = Some(iface.clone());

        overridden = true;
    }

    if let Some(smac) = &args.ovr_smac {
        batch.opt_eth.get_or_insert(Default::default()).src_mac = Some(smac.clone());

        overridden = true;
    }

    if let Some(dmac) = &args.ovr_dmac {
        batch.opt_eth.get_or_insert(Default::default()).dst_mac = Some(dmac.clone());

        overridden = true;
    }

    if let Some(src_ip) = &args.ovr_src_ip {
        batch.opt_ip.src = Some(vec![src_ip.clone()]);

        overridden = true;
    }

    if let Some(dst_ip) = &args.ovr_dst_ip {
        batch.opt_ip.dst = Some(vec![dst_ip.clone()]);

        overridden = true;
    }

    if let Some(thread_cnt) = args.ovr_thread_cnt {
        batch.thread_cnt = thread_cnt as u16;

        overridden = true;
    }

    if let Some(duration) = args.ovr_duration {
        batch.duration = Some(duration as u64);

        overridden = true;
    }

    if let Some(send_interval) = args.ovr_send_interval {
        batch.send_interval = Some(send_interval as u64);
        overridden = true;
    }

    if let Some(pps) = args.ovr_pps {
        batch.pps = Some(pps as u64);
        overridden = true;
    }

    if let Some(bps) = args.ovr_bps {
        batch.bps = Some(bps);
        overridden = true;
    }

    if let Some(wait_for_finish) = args.ovr_wait {
        batch.wait_for_finish = wait_for_finish;
        overridden = true;
    }

    if let Some(max_pkts) = args.ovr_max_pkts {
        batch.max_pkt = Some(max_pkts as u64);
        overridden = true;
    }

    if let Some(max_bytes) = args.ovr_max_bytes {
        batch.max_byt = Some(max_bytes as u64);
        overridden = true;
    }

    if let Some(do_csum) = args.ovr_csum {
        batch.opt_ip.do_csum = do_csum;
        overridden = true;
    }

    if let Some(ttl_min) = args.ovr_min_ttl {
        batch.opt_ip.ttl_min = Some(ttl_min);
        overridden = true;
    }

    if let Some(ttl_max) = args.ovr_max_ttl {
        batch.opt_ip.ttl_max = Some(ttl_max);
        overridden = true;
    }

    if let Some(id_min) = args.ovr_min_id {
        batch.opt_ip.id_min = Some(id_min);
        overridden = true;
    }

    if let Some(id_max) = args.ovr_max_id {
        batch.opt_ip.id_max = Some(id_max);
        overridden = true;
    }

    // Check for transport overrides.
    if let Some(protocol) = &args.ovr_protocol {
        overridden = true;

        match protocol.to_lowercase().as_str() {
            "tcp" => {
                // Check for port overrides.
                let src_port = args.ovr_sport;
                let dst_port = args.ovr_dport;

                // Check for flags.
                let flags = {
                    let mut f = 0u8;

                    if let Some(syn) = args.ovr_syn {
                        if syn {
                            f |= TcpFlags::SYN;
                        }
                    }

                    if let Some(ack) = args.ovr_ack {
                        if ack {
                            f |= TcpFlags::ACK;
                        }
                    }

                    if let Some(fin) = args.ovr_fin {
                        if fin {
                            f |= TcpFlags::FIN;
                        }
                    }

                    if let Some(rst) = args.ovr_rst {
                        if rst {
                            f |= TcpFlags::RST;
                        }
                    }

                    if let Some(psh) = args.ovr_psh {
                        if psh {
                            f |= TcpFlags::PSH;
                        }
                    }

                    if let Some(urg) = args.ovr_urg {
                        if urg {
                            f |= TcpFlags::URG;
                        }
                    }

                    if let Some(ece) = args.ovr_ece {
                        if ece {
                            f |= TcpFlags::ECE;
                        }
                    }

                    if let Some(cwr) = args.ovr_cwr {
                        if cwr {
                            f |= TcpFlags::CWR;
                        }
                    }

                    f
                };

                let do_csum = args.ovr_l4_csum.unwrap_or(true);

                batch.protocol = Protocol::new(
                    "tcp",
                    TcpOpts {
                        src_port,
                        dst_port,
                        flags,
                        do_csum,
                    },
                )
                .map_err(|e| anyhow!("Failed to create TCP protocol options: {}", e))?;
            }
            "udp" => {
                let src_port = args.ovr_sport;
                let dst_port = args.ovr_dport;

                let do_csum = args.ovr_l4_csum.unwrap_or(true);

                batch.protocol = Protocol::new(
                    "udp",
                    UdpOpts {
                        src_port,
                        dst_port,
                        do_csum,
                    },
                )
                .map_err(|e| anyhow!("Failed to create UDP protocol options: {}", e))?;
            }
            "icmp" => {
                let icmp_type = args.ovr_type.unwrap_or(8); // Default to Echo Request
                let icmp_code = args.ovr_code.unwrap_or(0);

                let do_csum = args.ovr_l4_csum.unwrap_or(true);

                batch.protocol = Protocol::new(
                    "icmp",
                    IcmpOpts {
                        icmp_type,
                        icmp_code,
                        do_csum,
                    },
                )
                .map_err(|e| anyhow!("Failed to create ICMP protocol options: {}", e))?;
            }
            _ => return Err(anyhow!("Unsupported protocol override: {}", protocol)),
        }
    }

    // Check if we need to override the payload options.
    if args.ovr_min_len.is_some()
        || args.ovr_max_len.is_some()
        || args.ovr_is_static.is_some()
        || args.ovr_is_string.is_some()
        || args.ovr_is_file.is_some()
        || args.ovr_pl.is_some()
    {
        overridden = true;

        let min_len = args.ovr_min_len;
        let max_len = args.ovr_max_len;
        let is_static = args.ovr_is_static;
        let is_string = args.ovr_is_string;
        let is_file = args.ovr_is_file;
        let exact = args.ovr_pl.clone();

        batch.payload = batch.payload.clone().map(|pl| Payload {
            len_min: min_len.or(pl.len_min),
            len_max: max_len.or(pl.len_max),
            is_static: is_static.unwrap_or(pl.is_static),
            is_string: is_string.unwrap_or(pl.is_string),
            is_file: is_file.unwrap_or(pl.is_file),
            exact: exact.or(pl.exact.clone()),
        });
    }

    Ok(overridden)
}

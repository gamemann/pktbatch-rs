use crate::{
    config::{base::Config, batch::data::protocol::ProtocolOpts, tech::Tech},
    logger::level::LogLevel,
    util::get_cpu_count,
};

impl Config {
    /// Lists the current configuration in a human-readable format.
    pub fn list(&self) {
        println!("Listing config settings...");

        println!();

        let logger = &self.logger;

        println!("Logger Settings:");
        println!("  Log Level: {:?}", logger.level.unwrap_or(LogLevel::Info));
        println!("  Log Path: {}", logger.path.as_deref().unwrap_or("N/A"));
        println!("  Log Path is File: {}", logger.path_is_file);
        println!(
            "  Log Date Format (File): {}",
            logger.date_format_file.as_deref().unwrap_or("N/A")
        );
        println!(
            "  Log Date Format (Line): {}",
            logger.date_format_line.as_deref().unwrap_or("N/A")
        );

        println!();

        let tech = &self.tech;

        match tech {
            Tech::AfXdp(opts) => {
                println!("Tech Settings: AF_XDP");
                println!(
                    "  Queue ID: {}",
                    opts.queue_id
                        .map_or("AUTO".to_string(), |id| id.to_string())
                );
                println!("  Need Wakeup: {}", opts.need_wakeup);
                println!("  Shared UMEM: {}", opts.shared_umem);
                println!("  Batch Size: {}", opts.batch_size);
                println!("  Zero Copy: {}", opts.zero_copy);
            }
        }

        println!();

        let batch = &self.batch;

        println!("Batch Settings:");
        println!("  Number of Batches: {}", batch.batches.len());

        if let Some(overrides) = &batch.ovr_opts {
            println!("  Overrides:");
            if let Some(iface) = &overrides.iface {
                println!("    Interface: {}", iface);
            }
        }

        for (i, batch_data) in batch.batches.iter().enumerate() {
            println!("  Batch {}:", i + 1);
            println!("    Name: {}", batch_data.name.as_deref().unwrap_or("N/A"));
            println!(
                "    Interface: {}",
                batch_data.iface.clone().unwrap_or("N/A".to_string())
            );
            println!("    Wait For Finish: {}", batch_data.wait_for_finish);
            println!(
                "    Send Interval: {}",
                batch_data
                    .send_interval
                    .map_or("N/A".to_string(), |v| v.to_string())
            );
            println!(
                "    Thread Count: {}",
                batch_data.thread_cnt.unwrap_or(get_cpu_count() as u16)
            );
            println!(
                "    Max Packets: {}",
                batch_data
                    .max_pkt
                    .map_or("N/A".to_string(), |v| v.to_string())
            );
            println!(
                "    Max Bytes: {}",
                batch_data
                    .max_byt
                    .map_or("N/A".to_string(), |v| v.to_string())
            );
            println!(
                "    Packets Per Second: {}",
                batch_data.pps.map_or("N/A".to_string(), |v| v.to_string())
            );
            println!(
                "    Bytes Per Second: {}",
                batch_data.bps.map_or("N/A".to_string(), |v| v.to_string())
            );

            {
                let eth_opts = &batch_data.opt_eth;

                println!("    Ethernet Options: {:?}", batch_data.opt_eth);

                println!(
                    "      Source MAC: {}",
                    eth_opts.as_ref().map_or_else(
                        || "N/A".to_string(),
                        |e| e.src_mac.clone().unwrap_or_else(|| "AUTO".to_string())
                    )
                );
                println!(
                    "      Destination MAC: {}",
                    eth_opts.as_ref().map_or_else(
                        || "N/A".to_string(),
                        |e| e.dst_mac.clone().unwrap_or_else(|| "AUTO".to_string())
                    )
                );
            }

            {
                let ip_opts = &batch_data.opt_ip;

                println!("    IP Options:");

                println!(
                    "      ToS: {}",
                    ip_opts
                        .as_ref()
                        .and_then(|i| i.tos)
                        .map_or("N/A".to_string(), |v| v.to_string())
                );

                println!(
                    "      TTL (Min): {}",
                    ip_opts
                        .as_ref()
                        .and_then(|i| i.ttl_min)
                        .map_or("N/A".to_string(), |v| v.to_string())
                );

                println!(
                    "      TTL (Max): {}",
                    ip_opts
                        .as_ref()
                        .and_then(|i| i.ttl_max)
                        .map_or("N/A".to_string(), |v| v.to_string())
                );

                println!(
                    "      ID (Min): {}",
                    ip_opts
                        .as_ref()
                        .and_then(|i| i.id_min)
                        .map_or("N/A".to_string(), |v| v.to_string())
                );

                println!(
                    "      ID (Max): {}",
                    ip_opts
                        .as_ref()
                        .and_then(|i| i.id_max)
                        .map_or("N/A".to_string(), |v| v.to_string())
                );

                println!(
                    "      Do Checksum: {}",
                    ip_opts
                        .as_ref()
                        .map_or("N/A".to_string(), |i| i.do_csum.to_string())
                );

                println!(
                    "      Source IP: {}",
                    ip_opts
                        .as_ref()
                        .and_then(|i| i.src.as_ref())
                        .unwrap_or(&"N/A".to_string())
                );
                println!("      Source IPs:");
                if let Some(srcs) = ip_opts.as_ref().and_then(|i| i.srcs.as_ref()) {
                    for src in srcs {
                        println!("        - {}", src);
                    }
                } else {
                    println!("        N/A");
                }

                println!(
                    "      Destination IP: {}",
                    ip_opts
                        .as_ref()
                        .and_then(|i| i.dst.as_ref())
                        .unwrap_or(&"N/A".to_string())
                );

                println!("      Destination IPs:");
                if let Some(dsts) = ip_opts.as_ref().and_then(|i| i.dsts.as_ref()) {
                    for dst in dsts {
                        println!("        - {}", dst);
                    }
                } else {
                    println!("        N/A");
                }
            }

            {
                let proto_opts = &batch_data.opt_protocol;

                let name = match proto_opts {
                    ProtocolOpts::Tcp(_) => "TCP",
                    ProtocolOpts::Udp(_) => "UDP",
                    ProtocolOpts::Icmp(_) => "ICMP",
                };

                println!("    Protocol Options ({}):", name);

                match proto_opts {
                    ProtocolOpts::Tcp(tcp) => {
                        println!(
                            "      Source Port: {}",
                            tcp.src_port.map_or("RANDOM".to_string(), |p| p.to_string())
                        );

                        println!(
                            "      Destination Port: {}",
                            tcp.dst_port.map_or("RANDOM".to_string(), |p| p.to_string())
                        );

                        println!("      Flag SYN: {}", tcp.flag_syn);
                        println!("      Flag ACK: {}", tcp.flag_ack);
                        println!("      Flag PSH: {}", tcp.flag_psh);
                        println!("      Flag RST: {}", tcp.flag_rst);
                        println!("      Flag FIN: {}", tcp.flag_fin);
                        println!("      Flag URG: {}", tcp.flag_urg);
                        println!("      Flag ECE: {}", tcp.flag_ece);
                        println!("      Flag CWR: {}", tcp.flag_cwr);
                    }
                    ProtocolOpts::Udp(udp) => {
                        println!(
                            "      Source Port: {}",
                            udp.src_port.map_or("RANDOM".to_string(), |p| p.to_string())
                        );

                        println!(
                            "      Destination Port: {}",
                            udp.dst_port.map_or("RANDOM".to_string(), |p| p.to_string())
                        );
                    }
                    ProtocolOpts::Icmp(icmp) => {
                        println!("      ICMP Type: {}", icmp.icmp_type.unwrap_or(8));
                        println!("      ICMP Code: {}", icmp.icmp_code.unwrap_or(0));
                    }
                }
            }

            {
                let payload_opts = &batch_data.opt_payload;

                println!("    Payload Options:");
                println!("      Length (Min): {}", payload_opts.len_min.unwrap_or(0));
                println!("      Length (Max): {}", payload_opts.len_max.unwrap_or(0));
                println!("      Is Static: {}", payload_opts.is_static);
                println!("      Is File: {}", payload_opts.is_file);
                println!("      Is String: {}", payload_opts.is_string);
                println!(
                    "      Exact: {}",
                    payload_opts.exact.clone().unwrap_or("NONE".to_string())
                );
            }

            println!()
        }
    }
}

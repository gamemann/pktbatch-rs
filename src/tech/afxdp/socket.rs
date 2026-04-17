use anyhow::{Context, Result};
use std::{io::Write, num::NonZeroU32, sync::Arc};
use xsk_rs::{
    CompQueue, FrameDesc, TxQueue, Umem,
    config::{BindFlags, FrameSize, Interface, QueueSize, SocketConfig, UmemConfig},
};

use crate::tech::afxdp::opt::AfXdpOpts;

pub struct XskTxSocket {
    pub umem: Umem,
    pub cq: CompQueue,
    pub tx_q: TxQueue,
    pub descs: Vec<FrameDesc>,
    pub batch_size: usize,
}

pub struct XskTxConfig {
    pub if_name: String,
    pub queue_id: u16,
    pub tx_q_size: u32,
    pub cq_size: u32,
    pub frame_size: u32,
    pub frame_count: u32,
    pub batch_size: usize,
    pub need_wakeup: bool,
    pub zero_copy: bool,
    pub shared_umem: bool,
}

impl From<AfXdpOpts> for XskTxConfig {
    fn from(opts: AfXdpOpts) -> Self {
        Self {
            if_name: String::new(), // must be set by caller
            queue_id: opts.queue_id.unwrap_or(0),
            tx_q_size: 1024,
            cq_size: opts.batch_size,
            frame_size: 2048,
            frame_count: opts.batch_size as u32 * 2, // enough frames for 2 batches
            batch_size: opts.batch_size as usize,
            need_wakeup: opts.need_wakeup,
            zero_copy: opts.zero_copy,
            shared_umem: opts.shared_umem,
        }
    }
}

/// Holds a UMEM that can optionally be shared across multiple sockets.
pub struct XskUmem {
    pub umem: Umem,
    pub descs: Vec<FrameDesc>,
}

impl XskUmem {
    pub fn new(config: &XskTxConfig) -> Result<Self> {
        let frame_size: FrameSize = config.frame_size.try_into().context("invalid frame size")?;
        let cq_size: QueueSize = config.cq_size.try_into().context("invalid cq size")?;

        let umem_config = UmemConfig::builder()
            .frame_size(frame_size)
            .comp_queue_size(cq_size)
            .fill_queue_size(config.cq_size.try_into()?)
            .build()
            .context("failed to build umem config")?;

        let frame_count =
            NonZeroU32::new(config.frame_count).context("frame count must be non-zero")?;

        let (umem, descs) = Umem::new(umem_config, frame_count, config.zero_copy)
            .context("failed to create umem")?;

        Ok(Self { umem, descs })
    }
}

impl XskTxSocket {
    /// Create a socket with its own dedicated UMEM.
    pub fn new(config: XskTxConfig) -> Result<Self> {
        let shared_umem = XskUmem::new(&config)?;
        Self::new_with_umem(config, shared_umem)
    }

    /// Create a socket using a pre-existing shared UMEM.
    ///
    /// All sockets sharing a UMEM must be bound to the same interface
    /// but can use different queue IDs.
    pub fn new_shared(config: XskTxConfig, umem: &XskUmem) -> Result<Self> {
        if !config.shared_umem {
            anyhow::bail!("shared_umem must be true when using new_shared()");
        }

        let bind_flags = Self::build_bind_flags(&config);

        let socket_config = SocketConfig::builder()
            .tx_queue_size(
                config
                    .tx_q_size
                    .try_into()
                    .context("invalid tx queue size")?,
            )
            .bind_flags(bind_flags)
            .build();

        let if_name: Interface = config.if_name.parse().context("invalid interface name")?;

        let (tx_q, _rx_q, fq_and_cq) = unsafe {
            xsk_rs::Socket::new(socket_config, &umem.umem, &if_name, config.queue_id as u32)
                .context("failed to create socket with shared umem")?
        };

        let (_fq, cq) =
            fq_and_cq.context("failed to get fill/comp queues for shared umem socket")?;

        Ok(Self {
            umem: umem.umem.clone(),
            cq,
            tx_q,
            descs: umem.descs.clone(),
            batch_size: config.batch_size,
        })
    }

    fn new_with_umem(config: XskTxConfig, xsk_umem: XskUmem) -> Result<Self> {
        let bind_flags = Self::build_bind_flags(&config);

        let socket_config = SocketConfig::builder()
            .tx_queue_size(
                config
                    .tx_q_size
                    .try_into()
                    .context("invalid tx queue size")?,
            )
            .bind_flags(bind_flags)
            .build();

        let if_name: Interface = config.if_name.parse().context("invalid interface name")?;

        let (tx_q, _rx_q, fq_and_cq) = unsafe {
            xsk_rs::Socket::new(
                socket_config,
                &xsk_umem.umem,
                &if_name,
                config.queue_id as u32,
            )
            .context("failed to create socket")?
        };

        let (_fq, cq) = fq_and_cq.context("failed to get fill/comp queues")?;

        Ok(Self {
            umem: xsk_umem.umem,
            cq,
            tx_q,
            descs: xsk_umem.descs,
            batch_size: config.batch_size,
        })
    }

    fn build_bind_flags(config: &XskTxConfig) -> BindFlags {
        let mut flags = BindFlags::empty();

        if config.need_wakeup {
            flags |= BindFlags::XDP_USE_NEED_WAKEUP;
        }

        if config.zero_copy {
            flags |= BindFlags::XDP_ZEROCOPY;
        } else {
            flags |= BindFlags::XDP_COPY;
        }

        flags
    }

    #[inline]
    fn submit_and_drain(&mut self, count: usize) -> Result<()> {
        let submitted = unsafe { self.tx_q.produce(&self.descs[..count]) };
        if submitted == 0 {
            anyhow::bail!("tx queue failed to accept {} frame(s)", count);
        }

        if self.tx_q.needs_wakeup() {
            self.tx_q.wakeup().context("tx wakeup failed")?;
        }

        let mut completed = vec![self.descs[0]; count];
        let mut remaining = count;

        while remaining > 0 {
            let n = unsafe { self.cq.consume(&mut completed[..remaining]) };
            remaining = remaining.saturating_sub(n);
        }

        Ok(())
    }

    /// Send a single packet.
    #[inline]
    pub fn send(&mut self, pkt: &[u8]) -> Result<()> {
        let desc = self
            .descs
            .first_mut()
            .context("no frame descriptors available")?;

        unsafe {
            self.umem.data_mut(desc).cursor().write_all(pkt)?;
        }

        self.submit_and_drain(1)
    }

    /// Send a batch of packets, chunked by `self.batch_size`.
    #[inline]
    pub fn send_batch(&mut self, pkts: &[&[u8]]) -> Result<()> {
        for chunk in pkts.chunks(self.batch_size) {
            let count = chunk.len();

            for (i, pkt) in chunk.iter().enumerate() {
                let desc = self
                    .descs
                    .get_mut(i)
                    .context("not enough frame descriptors")?;
                unsafe {
                    self.umem.data_mut(desc).cursor().write_all(pkt)?;
                }
            }

            self.submit_and_drain(count)?;
        }

        Ok(())
    }

    /// Send the same packet repeated `batch_size` times.
    #[inline]
    pub fn send_repeated(&mut self, pkt: &[u8]) -> Result<()> {
        let count = self.batch_size;

        for i in 0..count {
            let desc = self
                .descs
                .get_mut(i)
                .context("not enough frame descriptors")?;

            unsafe {
                self.umem.data_mut(desc).cursor().write_all(pkt)?;
            }
        }

        self.submit_and_drain(count)
    }
}

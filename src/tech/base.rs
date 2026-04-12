use crate::{
    config::tech::Tech as TechCfg,
    tech::afxdp::{TechAfXdp, opt::AfXdpOpts},
};

#[derive(Clone)]
pub enum TechBase {
    AfXdp(TechAfXdp),
}

pub type Tech = TechBase;

impl From<TechCfg> for TechBase {
    fn from(tech: TechCfg) -> Self {
        match tech {
            TechCfg::AfXdp(opts) => TechBase::AfXdp(TechAfXdp {
                opts: AfXdpOpts {
                    queue_id: opts.queue_id,
                    need_wakeup: opts.need_wakeup,
                    shared_umem: opts.shared_umem,
                    batch_size: opts.batch_size,
                    zero_copy: opts.zero_copy,
                },
            }),
        }
    }
}

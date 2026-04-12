use crate::config::batch::ovr_opts::BatchOverrideOpts as OvrOptsCfg;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OvrOpts {
    pub iface: Option<String>,
}

impl From<OvrOptsCfg> for OvrOpts {
    fn from(cfg: OvrOptsCfg) -> Self {
        Self { iface: cfg.iface }
    }
}

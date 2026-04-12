use crate::config::batch::data::eth::EthOpts as EthOptsCfg;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct EthOpts {
    src_mac: Option<String>,
    dst_mac: Option<String>,
}

impl From<EthOptsCfg> for EthOpts {
    fn from(cfg: EthOptsCfg) -> Self {
        Self {
            src_mac: cfg.src_mac,
            dst_mac: cfg.dst_mac,
        }
    }
}

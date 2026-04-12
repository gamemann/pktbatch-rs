use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{cli::base::Cli, config::base::Config, logger::base::Logger};

pub struct ContextData {
    pub cfg: RwLock<Config>,
    pub logger: RwLock<Logger>,
    pub cli: RwLock<Cli>,
}

pub type Context = Arc<ContextData>;

impl ContextData {
    pub fn new(cfg: Config, logger: Logger, cli: Cli) -> Arc<Self> {
        Arc::new(Self {
            cfg: RwLock::new(cfg),
            logger: RwLock::new(logger),
            cli: RwLock::new(cli),
        })
    }
}

use crate::cli::arg::Args;

use clap::Parser;

#[derive(Clone, Default)]
pub struct CliBase {
    pub args: Args,
}

pub type Cli = CliBase;

impl CliBase {
    pub fn parse() -> Self {
        let args = Args::parse();

        Self { args }
    }
}

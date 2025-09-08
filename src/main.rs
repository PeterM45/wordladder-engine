use anyhow::Result;
use clap::Parser;
use wordladder_engine::cli::{Cli, run};

fn main() -> Result<()> {
    let cli = Cli::parse();
    run(cli)
}

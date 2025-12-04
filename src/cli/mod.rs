pub mod  args;

use  args::Cli;
use clap::Parser;

use crate::scraper::swisstarget;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let rt = tokio::runtime::Runtime::new()?;

    rt.block_on(async {
        swisstarget::run(&cli.smiles, &cli.output).await?;
        Ok::<(), Box<dyn std::error::Error>>(())
    })?;

    Ok(())
}
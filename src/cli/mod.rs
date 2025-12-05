pub mod args;

use args::Cli;
use clap::Parser;
use crate::cli::args::Commands;
use crate::scraper::swisstarget;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let rt = tokio::runtime::Runtime::new()?;

    rt.block_on(async {
        match cli.command {
            Some(Commands::Swisstarget { smiles, output }) => {
                swisstarget::run(smiles, &output).await?;
            }
            None => {
                println!("No command provided. Use --help for usage.");
            }
        }

        Ok::<(), Box<dyn std::error::Error>>(())
    })?;

    Ok(())
}


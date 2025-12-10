pub mod args;

use crate::cli::args::Commands;
use crate::scraper::swisstarget;
use crate::tool;
use crate::tool::panther::run_panther_analysis;
use args::Cli;
use clap::Parser;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let rt = tokio::runtime::Runtime::new()?;

    rt.block_on(async {
        match cli.command {
            Some(Commands::Swisstarget { smiles, output }) => {
                swisstarget::run(smiles, &output).await?;
            }
            Some(Commands::Data { location, output }) => {
                let _ = tool::data::make_csv_from_swisstarget(&location, &output);
            }
            Some(Commands::Panther { csv_path, output }) => {
                run_panther_analysis(&csv_path, &output).await?;
            }
            Some(Commands::Pantherold {
                genes,
                organism,
                output,
                file,
            }) => {
                tool::pantherold::panther_protein_class_to_txt(genes, &organism, &output, file)
                    .await?;
            }

            None => {
                println!("No command provided. Use --help for usage.");
            }
        }

        Ok::<(), Box<dyn std::error::Error>>(())
    })?;

    Ok(())
}

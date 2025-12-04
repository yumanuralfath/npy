use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "npy",
    about = "Data pipeline and scraper for network pharmacology by Yumana",
    version
)]
pub struct Cli{
    #[arg(short, long)]
    pub smiles: String,

    #[arg(short, long)]
    pub output: String
}
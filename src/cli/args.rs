use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "npy",
    version
)]
pub struct Cli{
    #[arg(short, long)]
    pub smiles: String,

    #[arg(short, long, default_value = "output.csv")]
    pub output: String
}
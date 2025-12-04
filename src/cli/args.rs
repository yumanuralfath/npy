use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "npy", version)]
pub struct Cli {
    #[arg(short, long, value_delimiter = ',')]
    pub smiles: Vec<String>,

    #[arg(short, long, default_value = "output")]
    pub output: String,
}


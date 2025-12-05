use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "npy", version)]
#[command(about = "ask = yuma@yumana.my.id")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(
        short,
        long,
        help = "Verbose mode for see full log when program running"
    )]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(
        about = "Target prediction with SwissTargetPrediction",
        after_help = "EXAMPLES:\n\
                    npy swisstarget -s \"CCO\" -o hasil\n\
                    npy swisstarget --smiles \"CCN(CC)CC\" --output prediksi\n\
            npy swisstarget -s \"cco,ccn\" #for multi smiles"
    )]
    Swisstarget {
        #[arg(short, long, value_delimiter = ',', help = "SMILES string for analyze")]
        smiles: Vec<String>,

        #[arg(short, long, default_value = "output")]
        output: String,
    },
}

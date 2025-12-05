use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "npy", version)]
#[command(about = "yuma@yumana.my.id")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(short, long, help = "Mode verbose untuk melihat full log ketika running")]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(
        about = "Prediksi target protein menggunakan SwissTargetPrediction",
        after_help = "EXAMPLES:\n\
                    npy swisstarget -s \"CCO\" -o hasil\n\
                    npy swisstarget --smiles \"CCN(CC)CC\" --output prediksi"
    )]
    Swisstarget {
        #[arg(short, long, value_delimiter = ',', help = "SMILES string yang akan diproses")]
        smiles: Vec<String>,

        #[arg(short, long, default_value = "output")]
        output: String,
    }
}
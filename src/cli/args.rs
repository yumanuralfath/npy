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
                    npy swisstarget --smiles \"CCN(CC)CC\" --output hasil\n\
            npy swisstarget -s \"cco,ccn\" -o hasil   #for multi smiles"
    )]
    Swisstarget {
        #[arg(short, long, value_delimiter = ',', help = "SMILES string for analyze")]
        smiles: Vec<String>,

        #[arg(short, long, default_value = "output", help = "Output file location")]
        output: String,
    },
    #[command(about = "Make csv data from swisstarget csv")]
    Data {
        #[arg(
            short,
            long,
            help = "location csv file from swisstarfet prediction",
            default_value = "output/swiss_target_prediction"
        )]
        location: String,

        #[arg(
            short,
            long,
            help = "location output file",
            default_value = "output/data"
        )]
        output: String,
    },
}

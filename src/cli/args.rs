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
    #[command(about = "[DEPERECATED] Gene list analyze with pantherdb do not use this")]
    Pantherold {
        #[arg(
            short,
            long,
            help = "file with have unique gene",
            default_value = "output/data/unique_genes.csv"
        )]
        file: Option<String>,

        #[arg(short, long, help = "id unique gene")]
        genes: Option<Vec<String>>,

        #[arg(
            long,
            help = "select organism default homo sapiens",
            default_value = "9606"
        )]
        organism: String,

        #[arg(long, help = "output data", default_value = "output/data/panther.txt")]
        output: String,
    },
    #[command(about = "Gene list analyze with pantherdb")]
    Panther {
        #[arg(
            short,
            long,
            help = "csv file with have unique gene",
            default_value = "output/data/unique_genes.csv"
        )]
        csv_path: String,

        #[arg(
            long,
            help = "output Result data",
            default_value = "output/data/pantherdb_result.txt"
        )]
        output: String,
    },
    #[command(about = "Venny analyst to see overlaps and differences in a venn diagram")]
    Venny {
        #[arg(
            short,
            long,
            help = "genecard csv file path",
            default_value = "output/Genecards/Genecards.csv"
        )]
        genecards: String,

        #[arg(
            short,
            long,
            help = "unique_genes csv file path",
            default_value = "output/data/unique_genes.csv"
        )]
        unique_genes: String,

        #[arg(
            long,
            help = "output Result data",
            default_value = "output/data/venny.csv"
        )]
        output: String,
    },

    #[command(
        about = "Protein-protein interaction analysis with STRING database",
        after_help = "EXAMPLES:\n\
                    npy string -c proteins.csv -s 9606\n\
                    npy string --csv proteins.csv --species 9606 --output hasil\n\
                    npy string -c genes.csv -s 10090  #for mouse"
    )]
    String {
        #[arg(
            short,
            long,
            help = "CSV file with protein IDs in column A (no header)",
            default_value = "output/data/venny.csv"
        )]
        csv: String,
        #[arg(
            short,
            long,
            help = "Species NCBI taxonomy ID (9606=Human, 10090=Mouse)",
            default_value = "9606"
        )]
        species: u32,
        #[arg(
            short,
            long,
            help = "Output directory for results",
            default_value = "output/string"
        )]
        output: String,
    },

    #[command(
        about = "Run all pipeline at first",
        after_help = "EXAMPLES:\n\
                    npy run -s smiles.csv --genecards output/Genecards/Genecards.csv"
    )]
    Run {
        #[arg(
            short,
            long,
            help = "CSV containing SMILES",
            default_value = "output/smiles/smiles.csv"
        )]
        smiles_csv: String,

        #[arg(
            long,
            help = "Genecard CSV file path",
            default_value = "output/Genecards/Genecards.csv"
        )]
        genecards: String,
    },
}

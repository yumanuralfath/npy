use crate::scraper::swisstarget;
use crate::tool::panther::run_panther_analysis;
use crate::tool::runall::run_all_pipeline;
use crate::tool::string::handle_string_command;
use crate::tool::{self, venny};
use std::io::{self, Write};

pub async fn start_repl() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║         NPY Interactive REPL Mode v1.0           ║");
    println!("║          ask = yuma@yumana.my.id                 ║");
    println!("╚═══════════════════════════════════════════════════╝");
    println!("\nType 'help' for available commands or 'exit' to quit.\n");

    loop {
        print!("npy> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "exit" || input == "quit" {
            println!("Goodbye! 👋");
            break;
        }

        if input == "help" {
            print_help();
            continue;
        }

        if input == "clear" || input == "cls" {
            print!("\x1B[2J\x1B[1;1H");
            continue;
        }

        // Parse command
        let args: Vec<&str> = input.split_whitespace().collect();
        if args.is_empty() {
            continue;
        }

        match execute_command(args).await {
            Ok(_) => println!("✓ Command completed successfully\n"),
            Err(e) => eprintln!("✗ Error: {}\n", e),
        }
    }

    Ok(())
}

async fn execute_command(args: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    match args[0] {
        "init" => {
            tool::init::init_default_files()?;
        }

        "swisstarget" => {
            let (smiles, output) = parse_swisstarget_args(&args)?;
            swisstarget::run(smiles, &output).await?;
        }

        "data" => {
            let (location, output) = parse_data_args(&args);
            tool::data::make_csv_from_swisstarget(&location, &output)?;
        }

        "panther" => {
            let (csv_path, output) = parse_panther_args(&args);
            run_panther_analysis(&csv_path, &output).await?;
        }

        "venny" => {
            let (genecards, unique_genes, output) = parse_venny_args(&args);
            venny::compare_genecards_and_list(&genecards, &unique_genes, &output);
        }

        "string" => {
            let (csv, species, output) = parse_string_args(&args)?;
            handle_string_command(&csv, species, &output).await?;
        }

        "run" => {
            let (smiles_csv, genecards) = parse_run_args(&args);
            run_all_pipeline(&smiles_csv, &genecards).await?;
        }

        _ => {
            return Err(format!(
                "Unknown command: '{}'. Type 'help' for available commands.",
                args[0]
            )
            .into());
        }
    }

    Ok(())
}

fn parse_swisstarget_args(
    args: &[&str],
) -> Result<(Vec<String>, String), Box<dyn std::error::Error>> {
    let mut smiles = Vec::new();
    let mut output = String::from("output");

    let mut i = 1;
    while i < args.len() {
        match args[i] {
            "-s" | "--smiles" => {
                if i + 1 < args.len() {
                    smiles = args[i + 1].split(',').map(|s| s.to_string()).collect();
                    i += 2;
                } else {
                    return Err("Missing value for --smiles".into());
                }
            }
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    output = args[i + 1].to_string();
                    i += 2;
                } else {
                    return Err("Missing value for --output".into());
                }
            }
            _ => i += 1,
        }
    }

    if smiles.is_empty() {
        return Err("SMILES string required. Use: swisstarget -s \"CCO\" -o output".into());
    }

    Ok((smiles, output))
}

fn parse_data_args(args: &[&str]) -> (String, String) {
    let mut location = String::from("output/swiss_target_prediction");
    let mut output = String::from("output/data");

    let mut i = 1;
    while i < args.len() {
        match args[i] {
            "-l" | "--location" => {
                if i + 1 < args.len() {
                    location = args[i + 1].to_string();
                    i += 2;
                }
            }
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    output = args[i + 1].to_string();
                    i += 2;
                }
            }
            _ => i += 1,
        }
    }

    (location, output)
}

fn parse_panther_args(args: &[&str]) -> (String, String) {
    let mut csv_path = String::from("output/data/unique_genes.csv");
    let mut output = String::from("output/data/pantherdb_result.txt");

    let mut i = 1;
    while i < args.len() {
        match args[i] {
            "-c" | "--csv-path" => {
                if i + 1 < args.len() {
                    csv_path = args[i + 1].to_string();
                    i += 2;
                }
            }
            "--output" => {
                if i + 1 < args.len() {
                    output = args[i + 1].to_string();
                    i += 2;
                }
            }
            _ => i += 1,
        }
    }

    (csv_path, output)
}

fn parse_venny_args(args: &[&str]) -> (String, String, String) {
    let mut genecards = String::from("output/Genecards/Genecards.csv");
    let mut unique_genes = String::from("output/data/unique_genes.csv");
    let mut output = String::from("output/data/venny.csv");

    let mut i = 1;
    while i < args.len() {
        match args[i] {
            "-g" | "--genecards" => {
                if i + 1 < args.len() {
                    genecards = args[i + 1].to_string();
                    i += 2;
                }
            }
            "-u" | "--unique-genes" => {
                if i + 1 < args.len() {
                    unique_genes = args[i + 1].to_string();
                    i += 2;
                }
            }
            "--output" => {
                if i + 1 < args.len() {
                    output = args[i + 1].to_string();
                    i += 2;
                }
            }
            _ => i += 1,
        }
    }

    (genecards, unique_genes, output)
}

fn parse_string_args(args: &[&str]) -> Result<(String, u32, String), Box<dyn std::error::Error>> {
    let mut csv = String::from("output/data/venny.csv");
    let mut species = 9606u32;
    let mut output = String::from("output/string");

    let mut i = 1;
    while i < args.len() {
        match args[i] {
            "-c" | "--csv" => {
                if i + 1 < args.len() {
                    csv = args[i + 1].to_string();
                    i += 2;
                }
            }
            "-s" | "--species" => {
                if i + 1 < args.len() {
                    species = args[i + 1].parse()?;
                    i += 2;
                }
            }
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    output = args[i + 1].to_string();
                    i += 2;
                }
            }
            _ => i += 1,
        }
    }

    Ok((csv, species, output))
}

fn parse_run_args(args: &[&str]) -> (String, String) {
    let mut smiles_csv = String::from("output/smiles/smiles.csv");
    let mut genecards = String::from("output/Genecards/Genecards.csv");

    let mut i = 1;
    while i < args.len() {
        match args[i] {
            "-s" | "--smiles-csv" => {
                if i + 1 < args.len() {
                    smiles_csv = args[i + 1].to_string();
                    i += 2;
                }
            }
            "--genecards" => {
                if i + 1 < args.len() {
                    genecards = args[i + 1].to_string();
                    i += 2;
                }
            }
            _ => i += 1,
        }
    }

    (smiles_csv, genecards)
}

fn print_help() {
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║              Available Commands                   ║");
    println!("╚═══════════════════════════════════════════════════╝");
    println!();
    println!("  init");
    println!("    Initialize folder structure and default files");
    println!();
    println!("  swisstarget -s <smiles> -o <output>");
    println!("    Target prediction with SwissTargetPrediction");
    println!("    Example: swisstarget -s \"CCO\" -o hasil");
    println!("             swisstarget -s \"cco,ccn\" -o hasil");
    println!();
    println!("  data -l <location> -o <output>");
    println!("    Make CSV data from swisstarget results");
    println!();
    println!("  panther -c <csv_path> --output <output>");
    println!("    Gene list analysis with pantherdb");
    println!();
    println!("  venny -g <genecards> -u <unique_genes> --output <output>");
    println!("    Venny analysis for overlaps in Venn diagram");
    println!();
    println!("  string -c <csv> -s <species> -o <output>");
    println!("    Protein-protein interaction with STRING database");
    println!("    Example: string -c proteins.csv -s 9606");
    println!();
    println!("  run -s <smiles_csv> --genecards <genecards>");
    println!("    Run complete pipeline");
    println!();
    println!("  help     - Show this help message");
    println!("  clear    - Clear screen");
    println!("  exit     - Exit REPL mode");
    println!();
}

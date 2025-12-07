use csv::{ReaderBuilder, Writer, WriterBuilder};
use regex::Regex;
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

fn list_csv_files(folder: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    if !folder.is_dir() {
        return Err("Location path must be a folder".into());
    }

    let mut files = Vec::new();
    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("csv") {
            files.push(path);
        }
    }
    Ok(files)
}

fn split_genes(value: &str) -> Vec<String> {
    let re = Regex::new(r"[↔⇄→,;|/\s]+").unwrap();

    re.split(value)
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn process_csv_file(
    file_path: &Path,
    header_target: &str,
    wtr_all: &mut Writer<std::fs::File>,
    wtr_unique: &mut Writer<std::fs::File>,
    unique_set: &mut HashSet<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Processing file: {:?}", file_path);

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)?;

    let headers = rdr.headers()?.clone();
    let idx = match headers.iter().position(|h| h == header_target) {
        Some(i) => i,
        None => {
            eprintln!("Header {} not found in file {:?}", header_target, file_path);
            return Ok(());
        }
    };

    for result in rdr.records() {
        let record = result?;
        let raw_value = record[idx].trim();

        let genes = split_genes(raw_value);

        for gene in genes {
            wtr_all.write_record([&gene])?;

            if unique_set.insert(gene.clone()) {
                wtr_unique.write_record([&gene])?;
            }
        }
    }

    Ok(())
}

pub fn make_csv_from_swisstarget(
    location: &str,
    output: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let folder = Path::new(location);
    let header_target = "Common Name";

    fs::create_dir_all(output)?;

    let all_genes_path = format!("{}/all_genes.csv", output);
    let unique_genes_path = format!("{}/unique_genes.csv", output);

    println!("All genes output  : {}", all_genes_path);
    println!("Unique genes output : {}", unique_genes_path);

    let mut wtr_all = WriterBuilder::new().from_path(&all_genes_path)?;
    let mut wtr_unique = WriterBuilder::new().from_path(&unique_genes_path)?;

    let mut unique_set = HashSet::new();

    let csv_files = list_csv_files(folder)?;

    for file in csv_files {
        process_csv_file(
            &file,
            header_target,
            &mut wtr_all,
            &mut wtr_unique,
            &mut unique_set,
        )?;
    }

    wtr_all.flush()?;
    wtr_unique.flush()?;

    Ok(())
}

use csv::ReaderBuilder;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

use crate::tool::verbose;

pub fn compare_genecards_and_list(csvfirst: &str, csvsec: &str, output_csv: &str) {
    verbose("Starting Venny comparison...");

    verbose(format!("Reading Genecards file: {}", csvfirst));

    let mut rdr1 = ReaderBuilder::new()
        .has_headers(true)
        .from_path(csvfirst)
        .expect("Cannot open Genecards CSV");

    let mut set_first = HashSet::new();
    for result in rdr1.records() {
        let record = result.unwrap();
        let gene = record.get(0).unwrap().trim().to_string();
        set_first.insert(gene);
    }

    verbose(format!("Loaded {} genes from Genecards", set_first.len()));

    verbose(format!("Reading Unique Genes file: {}", csvsec));

    let mut rdr2 = ReaderBuilder::new()
        .has_headers(false)
        .from_path(csvsec)
        .expect("Cannot open unique_genes.csv");

    let mut set_second = HashSet::new();
    for result in rdr2.records() {
        let record = result.unwrap();
        let gene = record.get(0).unwrap().trim().to_string();
        set_second.insert(gene);
    }

    verbose(format!(
        "Loaded {} genes from Unique Genes",
        set_second.len()
    ));

    let intersection: HashSet<_> = set_first.intersection(&set_second).cloned().collect();
    let only_first: HashSet<_> = set_first.difference(&set_second).cloned().collect();
    let only_second: HashSet<_> = set_second.difference(&set_first).cloned().collect();

    let n1 = set_first.len() as f64;
    let n2 = set_second.len() as f64;
    let ni = intersection.len() as f64;

    verbose("Finished computing intersections.");

    // ======== HUMAN OUTPUT (terminal) ========
    println!("=== Venny Comparison Result ===");
    println!("First CSV (Genecards): {} items", n1);
    println!("Second CSV (Unique Genes): {} items", n2);

    println!(
        "\nIntersection: {} items ({:.2}% of first, {:.2}% of second)",
        ni,
        (ni / n1) * 100.0,
        (ni / n2) * 100.0
    );

    println!(
        "\nOnly in First CSV: {} items ({:.2}%)",
        only_first.len(),
        (only_first.len() as f64 / n1) * 100.0
    );

    println!(
        "Only in Second CSV: {} items ({:.2}%)",
        only_second.len(),
        (only_second.len() as f64 / n2) * 100.0
    );

    println!("\n=== Intersection Gene List ===");

    let mut inter_sorted: Vec<_> = intersection.into_iter().collect();
    inter_sorted.sort();
    for g in &inter_sorted {
        verbose(g);
    }
    // ======== SAVE FINAL CSV WITHOUT HEADER ========
    verbose(format!("Saving CSV intersection result to {}", output_csv));

    let mut file = File::create(output_csv).expect("Cannot create intersection csv");
    for g in inter_sorted {
        writeln!(file, "{}", g).unwrap();
    }

    println!("\nIntersection list saved to: {}", output_csv);
}

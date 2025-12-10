use crate::scraper::swisstarget;
use crate::tool::string::handle_string_command;
use crate::tool::{
    data::make_csv_from_swisstarget, panther::run_panther_analysis,
    venny::compare_genecards_and_list,
};

use csv::ReaderBuilder;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{error::Error, time::Duration};

pub async fn run_all_pipeline(
    smiles_csv_path: &str,
    genecard_path: &str,
) -> Result<(), Box<dyn Error>> {
    let mp = MultiProgress::new();

    // STYLE: untuk bar finite
    let style_bar = ProgressStyle::with_template(
        "\
[{elapsed_precise}] {wide_bar:.cyan/blue} {pos}/{len} \
({percent}%) - ETA: {eta_precise} - {msg}",
    )?;

    // STYLE: untuk step yang tidak punya total (API calls)
    let style_spinner = ProgressStyle::with_template("[{elapsed_precise}] {spinner:.green} {msg}")?
        .tick_strings(&["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"]);

    // STYLE: step selesai
    let style_done = ProgressStyle::with_template("[{elapsed_precise}] ✔ {msg}")?;

    // ---------------------------------------------------------
    // 1. READ SMILES CSV (Finite progress bar)
    // ---------------------------------------------------------
    let pb_read = mp.add(ProgressBar::new_spinner());
    pb_read.set_style(style_spinner.clone());
    pb_read.enable_steady_tick(Duration::from_millis(120));
    pb_read.set_message("Counting SMILES...");

    // Count first
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path(smiles_csv_path)?;

    let mut total = 0usize;
    for _ in reader.records() {
        total += 1;
    }

    pb_read.finish_and_clear();

    // REAL loading bar
    let pb_csv = mp.add(ProgressBar::new(total as u64));
    pb_csv.set_style(style_bar.clone());
    pb_csv.set_message("Reading SMILES from CSV");

    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path(smiles_csv_path)?;

    let mut smiles_list = Vec::new();

    for result in reader.records() {
        let record = result?;
        if let Some(sm) = record.get(0) {
            smiles_list.push(sm.to_string());
        }
        pb_csv.inc(1);
    }

    pb_csv.finish_with_message("SMILES loaded");

    // ---------------------------------------------------------
    // 2. SWISSTARGET (Indeterminate spinner + ETA-like feel)
    // ---------------------------------------------------------
    let pb_st = mp.add(ProgressBar::new_spinner());
    pb_st.set_style(style_spinner.clone());
    pb_st.enable_steady_tick(Duration::from_millis(100));
    pb_st.set_message("Running SwissTargetPrediction API…");

    swisstarget::run(smiles_list, "output/swiss_target_prediction").await?;

    pb_st.set_style(style_done.clone());
    pb_st.finish_with_message("SwissTargetPrediction completed");

    // ---------------------------------------------------------
    // 3. MAKE CSV DATA
    // ---------------------------------------------------------
    let pb_data = mp.add(ProgressBar::new_spinner());
    pb_data.set_style(style_spinner.clone());
    pb_data.enable_steady_tick(Duration::from_millis(100));
    pb_data.set_message("Generating merged DATA CSV…");

    make_csv_from_swisstarget("output/swiss_target_prediction", "output/data")?;

    pb_data.set_style(style_done.clone());
    pb_data.finish_with_message("Merged DATA CSV created");

    // ---------------------------------------------------------
    // 4. RUN PANTHER
    // ---------------------------------------------------------
    let pb_panther = mp.add(ProgressBar::new_spinner());
    pb_panther.set_style(style_spinner.clone());
    pb_panther.enable_steady_tick(Duration::from_millis(100));
    pb_panther.set_message("Running PantherDB analysis…");

    run_panther_analysis(
        "output/data/unique_genes.csv",
        "output/data/pantherdb_result.txt",
    )
    .await?;

    pb_panther.set_style(style_done.clone());
    pb_panther.finish_with_message("PantherDB analysis completed");

    // ---------------------------------------------------------
    // 5. VENNY
    // ---------------------------------------------------------
    let pb_venny = mp.add(ProgressBar::new_spinner());
    pb_venny.set_style(style_spinner.clone());
    pb_venny.enable_steady_tick(Duration::from_millis(100));
    pb_venny.set_message("Comparing Genecards vs Unique Genes…");

    compare_genecards_and_list(
        genecard_path,
        "output/data/unique_genes.csv",
        "output/data/venny.csv",
    );

    pb_venny.set_style(style_done.clone());
    pb_venny.finish_with_message("Venny CSV generated");

    // ---------------------------------------------------------
    // 6. STRING NETWORK
    // ---------------------------------------------------------
    let pb_string = mp.add(ProgressBar::new_spinner());
    pb_string.set_style(style_spinner.clone());
    pb_string.enable_steady_tick(Duration::from_millis(80));
    pb_string.set_message("Fetching STRING PPI network…");

    handle_string_command("output/data/venny.csv", 9606, "output/string").await?;

    pb_string.set_style(style_done.clone());
    pb_string.finish_with_message("STRING PPI completed");

    // ---------------------------------------------------------
    // DONE
    // ---------------------------------------------------------
    let pb_done = mp.add(ProgressBar::new(0));
    pb_done.set_style(style_done);
    pb_done.finish_with_message("🎉 Pipeline completed successfully!");

    Ok(())
}

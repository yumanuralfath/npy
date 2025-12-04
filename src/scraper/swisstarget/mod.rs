pub mod client;
pub mod job;
pub mod parse;
pub mod poll;

pub async fn run(smiles: Vec<String>, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = client::build_client()?;

    let target_folder = format!("{}/swiss_target_prediction", output);
    std::fs::create_dir_all(&target_folder)?;

    for smile in smiles {
        println!("[SwissTarget] Processing SMILES: {smile}");

        let job_id = job::submit_smiles(&client, &smile).await?;
        println!("[SwissTarget] Job ID: {}", job_id);

        let html = poll::fetch_result_html(&client, &job_id).await?;

        let csv_path = format!("{}/{}.csv", target_folder, safe_name(&smile));
        parse::parse_and_save_csv(&html, &csv_path)?;

        println!("[SwissTarget] Saved into: {}", csv_path);
    }
    Ok(())
}

pub fn safe_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect()
}

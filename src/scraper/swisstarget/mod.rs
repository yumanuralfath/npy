pub mod client;
pub mod job;
pub mod poll;
pub mod parse;

pub async fn run(
    smiles: &str,
    output: &str,
)-> Result<(), Box<dyn std::error::Error>> {
    let client =  client::build_client()?;

    let job_id = job::submit_smiles(&client, smiles).await?;
    println!("[SwissTarget] Job ID: {} ▄︻╦芫≡══-- ", job_id);

    let html =  poll::fetch_result_html(&client, &job_id).await?;

    parse::parse_and_save_csv(&html, output)?;

    println!("[SwissTarget] saved into: {} ✌︎㋡", output);

    Ok(())
}
use regex::Regex;
use reqwest::Client;

use crate::config::get_config;

pub async fn submit_smiles(
    client: &Client,
    smiles: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let cfg = &get_config().swisstarget;

    let params = [
        ("organism", "Homo_sapiens"),
        ("smiles", smiles),
        ("Example", ""),
        ("ioi", "2"),
    ];

    let url = format!("{}{}", cfg.base_url, cfg.predict_endpoint);

    let res = client.post(&url).form(&params).send().await?;
    let html = res.text().await?;
    println!("html: {html}");

    let re = Regex::new(r#"result\.php\?job=([0-9]+)&organism=Homo_sapiens"#)?;
    let caps = re.captures(&html).ok_or(
        "Job ID not Found, Server Error, please check smiles or test at website swisstarget",
    )?;

    Ok(caps.get(1).unwrap().as_str().to_string())
}

use clap::Parser;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

use crate::{cli::args::Cli, config::get_config};

pub fn build_client() -> Result<reqwest::Client, Box<dyn std::error::Error>> {
    let cfg = &get_config().swisstarget;
    let cli = Cli::parse();

    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_str(&cfg.headers.user_agent)?,
    );
    headers.insert("Origin", HeaderValue::from_str(&cfg.headers.origin)?);
    headers.insert("Referer", HeaderValue::from_str(&cfg.headers.referer)?);

    if cli.verbose {
        println!("Mode Verbose active cuy");
        println!("{:?}", &headers);
    }
    
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .cookie_store(true)
        .build()?;

    Ok(client)
}

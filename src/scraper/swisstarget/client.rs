use reqwest::header::{HeaderMap, HeaderValue};

use crate::config::get_config;

pub fn build_client() -> Result<reqwest::Client, Box<dyn std::error::Error>>{
    let cfg = &get_config().swisstarget;

    let mut headers = HeaderMap::new();
    headers.insert("User_Agent", HeaderValue::from_str(&cfg.headers.user_agent)?);
    headers.insert("Origin", HeaderValue::from_str(&cfg.headers.origin)?);
    headers.insert("Referer", HeaderValue::from_str(&cfg.headers.referer)?);
    
    println!("{:?}", &headers);
    let client = reqwest::Client::builder().default_headers(headers).cookie_store(true).build()?;

    
    Ok(client)
}
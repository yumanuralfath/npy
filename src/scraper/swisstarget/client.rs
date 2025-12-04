use reqwest::header::{HeaderMap, HeaderValue};

use crate::config::get_config;

pub fn build_client() -> Result<reqwest::Client, Box<dyn std::error::Error>>{
    let cfg = &get_config().swisstarget;

    let mut headers = HeaderMap::new();
    headers.insert("USER_AGENT", HeaderValue::from_str(&cfg.headers.user_agent)?);
    headers.insert("ORIGIN", HeaderValue::from_str(&cfg.headers.origin)?);
    headers.insert("REFERER", HeaderValue::from_str(&cfg.headers.referer)?);

    let client = reqwest::Client::builder().cookie_store(true).default_headers(headers).build()?;
    
    Ok(client)
}
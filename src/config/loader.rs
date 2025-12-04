use std::{fs, path::Path};

use crate::config::{models::AppConfig};

pub fn load_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "src/config/default.yaml".to_string());

    let content = fs::read_to_string(Path::new(&path))?;
    let config: AppConfig = serde_yaml::from_str(&content)?;

    Ok(config)
}
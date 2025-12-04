use std::{fs, path::{Path, PathBuf}};

use crate::config::{models::AppConfig};

pub fn load_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let path = get_config_path();
    // println!("📄 Menggunakan Config: {}", path.display());
    // let path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "src/config/default.yaml".to_string());

    let content = fs::read_to_string(Path::new(&path))?;
    let config: AppConfig = serde_yaml::from_str(&content)?;

    Ok(config)
}

pub fn get_config_path() -> PathBuf {
    let exe_dir = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    let prod_path = exe_dir.join("config").join("config.yaml");

    if prod_path.exists() {
        return prod_path;
    }

    let dev_path = PathBuf::from("src/config/default.yaml");
    if dev_path.exists() {
        return dev_path;
    }

    panic!("config.yaml tidak ditemukan di:\n  {}\n  {}", 
        prod_path.display(), 
        dev_path.display()
    );
}
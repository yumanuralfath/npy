pub mod models;
pub mod loader;

use std::sync::OnceLock;

use models::AppConfig;

static CONFIG: OnceLock<AppConfig> = OnceLock::new();

pub fn get_config() -> &'static AppConfig {
    CONFIG.get_or_init(|| {
        loader::load_config().expect("Failed to load config 𓁹‿𓁹")
    })
}
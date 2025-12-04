use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub swisstarget: SwissTargetConfig,
}

#[derive(Debug, Deserialize)]
pub  struct SwissTargetConfig {
    pub base_url: String,
    pub predict_endpoint: String,
    pub result_endpoint: String,

    pub headers: SwissHeaders,
    pub selector: SwissSelectors,
}

#[derive(Debug, Deserialize)]
pub struct SwissHeaders {
    pub user_agent: String,
    pub origin: String,
    pub referer: String,
}

#[derive(Debug, Deserialize)]
pub struct SwissSelectors {
    pub result_table: String,
    pub cell: String,
}
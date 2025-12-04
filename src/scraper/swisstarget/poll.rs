use std::time::Duration;

use indicatif::ProgressBar;
use reqwest::Client;

use crate::config::get_config;

pub async fn fetch_result_html(client: &Client, job_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let cfg = &get_config().swisstarget;

    let url = format!("{}{}?job={}&organism=Homo_sapiens",cfg.base_url, cfg.result_endpoint, job_id) ;

    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(120));
    bar.set_message("Waiting for server ... ✌︎㋡");

    loop {
        let res = client.get(&url).send().await?;
        let text =  res.text().await?;

        if text.contains("id=\"resultTable\"") {
            bar.finish_with_message("Data Ready ✌︎︎ alhamdulillah");
            return Ok(text);
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
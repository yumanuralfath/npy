use csv::Writer;
use scraper::{Html, Selector};

use crate::config::get_config;

pub fn parse_and_save_csv(
    html: &str,
    output: &str
)-> Result<(), Box<dyn std::error::Error>> {
    let cfg = &get_config().swisstarget.selector;

    let document = Html::parse_document(html);
    let row_sel = Selector::parse(&cfg.result_table)?;
    let cell_sel = Selector::parse(&cfg.cell)?;

    let mut wtr = Writer::from_path(output)?;

    wtr.write_record(&[
        "Target",
        "Common Name",
        "Uniprot ID",
        "ChEMBL ID",
        "Target Class",
        "Probability",
        "Known Actives",
    ])?;

    for row in document.select(&row_sel) {
        let mut row_data = Vec::new();

        for cell in row.select(&cell_sel) {
            let text = cell.text().collect::<Vec<_>>().join(" ");
            row_data.push(text.trim().to_string());
        }

        if !row_data.is_empty() {
            wtr.write_record(&row_data)?;
        }
    }

    wtr.flush()?;
    Ok(())
}
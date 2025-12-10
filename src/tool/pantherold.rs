use crate::tool::{verbose, write_debug};
use csv::ReaderBuilder;
use reqwest::Client;
use serde::{Deserialize, Deserializer};
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Deserialize)]
struct PantherResponse {
    results: Results,
}

#[derive(Debug, Deserialize)]
struct Results {
    #[serde(deserialize_with = "deserialize_result_vec")]
    result: Vec<EnrichmentResult>,

    input_list: InputList,
}

#[derive(Debug, Deserialize)]
struct InputList {
    mapped_count: u32,
}

#[derive(Debug, Deserialize)]
struct EnrichmentResult {
    number_in_list: u32,
    fdr: f64,
    term: Term,
}

#[derive(Debug, Deserialize)]
struct Term {
    id: Option<String>,
    label: String,
}

fn deserialize_result_vec<'de, D>(deserializer: D) -> Result<Vec<EnrichmentResult>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

    if value.is_array() {
        Ok(serde_json::from_value(value).unwrap())
    } else {
        Ok(vec![serde_json::from_value(value).unwrap()])
    }
}

pub fn read_csv(file: &str) -> Result<String, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().has_headers(false).from_path(file)?;

    let mut values: Vec<String> = Vec::new();

    for result in reader.records() {
        let record = result?;

        if let Some(value) = record.get(0) {
            let v = value.trim();
            if !v.is_empty() {
                values.push(v.to_string());
            }
        }
    }
    Ok(values.join(","))
}

pub async fn panther_protein_class_to_txt(
    genes: Option<Vec<String>>,
    organism: &str,
    output: &str,
    file: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let url = "http://pantherdb.org/services/oai/pantherdb/enrich/overrep";
    verbose(format!("Requesting: {}", url));

    let gene_input = if let Some(g) = genes.as_ref() {
        if !g.is_empty() {
            g.join(",")
        } else {
            match &file {
                Some(path) => read_csv(path)?,
                None => String::new(),
            }
        }
    } else {
        match &file {
            Some(path) => read_csv(path)?,
            None => String::new(),
        }
    };

    verbose(format!("geneInputList = {}", gene_input));

    let client = Client::new();
    let response = client
        .post(url)
        .query(&[
            ("geneInputList", gene_input.as_str()),
            ("organism", organism),
            ("annotDataSet", "ANNOT_TYPE_ID_PANTHER_PC"),
            ("enrichmentTestType", "FISHER"),
            ("correction", "FDR"),
        ])
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Request failed: {}", response.status()).into());
    }

    let text = response.text().await?;
    let json: PantherResponse = serde_json::from_str(&text)?;

    verbose(write_debug("Debug.json", &text)?);

    let total_mapped = json.results.input_list.mapped_count as f64;

    let mut valid_results: Vec<&EnrichmentResult> = json
        .results
        .result
        .iter()
        .filter(|r| r.number_in_list > 0 && r.fdr < 0.05)
        .collect();

    valid_results.sort_by(|a, b| a.fdr.partial_cmp(&b.fdr).unwrap());

    let mut out = String::new();
    let mut cumulative_pct = 0.0;

    for (i, item) in valid_results.iter().enumerate() {
        let count = item.number_in_list;
        let pct = (count as f64 / total_mapped) * 100.0;
        cumulative_pct += pct;

        let name_display = match &item.term.id {
            Some(id) => format!("{} ({})", item.term.label, id),
            None => item.term.label.clone(),
        };

        let line = format!(
            "{}\t{}\t{}\t{:.1}%\t{:.1}%\n",
            i + 1,          // Rank
            name_display,   // Name + ID
            count,          // Count
            pct,            // Percent
            cumulative_pct  // Cumulative
        );

        out.push_str(&line);
    }

    let mut file = File::create(output)?;
    file.write_all(out.as_bytes())?;

    verbose(format!("File saved to: {}", output));

    Ok(())
}

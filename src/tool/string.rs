use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Write;

use crate::tool::verbose;

#[derive(Debug, Deserialize, Serialize)]
pub struct MappedProtein {
    #[serde(rename = "queryItem")]
    pub query_item: Option<String>,
    #[serde(rename = "queryIndex")]
    pub query_index: u32,
    #[serde(rename = "stringId")]
    pub string_id: String,
    #[serde(rename = "ncbiTaxonId")]
    pub ncbi_taxon_id: u32,
    #[serde(rename = "taxonName")]
    pub taxon_name: String,
    #[serde(rename = "preferredName")]
    pub preferred_name: String,
    pub annotation: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NetworkInteraction {
    #[serde(rename = "stringId_A")]
    pub string_id_a: String,
    #[serde(rename = "stringId_B")]
    pub string_id_b: String,
    #[serde(rename = "preferredName_A")]
    pub preferred_name_a: String,
    #[serde(rename = "preferredName_B")]
    pub preferred_name_b: String,
    #[serde(rename = "ncbiTaxonId")]
    #[serde(deserialize_with = "deserialize_string_to_u32")]
    pub ncbi_taxon_id: u32,
    pub score: f64,
    pub nscore: Option<f64>,
    pub fscore: Option<f64>,
    pub pscore: Option<f64>,
    pub ascore: Option<f64>,
    pub escore: Option<f64>,
    pub dscore: Option<f64>,
    pub tscore: Option<f64>,
}

fn deserialize_string_to_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    let s = String::deserialize(deserializer)?;
    s.parse::<u32>().map_err(Error::custom)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub number_of_nodes: u32,
    pub number_of_edges: u32,
    pub average_node_degree: f64,
    pub local_clustering_coefficient: f64,
    pub expected_number_of_edges: Option<u32>,
    pub p_value: Option<f64>,
}

#[derive(Debug)]
pub struct StringResult {
    pub mapped_proteins: Vec<MappedProtein>,
    pub mapped_tsv: String,

    #[allow(dead_code)]
    pub interactions: Vec<NetworkInteraction>,

    pub tsv_data: String,
    pub png_data: Vec<u8>,
    pub network_stats: NetworkStats,
}

pub fn read_proteins_from_csv(filepath: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(filepath)?;

    let mut proteins = Vec::new();

    for result in reader.records() {
        let record = result?;
        if let Some(protein) = record.get(0) {
            let protein = protein.trim();
            if !protein.is_empty() {
                proteins.push(protein.to_string());
            }
        }
    }

    Ok(proteins)
}

pub async fn map_protein_identifiers(
    proteins: &[String],
    species: u32,
) -> Result<(Vec<MappedProtein>, String), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let base_url = "https://string-db.org/api";

    let identifiers = proteins.join("%0d");

    // Gunakan format TSV sesuai dokumentasi
    let url = format!("{}/tsv/get_string_ids", base_url);

    let params = [
        ("identifiers", identifiers.as_str()),
        ("species", &species.to_string()),
        ("limit", "1"),
        ("echo_query", "1"),
        ("caller_identity", "www.yumana.my.id"),
    ];

    let response = client.post(&url).form(&params).send().await?;
    let tsv_text = response.text().await?;

    let mut mapped_proteins = Vec::new();
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(tsv_text.as_bytes());

    for result in reader.deserialize() {
        let protein: MappedProtein = result?;
        mapped_proteins.push(protein);
    }

    Ok((mapped_proteins, tsv_text))
}

pub async fn query_string_database(
    proteins: &[String],
    species: u32,
) -> Result<StringResult, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let base_url = "https://string-db.org/api";

    println!("Mapping protein identifiers to STRING IDs...");
    let (mapped_proteins, mapped_tsv) = map_protein_identifiers(proteins, species).await?;

    if mapped_proteins.is_empty() {
        return Err("No proteins could be mapped to STRING identifiers".into());
    }

    println!("Successfully mapped {} proteins", mapped_proteins.len());

    let string_ids: Vec<String> = mapped_proteins
        .iter()
        .map(|p| p.string_id.clone())
        .collect();
    let identifiers = string_ids.join("%0d");

    let params = [
        ("identifiers", identifiers.as_str()),
        ("species", &species.to_string()),
        ("caller_identity", "www.yumana.my.id"),
    ];

    println!("Fetching network interactions (TSV)...");
    let tsv_url = format!("{}/tsv/network", base_url);
    let tsv_response = client.post(&tsv_url).form(&params).send().await?;
    let tsv_data = tsv_response.text().await?;

    // 2. Get network interactions (JSON untuk parsing)
    println!("Fetching network interactions (JSON)...");
    let json_url = format!("{}/json/network", base_url);
    let json_response = client.post(&json_url).form(&params).send().await?;
    let json_text = json_response.text().await?;

    let interactions: Vec<NetworkInteraction> =
        serde_json::from_str(&json_text).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    println!("Fetching high-resolution network image...");
    let image_params = [
        ("identifiers", identifiers.as_str()),
        ("species", &species.to_string()),
        ("network_flavor", "confidence"),
        ("caller_identity", "www.yumana.my.id"),
    ];
    let image_url = format!("{}/highres_image/network", base_url);
    let image_response = client.post(&image_url).form(&image_params).send().await?;
    let png_data = image_response.bytes().await?.to_vec();

    println!("Fetching PPI enrichment analysis...");
    let enrichment_url = format!("{}/json/ppi_enrichment", base_url);
    let enrichment_response = client.post(&enrichment_url).form(&params).send().await?;
    let enrichment_text = enrichment_response.text().await?;

    let network_stats = parse_ppi_enrichment(&enrichment_text)?;

    Ok(StringResult {
        mapped_proteins,
        mapped_tsv,
        interactions,
        tsv_data,
        png_data,
        network_stats,
    })
}

fn parse_ppi_enrichment(json_text: &str) -> Result<NetworkStats, Box<dyn Error>> {
    #[derive(Deserialize, Debug)]
    struct PpiEnrichment {
        number_of_nodes: u32,
        number_of_edges: u32,
        average_node_degree: f64,
        local_clustering_coefficient: f64,
        expected_number_of_edges: u32,
        p_value: f64,
    }

    let enrichment_array: Vec<PpiEnrichment> = serde_json::from_str(json_text)
        .map_err(|e| format!("Failed to parse PPI enrichment JSON: {}", e))?;

    let enrichment = enrichment_array
        .into_iter()
        .next()
        .ok_or("Empty PPI enrichment array")?;

    println!("✓ PPI enrichment parsed successfully");

    Ok(NetworkStats {
        number_of_nodes: enrichment.number_of_nodes,
        number_of_edges: enrichment.number_of_edges,
        average_node_degree: enrichment.average_node_degree,
        local_clustering_coefficient: enrichment.local_clustering_coefficient,
        expected_number_of_edges: Some(enrichment.expected_number_of_edges),
        p_value: Some(enrichment.p_value),
    })
}

pub async fn process_csv_and_query_string(
    csv_path: &str,
    species: u32,
    output_dir: &str,
) -> Result<NetworkStats, Box<dyn Error>> {
    std::fs::create_dir_all(output_dir)?;

    let proteins = read_proteins_from_csv(csv_path)?;

    if proteins.is_empty() {
        return Err("Tidak ada protein yang ditemukan di CSV".into());
    }

    println!("Ditemukan {} proteins dari CSV", proteins.len());

    let result = query_string_database(&proteins, species).await?;

    let output_mapped_tsv = format!("{}/mapped_proteins.tsv", output_dir);
    let output_interactions_tsv = format!("{}/network_interactions.tsv", output_dir);
    let output_png = format!("{}/network_image_highres.png", output_dir);
    let output_analysis_json = format!("{}/network_analysis.json", output_dir);
    let output_mapped_json = format!("{}/mapped_proteins.json", output_dir);

    let mut mapped_tsv_file = File::create(&output_mapped_tsv)?;
    mapped_tsv_file.write_all(result.mapped_tsv.as_bytes())?;
    println!("✓ Mapped proteins TSV disimpan ke: {}", output_mapped_tsv);

    let mut tsv_file = File::create(&output_interactions_tsv)?;
    tsv_file.write_all(result.tsv_data.as_bytes())?;
    println!(
        "✓ Network interactions TSV disimpan ke: {}",
        output_interactions_tsv
    );

    let mut png_file = File::create(&output_png)?;
    png_file.write_all(&result.png_data)?;
    println!("✓ High-res PNG disimpan ke: {}", output_png);

    let mapped_json = serde_json::to_string_pretty(&result.mapped_proteins)?;
    let mut mapped_file = File::create(&output_mapped_json)?;
    mapped_file.write_all(mapped_json.as_bytes())?;
    println!("✓ Mapped proteins JSON disimpan ke: {}", output_mapped_json);

    let analysis_json = serde_json::to_string_pretty(&result.network_stats)?;
    let mut analysis_file = File::create(&output_analysis_json)?;
    analysis_file.write_all(analysis_json.as_bytes())?;
    println!(
        "✓ Network analysis JSON disimpan ke: {}",
        output_analysis_json
    );

    let stats_content = format!(
        "\n=== Network Analysis (PPI Enrichment) ===\n\
        Number of nodes: {}\n\
        Number of edges: {}\n\
        Average node degree: {:.2}\n\
        Local clustering coefficient: {:.4}\n\
        Expected number of edges: {}\n\
        P-value: {}\n",
        result.network_stats.number_of_nodes,
        result.network_stats.number_of_edges,
        result.network_stats.average_node_degree,
        result.network_stats.local_clustering_coefficient,
        result
            .network_stats
            .expected_number_of_edges
            .map(|v| v.to_string())
            .unwrap_or_else(|| "N/A".to_string()),
        result
            .network_stats
            .p_value
            .map(|v| format!("{:.6e}", v))
            .unwrap_or_else(|| "N/A".to_string()),
    );

    verbose(stats_content);
    Ok(result.network_stats)
}

pub async fn handle_string_command(
    csv: &str,
    species: u32,
    output: &str,
) -> Result<(), Box<dyn Error>> {
    println!("Starting STRING database analysis...");
    verbose(format!("CSV file: {}", csv));
    verbose(format!("Species: {}", species));
    verbose(format!("Output directory: {}", output));

    let stats = process_csv_and_query_string(csv, species, output).await?;
    verbose(stats);

    println!("\nAnalysis completed successfully!");

    Ok(())
}

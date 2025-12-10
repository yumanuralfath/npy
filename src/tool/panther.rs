use regex::Regex;
use reqwest::header::{CONTENT_TYPE, ORIGIN, REFERER};
use reqwest::multipart;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::time::Duration;

use crate::tool::verbose;

fn build_client_with_jar()
-> Result<(reqwest::Client, std::sync::Arc<reqwest::cookie::Jar>), Box<dyn Error>> {
    use reqwest::cookie::Jar;
    use std::sync::Arc;

    let jar = Arc::new(Jar::default());

    let client = reqwest::Client::builder()
        .cookie_provider(jar.clone())
        .timeout(Duration::from_secs(60))
        .user_agent(
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 \
             (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36",
        )
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()?;

    Ok((client, jar))
}

fn extract_tracking_id(html: &str) -> Option<String> {
    let re = Regex::new(r"trackingId=([A-F0-9]+)").ok()?;
    re.captures(html)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
}

pub async fn panther_analysis_export(
    csv_path: &str,
    organism: &str,
    dataset: &str,
    chart_type: u8,
) -> Result<String, Box<dyn Error>> {
    println!("[INFO] Memulai analisis PANTHER");
    verbose(format!("[INFO] File: {}", csv_path));
    verbose(format!("[INFO] Organism: {}", organism));
    verbose(format!("[INFO] Dataset: {}", dataset));

    let (client, _jar) = build_client_with_jar()?;

    verbose("[1/6] Mengakses homepage...");
    let _ = client
        .get("https://pantherdb.org/")
        .send()
        .await?
        .text()
        .await?;

    tokio::time::sleep(Duration::from_millis(800)).await;

    verbose("[2/6] Mengirim validasi manusia...");
    let res_validate = client
        .post("https://pantherdb.org/servlet/ValidateHuman")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header(REFERER, "https://pantherdb.org/validateHuman.jsp")
        .header(ORIGIN, "https://pantherdb.org")
        .body("humanCheck=yes")
        .send()
        .await?;

    let status = res_validate.status();
    if !status.is_success() {
        return Err(format!("Validasi gagal dengan status: {}", status).into());
    }

    verbose("[2/6] ✓ Validasi berhasil");
    tokio::time::sleep(Duration::from_millis(800)).await;

    verbose("[3/6] Membaca file CSV...");
    let mut file = File::open(csv_path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let content = String::from_utf8_lossy(&buf);
    let gene_count = content.lines().filter(|l| !l.trim().is_empty()).count();
    verbose(format!("[3/6] ✓ Ditemukan {} gen", gene_count));

    verbose("[4/6] Mengunggah data untuk analisis...");
    let form = multipart::Form::new()
        .text("idField", "")
        .part(
            "fileData",
            multipart::Part::bytes(buf)
                .file_name("unique_genes.csv")
                .mime_str("text/csv")?,
        )
        .text("fileType", "10")
        .text("mapOrg", "Abrus_precatorius_ABRPR")
        .text("organism", organism.to_string())
        .text("dataset", dataset.to_string())
        .text("resultType", "2")
        .text("chartType", chart_type.to_string());

    let res_analysis = client
        .post("https://pantherdb.org/geneListAnalysis.do")
        .header(REFERER, "https://pantherdb.org/")
        .header(ORIGIN, "https://pantherdb.org")
        .multipart(form)
        .send()
        .await?;

    let analysis_html = res_analysis.text().await?;

    if analysis_html.contains("Validating User") {
        return Err("Session hilang saat analisis".into());
    }

    let tracking_id =
        extract_tracking_id(&analysis_html).ok_or("Gagal mendapatkan tracking ID dari response")?;

    verbose(format!(
        "[4/6] ✓ Analisis selesai (Tracking ID: {})",
        tracking_id
    ));
    tokio::time::sleep(Duration::from_millis(500)).await;

    verbose("[5/6] Mengakses halaman chart...");
    let chart_url = format!(
        "https://pantherdb.org/chart/pantherChart.jsp?listType=1&filterLevel=1&type=5&chartType={}&save=yes&basketItems=all&zoom=1&trackingId={}",
        chart_type, tracking_id
    );

    let res_chart = client
        .get(&chart_url)
        .header(REFERER, "https://pantherdb.org/geneListAnalysis.do")
        .send()
        .await?;

    let chart_status = res_chart.status();
    if !chart_status.is_success() {
        return Err(format!("Gagal akses chart: {}", chart_status).into());
    }

    verbose("[5/6] ✓ Chart berhasil diakses");
    tokio::time::sleep(Duration::from_millis(500)).await;

    verbose("[6/6] Mengunduh data export...");
    let export_url = "https://pantherdb.org/chart/pantherChartExport.jsp";

    let res_export = client
        .get(export_url)
        .header(REFERER, &chart_url)
        .send()
        .await?;

    let export_status = res_export.status();
    if !export_status.is_success() {
        return Err(format!("Gagal export data: {}", export_status).into());
    }

    let export_data = res_export.text().await?;

    if export_data.is_empty() {
        return Err("Export data kosong".into());
    }

    verbose(format!(
        "[6/6] ✓ Data berhasil diunduh ({} bytes)",
        export_data.len(),
    ));

    Ok(export_data)
}

pub async fn run_panther_analysis(csv_path: &str, output: &str) -> Result<String, Box<dyn Error>> {
    run_panther_analysis_custom(
        csv_path,
        "Homo sapiens",
        "Homo sapiens", // Dataset for protein class
        2,              // Chart type 2 = Protein Class
        output,
    )
    .await
}

pub async fn run_panther_analysis_custom(
    csv_path: &str,
    organism: &str,
    dataset: &str,
    chart_type: u8,
    output: &str,
) -> Result<String, Box<dyn Error>> {
    let result = panther_analysis_export(csv_path, organism, dataset, chart_type).await?;

    let output_path = output;
    std::fs::write(output_path, &result)?;
    println!("\n[SUCCESS] Hasil disimpan ke: {}", output_path);

    verbose("\n--- Preview Data ---");
    for line in result.lines().take(10) {
        verbose(line);
    }
    if result.lines().count() > 10 {
        verbose(format!("... ({} baris total)", result.lines().count()));
    }

    Ok(result)
}

// pub async fn run_panther_analysis_with_retry(
//     csv_path: &str,
//     max_retries: u32,
// ) -> Result<String, Box<dyn Error>> {
//     for attempt in 1..=max_retries {
//         verbose(
//             "\n{:=^60}",
//             format!(" Percobaan {}/{} ", attempt, max_retries)
//         );
//
//         match run_panther_analysis(csv_path).await {
//             Ok(result) => {
//                 verbose("\n🎉 Analisis berhasil pada percobaan ke-{}", attempt);
//                 return Ok(result);
//             }
//             Err(e) if attempt == max_retries => {
//                 verbose("\n❌ Semua percobaan gagal");
//                 return Err(e);
//             }
//             Err(e) => {
//                 verbose("⚠️  Percobaan {} gagal: {}", attempt, e);
//                 verbose("Menunggu 3 detik sebelum mencoba lagi...");
//                 tokio::time::sleep(Duration::from_secs(3)).await;
//             }
//         }
//     }
//
//     Err("Tidak ada percobaan yang berhasil".into())
// }

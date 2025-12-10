use std::fs::{self, File};
use std::io::Write;

pub fn init_default_files() -> std::io::Result<()> {
    // Buat folder
    fs::create_dir_all("output/smiles")?;
    fs::create_dir_all("output/Genecards")?;

    // File default
    let smiles_path = "output/smiles/smiles.csv";
    let genecard_path = "output/Genecards/Genecards.csv";

    // Template kosong atau header default
    let smiles_content = "SMILES\n";
    let genecard_content = "Gene\n";

    // Buat file SMILES
    let mut f1 = File::create(smiles_path)?;
    f1.write_all(smiles_content.as_bytes())?;

    // Buat file Genecards
    let mut f2 = File::create(genecard_path)?;
    f2.write_all(genecard_content.as_bytes())?;

    println!("Created:");
    println!("  {}", smiles_path);
    println!("  {}", genecard_path);

    Ok(())
}

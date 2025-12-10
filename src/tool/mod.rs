use std::{fs::File, io::Write};

use clap::Parser;

use crate::cli::args::Cli;

pub mod data;
pub mod panther;
pub mod pantherold;
pub mod string;
pub mod venny;

pub fn verbose<T: std::fmt::Debug>(value: T) {
    if Cli::parse().verbose {
        println!("[VERBOSE] {:?}", value);
    }
}

pub fn write_debug(path: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    println!("[VERBOSE]  File save {path}");
    Ok(())
}

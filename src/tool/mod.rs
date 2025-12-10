use clap::Parser;
use std::{fs::OpenOptions, io::Write};

use crate::cli::args::Cli;

pub mod data;
pub mod init;
pub mod panther;
pub mod pantherold;
pub mod repl;
pub mod runall;
pub mod string;
pub mod venny;

fn write_verbose_log(msg: &str) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("verbose.log")
    {
        let _ = writeln!(file, "{}", msg);
    }
}

pub fn verbose<T: std::fmt::Debug>(value: T) {
    let msg = format!("[VERBOSE] {:?}", value);

    if Cli::parse().verbose {
        println!("{}", msg);
    }

    write_verbose_log(&msg);
}

pub fn verbose_with_name<T: std::fmt::Debug>(name: &str, value: T) {
    let msg = format!("[VERBOSE] {}: {:?}", name, value);

    if Cli::parse().verbose {
        println!("{}", msg);
    }

    write_verbose_log(&msg);
}

pub fn write_debug(path: &str, content: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    file.write_all(content.as_bytes())?;
    let msg = format!("[DEBUG FILE] Saved {path}");

    if Cli::parse().verbose {
        println!("{}", msg);
    }

    write_verbose_log(&msg);

    Ok(())
}

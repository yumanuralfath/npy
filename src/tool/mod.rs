use clap::Parser;

use crate::cli::args::Cli;

pub mod data;
pub mod panther;

pub fn verbose<T: std::fmt::Debug>(value: T) {
    if Cli::parse().verbose {
        println!("{:?}", value)
    }
}

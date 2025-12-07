mod cli;
mod config;
mod scraper;
mod tool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::run()
}

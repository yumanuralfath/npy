mod cli;
mod config;
mod scraper;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::run()
}

use crate::cli::{Cli, Commands};
use crate::infrastructure::client::Client;
use crate::infrastructure::config::Config;
use crate::infrastructure::templates::Templates;
use anyhow::Result;
use clap::Parser;
use crate::infrastructure::request_cache::RequestCache;

mod api;
mod cli;
mod infrastructure;
mod export;

fn main() -> Result<()> {
    dotenv::from_filename(".env.local")?;
    let config = Config::from_dotenv()?;
    let client = Client::new(&config)?;
    let templates = Templates::new()?;
    let cache = RequestCache::new(&config);

    let args = Cli::parse();
    match args.command {
        Commands::Merkzettel(args) => args.run(&client, &templates, &cache)
    }
}
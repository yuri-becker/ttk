pub mod merkzettel;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "tkk – thalia.de toolkit",
    about = "Tools for Thalia.de stuff",
    arg_required_else_help = true,
    subcommand_required = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Merkzettel(merkzettel::Args),
}

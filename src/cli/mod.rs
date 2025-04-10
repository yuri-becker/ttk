pub mod merkzettel;

use clap::Command;

pub fn cli() -> Command {
    Command::new("thalia.de toolset")
        .about("Tools for Thalia.de stuff")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("merkzettel").about("Exports the Merkzettel into a readable format"),
        )
}

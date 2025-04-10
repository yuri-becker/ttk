use crate::cli::{cli, merkzettel};
use crate::infrastructure::client::Client;
use crate::infrastructure::config::Config;

mod api;
mod cli;
mod infrastructure;

fn main() {
    dotenv::from_filename(".env.local").unwrap();
    let config = Config::from_dotenv();
    let client = Client::new(&config);

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("merkzettel", _)) => merkzettel::run(&client),
        _ => unreachable!(),
    }
    .unwrap();
}

use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "budge - a simple CLI to manage your budget in a TSV")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Reverses a string
    Add(Add),
}

#[derive(Args)]
struct Add {
    /// The string to reverse
    string: Option<String>,
}

use std::env;
extern crate dirs;

fn main() {
    for argument in env::args() {
        println!("{argument}");
    }

    let data_dir = dirs::data_dir().expect("could not get user's data directory");
    println!("{}", data_dir.display());
}

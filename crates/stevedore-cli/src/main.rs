//! `stevedore` — command-line mover for secrets between stores.

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "stevedore",
    version,
    about = "Move secrets between password managers and vaults"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List the stores stevedore knows about.
    Stores,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Stores => {
            println!("sources: {}", stevedore::dashlane::NAME);
            println!("sinks:   (none yet)");
        }
    }
    Ok(())
}

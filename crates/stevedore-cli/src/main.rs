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
        // Moving secrets needs a source and a sink. Dashlane is modelled; the
        // second store isn't, so there is no route to offer yet and no `migrate`
        // command that could honestly succeed.
        Command::Stores => {
            println!("sources: {}", stevedore::dashlane::NAME);
            println!("sinks:   (none yet)");
        }
    }
    Ok(())
}

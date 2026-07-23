//! `stevedore` — command-line mover for secrets between stores.

use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Parser, Subcommand, ValueEnum};

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
    /// List the source and sink stores stevedore knows about.
    Stores,
    /// Move secrets from a source store to a sink store.
    ///
    /// The default is a dry-run: stevedore reads the source and reports what
    /// would move without writing anything. Pass `--apply` to actually write.
    Migrate(MigrateArgs),
}

#[derive(Args)]
struct MigrateArgs {
    /// Store to read secrets from.
    #[arg(long)]
    from: Store,
    /// Store to write secrets to.
    #[arg(long)]
    to: Store,
    /// Path to the source export (e.g. a Dashlane export).
    #[arg(long)]
    input: Option<PathBuf>,
    /// Actually write to the sink. Without this, stevedore only plans (dry-run).
    #[arg(long)]
    apply: bool,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Store {
    Dashlane,
    Proton,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Stores => {
            println!("sources: {}", stevedore::dashlane::NAME);
            println!("sinks:   {}", stevedore::proton::NAME);
            println!(
                "routes:  {} -> {} (in progress, ADR-0003)",
                stevedore::dashlane::NAME,
                stevedore::proton::NAME
            );
        }
        Command::Migrate(MigrateArgs {
            from,
            to,
            input,
            apply,
        }) => {
            // The migration engine is still under construction (ADR-0003). For
            // now the CLI wires the arguments through and reports honestly
            // rather than pretending to move anything.
            let where_from = input
                .map(|p| format!(" from {}", p.display()))
                .unwrap_or_default();
            anyhow::bail!(
                "migrate {from:?} -> {to:?}{where_from} (apply={apply}) is not \
                 implemented yet; see ADR-0003"
            );
        }
    }
    Ok(())
}

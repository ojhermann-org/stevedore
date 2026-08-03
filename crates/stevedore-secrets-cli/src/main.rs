//! `stevedore` — command-line mover for secrets between stores.

// A CLI's results are its product, and the product goes to stdout. This covers
// the product only — diagnostics still belong on stderr.
#![expect(clippy::print_stdout, reason = "the CLI's results are its stdout")]

use std::fmt;

use anyhow::{Result, bail};
use clap::{Parser, Subcommand};
use stevedore_secrets::{
    dashlane,
    mover::{self, Plan},
    proton,
};

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

    /// Move secrets from Dashlane into a Proton Pass vault.
    ///
    /// Reports what would change and writes nothing, unless given `--apply`.
    Move {
        /// Name of the Proton Pass vault to write into.
        #[arg(long, value_name = "NAME")]
        to_vault: String,

        /// Create the items. Without this, nothing is written.
        #[arg(long)]
        apply: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Stores => {
            println!("sources: {}", dashlane::NAME);
            println!("sinks:   {}", proton::NAME);
        }
        Command::Move { to_vault, apply } => run_move(&to_vault, apply)?,
    }
    Ok(())
}

fn run_move(vault_name: &str, apply: bool) -> Result<()> {
    let vault = proton::vault(vault_name)?;
    let plan = mover::plan(&vault)?;

    print!("{}", Summary { plan: &plan, apply });

    if !apply {
        if plan.is_empty() {
            println!("\n  nothing to do");
        } else {
            println!("\n  nothing written — re-run with --apply");
        }
        return Ok(());
    }

    let report = mover::apply(&vault, plan)?;
    println!("\n  created {}", report.created);
    if !report.failures.is_empty() {
        println!("  failed  {}", report.failures.len());
        for failure in &report.failures {
            println!("    - {}: {}", failure.title, failure.error);
        }
        bail!("{} item(s) could not be created", report.failures.len());
    }

    Ok(())
}

/// What a plan would do, rendered for the terminal. `apply` picks the tense.
struct Summary<'a> {
    plan: &'a Plan,
    apply: bool,
}

impl fmt::Display for Summary<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let plan = self.plan;
        let (create, skip) = if self.apply {
            ("create", "skip")
        } else {
            ("would create", "would skip")
        };

        writeln!(f, "  {create}  {} logins", plan.logins())?;
        writeln!(f, "  {create}  {} notes", plan.notes())?;
        writeln!(f, "  {skip}  {} (already present)", plan.skipped.len())?;

        if !plan.unclassified.is_empty() {
            writeln!(
                f,
                "  unclassified  {} (state unknown)",
                plan.unclassified.len()
            )?;
        }
        if !plan.notes_dropped.is_empty() {
            writeln!(
                f,
                "  note dropped  {} logins carry note text",
                plan.notes_dropped.len()
            )?;
        }

        if !plan.unclassified.is_empty() {
            writeln!(
                f,
                "\n  unclassified — the vault holds an item stevedore cannot read the state of:"
            )?;
            for existing in &plan.unclassified {
                writeln!(f, "    - {}", existing.title)?;
            }
        }

        if !plan.notes_dropped.is_empty() {
            writeln!(
                f,
                "\n  note text stays behind — Proton's login template has no field for it:"
            )?;
            for title in &plan.notes_dropped {
                writeln!(f, "    - {title}")?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use stevedore_secrets::{mover::Existing, proton::Kind};

    use super::*;

    fn summary(plan: &Plan, apply: bool) -> String {
        Summary { plan, apply }.to_string()
    }

    #[test]
    fn a_dry_run_says_what_it_would_do() {
        let plan = Plan::default();
        let out = summary(&plan, false);
        assert!(out.contains("would create  0 logins"));
        assert!(out.contains("would skip  0 (already present)"));
    }

    #[test]
    fn an_applied_run_speaks_in_the_past() {
        let out = summary(&Plan::default(), true);
        assert!(out.contains("create  0 logins"));
        assert!(!out.contains("would"));
    }

    /// Both are silent when empty: a clean move should not report on the two
    /// things it could not settle.
    #[test]
    fn the_two_caveats_appear_only_when_they_apply() {
        let out = summary(&Plan::default(), false);
        assert!(!out.contains("unclassified"));
        assert!(!out.contains("note dropped"));
    }

    #[test]
    fn the_caveats_are_counted_and_then_named() {
        let mut plan = Plan::default();
        plan.unclassified.push(Existing {
            title: "Odd one".to_owned(),
            kind: Kind::Login,
        });
        plan.notes_dropped.push("GitHub".to_owned());

        let out = summary(&plan, false);
        assert!(out.contains("unclassified  1 (state unknown)"));
        assert!(out.contains("note dropped  1 logins carry note text"));
        assert!(out.contains("    - Odd one"));
        assert!(out.contains("    - GitHub"));
    }
}

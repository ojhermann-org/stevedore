//! Driving the `dcli` binary.

use std::io::ErrorKind;
use std::process::{Command, Stdio};

use serde::de::DeserializeOwned;

use crate::error::{Error, Result};

/// The external CLI stevedore drives. It must already be registered and
/// unlocked — stevedore never authenticates. See `docs/dcli/`.
pub(super) const DCLI: &str = "dcli";

/// What `dcli status` reports.
#[derive(Debug, Clone)]
pub struct Status {
    pub logged_in: bool,
    pub locked: bool,
    /// The account email, when logged in.
    pub login: Option<String>,
}

/// Ask `dcli` whether it can serve a read.
///
/// # Errors
///
/// [`Error::CliMissing`] when `dcli` isn't installed, or [`Error::CliFailed`].
pub fn status() -> Result<Status> {
    let out = run(&["status"])?;
    let mut status = Status {
        logged_in: false,
        locked: false,
        login: None,
    };
    for line in out.lines() {
        match line.split_once(':').map(|(k, v)| (k.trim(), v.trim())) {
            Some(("Logged in", v)) => status.logged_in = v == "yes",
            Some(("Locked", v)) => status.locked = v == "yes",
            Some(("Login", v)) => status.login = Some(v.to_owned()),
            _ => {}
        }
    }
    Ok(status)
}

/// Pull the freshest vault data from Dashlane.
///
/// Never called automatically. `dcli` already syncs hourly on its own, and a
/// network round trip against someone's password manager is theirs to ask for.
///
/// # Errors
///
/// [`Error::CliNotLoggedIn`] if the vault isn't ready, or [`Error::CliFailed`].
pub fn sync() -> Result<()> {
    ready()?;
    run(&["sync"]).map(|_| ())
}

/// Read every login in the vault.
///
/// # Errors
///
/// [`Error::CliNotLoggedIn`], [`Error::CliLocked`], [`Error::CliFailed`], or
/// [`Error::CliOutput`] if the response isn't the expected shape.
pub fn logins() -> Result<Vec<super::Login>> {
    list(&["password", "-o", "json"], "logins")
}

/// Read every secure note in the vault.
///
/// # Errors
///
/// As [`logins`].
pub fn notes() -> Result<Vec<super::Note>> {
    list(&["note", "-o", "json"], "notes")
}

fn list<T: DeserializeOwned>(args: &[&str], field: &'static str) -> Result<Vec<T>> {
    ready()?;
    let out = run(args)?;
    serde_json::from_str(&out).map_err(|_| Error::CliOutput {
        program: DCLI,
        field,
    })
}

/// Refuse to run anything against a vault that can't answer.
///
/// This guard is not politeness. `dcli` treats an unauthenticated invocation as
/// a cue to *start registration* — it prompts for an email, a 2FA code and the
/// Master Password. A child process would either block forever on that prompt
/// or fail in a way that looks nothing like "you aren't logged in".
fn ready() -> Result<()> {
    let status = status()?;
    if !status.logged_in {
        return Err(Error::CliNotLoggedIn { program: DCLI });
    }
    if status.locked {
        return Err(Error::CliLocked { program: DCLI });
    }
    Ok(())
}

fn run(args: &[&str]) -> Result<String> {
    let output = Command::new(DCLI)
        .args(args)
        // Closing stdin turns a would-be interactive prompt into a clean
        // failure instead of a process that hangs until someone kills it.
        .stdin(Stdio::null())
        .output()
        .map_err(|e| match e.kind() {
            ErrorKind::NotFound => Error::CliMissing { program: DCLI },
            _ => Error::Io(e),
        })?;

    if !output.status.success() {
        return Err(Error::CliFailed {
            program: DCLI,
            args: args.join(" "),
            status: output.status.to_string(),
            // stderr carries diagnostics, not vault contents — but it is
            // ANSI-coloured, which would otherwise land escape codes in logs.
            stderr: strip_ansi(&String::from_utf8_lossy(&output.stderr))
                .trim()
                .to_owned(),
        });
    }

    // stdout is a plaintext dump of the vault. It is never logged, never
    // attached to an error, and never written to disk.
    String::from_utf8(output.stdout).map_err(|_| Error::CliOutput {
        program: DCLI,
        field: "output",
    })
}

/// Remove ANSI escape sequences, which `dcli` writes into its error messages.
fn strip_ansi(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c != '\u{1b}' {
            out.push(c);
            continue;
        }
        // Skip up to the terminating byte of the escape sequence.
        for c in chars.by_ref() {
            if c.is_ascii_alphabetic() {
                break;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_ansi_colour_codes() {
        let coloured = "\u{1b}[31merror: No matching item found\u{1b}[0m";
        assert_eq!(strip_ansi(coloured), "error: No matching item found");
    }

    #[test]
    fn leaves_plain_text_alone() {
        assert_eq!(strip_ansi("no escapes here"), "no escapes here");
    }
}

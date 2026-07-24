//! Driving the `dcli` binary.

use std::io::ErrorKind;
use std::process::{Command, Stdio};

use serde::de::DeserializeOwned;

use crate::error::{CliError, Error, Result};

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
/// [`CliError::NotFound`] when `dcli` isn't installed, or [`CliError::Failed`].
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

/// Pull the freshest vault data from Dashlane. Call it only on request.
///
/// # Errors
///
/// [`Error::NotAuthenticated`] if the vault isn't ready, or [`CliError::Failed`].
pub fn sync() -> Result<()> {
    ready()?;
    run(&["sync"]).map(|_| ())
}

/// Read every login in the vault.
///
/// # Errors
///
/// [`Error::NotAuthenticated`], [`Error::Locked`], [`CliError::Failed`], or
/// [`Error::Unparsable`] if the response isn't the expected shape.
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
    serde_json::from_str(&out).map_err(|_| Error::Unparsable { what: field })
}

/// Refuse to run against a vault that can't answer.
///
/// An unauthenticated `dcli` starts registration and prompts for credentials,
/// which would hang a child process rather than fail.
fn ready() -> Result<()> {
    let status = status()?;
    if !status.logged_in {
        return Err(Error::NotAuthenticated);
    }
    if status.locked {
        return Err(Error::Locked);
    }
    Ok(())
}

fn run(args: &[&str]) -> Result<String> {
    let output = Command::new(DCLI)
        .args(args)
        // Closed stdin makes dcli's auth prompt fail cleanly instead of hanging.
        .stdin(Stdio::null())
        .output()
        .map_err(|e| match e.kind() {
            ErrorKind::NotFound => CliError::NotFound { program: DCLI }.into(),
            _ => Error::Io(e),
        })?;

    if !output.status.success() {
        return Err(CliError::Failed {
            program: DCLI,
            args: args.join(" "),
            status: output.status.to_string(),
            stderr: strip_ansi(&String::from_utf8_lossy(&output.stderr))
                .trim()
                .to_owned(),
        }
        .into());
    }

    // stdout is a plaintext vault dump: never log it or attach it to an error.
    String::from_utf8(output.stdout).map_err(|_| Error::Unparsable { what: "output" })
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
        // An ANSI escape ends at its first letter.
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

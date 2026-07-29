//! Driving the `dcli` binary.

use serde::de::DeserializeOwned;

use crate::cli;
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
/// [`CliError::NotFound`] when `dcli` isn't installed, or [`CliError::Failed`].
pub fn status() -> Result<Status> {
    let out = run(&["status"])?;
    let text = String::from_utf8_lossy(&out);
    let mut status = Status {
        logged_in: false,
        locked: false,
        login: None,
    };
    for line in text.lines() {
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
    crate::error::from_json(&out, field)
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

fn run(args: &[&str]) -> Result<Vec<u8>> {
    cli::run(DCLI, args)
}

//! Driving the `pass-cli` binary.

use serde::{Deserialize, Serialize};

use crate::cli;
use crate::error::{CliError, Error, Result};

use super::item::{NewLogin, NewNote};
use super::vault::{Vault, VaultList};

/// The external CLI stevedore drives. It must already be logged in — stevedore
/// never authenticates. See `docs/pass-cli/`.
pub(super) const PASS_CLI: &str = "pass-cli";

/// What `pass-cli info` reports about the logged-in account.
///
/// `pass-cli` also reports opaque account identifiers, which stevedore has no
/// use for and does not model.
#[derive(Debug, Clone, Deserialize)]
pub struct Session {
    /// The account email.
    pub email: Option<String>,

    /// The `pass-cli` update channel, e.g. `"stable"`.
    pub release_track: Option<String>,
}

/// Describe the account stevedore would write through.
///
/// # Errors
///
/// [`Error::NotAuthenticated`] if there is no usable session, [`CliError::NotFound`]
/// when `pass-cli` isn't installed, [`CliError::Failed`], or [`Error::Unparsable`].
pub fn session() -> Result<Session> {
    ready()?;
    let out = cli::run(PASS_CLI, &["info", "-o", "json"])?;
    crate::error::from_json(&out, "session")
}

/// Read every vault in the account. Vaults hold no secret values.
///
/// # Errors
///
/// As [`session`].
pub fn vaults() -> Result<Vec<Vault>> {
    ready()?;
    let out = cli::run(PASS_CLI, &["vault", "list", "--output", "json"])?;
    let list: VaultList = crate::error::from_json(&out, "vaults")?;
    Ok(list.vaults)
}

/// Find the vault with this name.
///
/// # Errors
///
/// [`Error::NoSuchVault`] if the account has no vault by that name, otherwise as
/// [`vaults`].
pub fn vault(name: &str) -> Result<Vault> {
    vaults()?
        .into_iter()
        .find(|vault| vault.name == name)
        .ok_or_else(|| Error::NoSuchVault {
            name: name.to_owned(),
        })
}

/// Create a login in `vault`.
///
/// # Errors
///
/// As [`session`], plus [`Error::Unserializable`].
pub fn create_login(vault: &Vault, login: &NewLogin) -> Result<()> {
    create(vault, "login", login)
}

/// Create a secure note in `vault`.
///
/// # Errors
///
/// As [`create_login`].
pub fn create_note(vault: &Vault, note: &NewNote) -> Result<()> {
    create(vault, "note", note)
}

fn create<T: Serialize>(vault: &Vault, kind: &str, item: &T) -> Result<()> {
    ready()?;
    let payload = serde_json::to_vec(item).map_err(|_| Error::Unserializable)?;
    let args = [
        "item",
        "create",
        kind,
        "--share-id",
        vault.share_id.as_str(),
        // The item, secrets included, travels on stdin.
        "--from-template",
        "-",
    ];
    cli::run_with_stdin(PASS_CLI, &args, &payload).map(|_| ())
}

/// Refuse to run against an account that can't answer.
///
/// `pass-cli test` is the tool's own check that an authenticated connection can
/// be established; if it can't, stevedore has no session to work through.
fn ready() -> Result<()> {
    match cli::run(PASS_CLI, &["test"]) {
        Ok(_) => Ok(()),
        Err(Error::Cli(CliError::Failed { .. })) => Err(Error::NotAuthenticated),
        Err(other) => Err(other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_session() {
        let json = br#"{
            "release_track": "stable",
            "id": "AAAA==",
            "username": "opaque",
            "email": "otto@example.test"
        }"#;
        let session: Session = crate::error::from_json(json, "session").unwrap();
        assert_eq!(session.email.as_deref(), Some("otto@example.test"));
        assert_eq!(session.release_track.as_deref(), Some("stable"));
    }

    #[test]
    fn a_session_without_an_email_parses() {
        let session: Session = crate::error::from_json(b"{}", "session").unwrap();
        assert!(session.email.is_none());
    }
}

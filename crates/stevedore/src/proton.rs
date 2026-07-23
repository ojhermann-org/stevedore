//! Sink: write secret records into a Proton Pass vault.
//!
//! Proton Pass has no public write API yet, so the path in (an import format it
//! accepts, vs. an authenticated session) is an open decision — see ADR-0003.
//! Until it's settled this reports the work as unimplemented.

use crate::error::{Error, Result};
use crate::secret::SecretRecord;

/// The store name, used in errors and the CLI's `stores` listing.
pub const NAME: &str = "proton";

/// Write `records` into the named Proton Pass `vault`, returning how many were
/// written.
///
/// # Errors
///
/// Currently always returns [`Error::SinkUnsupported`] — the Proton Pass writer
/// is not implemented yet.
pub fn write_vault(_vault: &str, _records: &[SecretRecord]) -> Result<usize> {
    Err(Error::SinkUnsupported { store: NAME })
}

//! Source: read secret records from a Dashlane export.
//!
//! Dashlane exports its vault from the desktop / web app (Settings → Export) as
//! a set of CSV files or an encrypted archive. The parser that turns that into
//! [`SecretRecord`]s is the first real milestone — see ADR-0003. Until it lands
//! this reports the work as unimplemented rather than pretending to succeed.

use std::path::Path;

use crate::error::{Error, Result};
use crate::secret::SecretRecord;

/// The store name, used in errors and the CLI's `stores` listing.
pub const NAME: &str = "dashlane";

/// Read secret records from a Dashlane export at `path`.
///
/// # Errors
///
/// Currently always returns [`Error::SourceUnsupported`] — the Dashlane parser
/// is not implemented yet.
pub fn read_export(_path: &Path) -> Result<Vec<SecretRecord>> {
    Err(Error::SourceUnsupported { store: NAME })
}

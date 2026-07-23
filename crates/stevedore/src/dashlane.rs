//! Source: read secret records from a Dashlane export.
//!
//! Dashlane exports its vault (Settings → Export) as CSV files or an encrypted
//! archive. Parsing that into [`SecretRecord`]s is the first real milestone
//! (ADR-0003); until then this reports the work as unimplemented.

use std::path::Path;

use crate::error::{Error, Result};
use crate::secret::SecretRecord;

/// Store name, used in errors and the CLI's `stores` listing.
pub const NAME: &str = "dashlane";

/// Read secret records from a Dashlane export at `path`.
///
/// # Errors
///
/// Always [`Error::SourceUnsupported`] for now — the parser isn't written yet.
pub fn read_export(_path: &Path) -> Result<Vec<SecretRecord>> {
    Err(Error::SourceUnsupported { store: NAME })
}

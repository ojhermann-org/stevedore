//! `stevedore` — move secrets between password managers and vaults.
//!
//! - [`SecretValue`] — a secret that redacts itself in `Debug` and `Display`.
//!   Read it deliberately with [`SecretValue::expose`].
//! - [`dashlane`] — read a Dashlane vault through Dashlane's own `dcli`.
//! - [`proton`] — write into a Proton Pass vault through Proton's own `pass-cli`.
//!
//! Dashlane is read, Proton Pass is written; neither does the other.

mod cli;

pub mod dashlane;
pub mod error;
pub mod proton;
pub mod secret;

pub use error::{CliError, Error, Result};
pub use secret::SecretValue;

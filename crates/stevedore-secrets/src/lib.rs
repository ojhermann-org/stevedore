//! `stevedore` — move secrets between password managers and vaults.
//!
//! - [`SecretValue`] — a secret that redacts itself in `Debug` and `Display`.
//!   Read it deliberately with [`SecretValue::expose`].
//! - [`dashlane`] — read a Dashlane vault through Dashlane's own `dcli`.
//!
//! Dashlane is the only store available today, so secrets can be read but not
//! yet written anywhere.

pub mod dashlane;
pub mod error;
pub mod secret;

pub use error::{Error, Result};
pub use secret::SecretValue;

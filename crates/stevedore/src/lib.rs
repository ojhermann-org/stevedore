//! `stevedore` — move secrets between password managers and vaults.
//!
//! The library is the cargo the CLI and MCP server both carry. It is being
//! built one store at a time, and the first is Dashlane: see [`dashlane`].
//!
//! Two decisions shape the API:
//!
//! - **Secret values redact themselves** ([`SecretValue`]) — the one thing this
//!   tool must never do is leak a value into a log. That contract is the only
//!   thing shared across stores, because it is a safety rule rather than a
//!   guess about what stores have in common.
//! - **Each store is modelled precisely, on its own terms.** A Dashlane login
//!   and a Dashlane note are separate types carrying every field Dashlane
//!   emits, and no common "record" type exists. Generalising waits until a
//!   second store has been modelled with the same care — a shared shape should
//!   be discovered from real examples, not invented ahead of them.
//!
//! Moving secrets *between* stores is therefore not wired up yet: a route needs
//! two stores, and there is one.

pub mod dashlane;
pub mod error;
pub mod secret;

pub use error::{Error, Result};
pub use secret::SecretValue;

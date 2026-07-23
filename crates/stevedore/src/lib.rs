//! `stevedore` — move secrets between password managers and vaults.
//!
//! The library is the cargo the CLI and MCP server both carry: read secret
//! records from a **source** store, optionally [`Plan`] the move, and write them
//! to a **sink** store. The first route under construction is Dashlane → Proton
//! Pass (see `docs/adr/0003-first-target-and-store-model.md`).
//!
//! Two decisions shape the API:
//!
//! - **Secret values redact themselves** ([`SecretValue`]) — the one thing this
//!   tool must never do is leak a value into a log.
//! - **There is no `Store` trait yet.** With two concrete stores it would be a
//!   guess; the source and sink are concrete modules ([`dashlane`], [`proton`])
//!   until a third or fourth reveals the real shape (ADR-0003).
//!
//! The source/sink functions are honest stubs for now — they report the work as
//! unimplemented rather than pretend to succeed — so the workspace compiles and
//! the shape is fixed while the real parsing and writing land.

pub mod dashlane;
pub mod error;
pub mod migrate;
pub mod proton;
pub mod secret;

pub use error::{Error, Result};
pub use migrate::Plan;
pub use secret::{SecretRecord, SecretValue};

//! `stevedore` — move secrets between password managers and vaults.
//!
//! The library is the cargo the CLI and MCP server both carry: read secret
//! records from a **source** store, optionally [`Plan`] the move, and write them
//! to a **sink** store. The first route under construction is Dashlane → Proton
//! Pass (see `docs/adr/0003-first-target-and-store-model.md`).
//!
//! Two design decisions shape the API and are worth stating up front:
//!
//! - **Secret values are redacting by construction.** [`SecretValue`] never
//!   reveals itself through `Debug` or `Display`; the one thing this tool must
//!   never do is leak a value into a log. Read the bytes deliberately with
//!   [`SecretValue::expose`].
//! - **There is no `Store` trait yet.** With two concrete stores an abstraction
//!   would be a guess; the source and sink are concrete modules ([`dashlane`],
//!   [`proton`]) until a third or fourth store reveals the real shape. This is a
//!   deliberate deferral, not an oversight (ADR-0003).
//!
//! Today the source/sink functions are honest stubs — they report the work as
//! unimplemented rather than pretending to succeed — so the workspace compiles
//! and the shape is fixed while the real parsing/writing lands.

pub mod dashlane;
pub mod error;
pub mod migrate;
pub mod proton;
pub mod secret;

pub use error::{Error, Result};
pub use migrate::Plan;
pub use secret::{SecretRecord, SecretValue};

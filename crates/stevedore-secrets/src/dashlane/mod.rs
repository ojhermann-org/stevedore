//! Read a Dashlane vault through Dashlane's `dcli` command-line tool.
//!
//! Secret values stay in-process, vault access is read-only, and nothing is
//! written to disk.
//!
//! stevedore **never authenticates**. Registering a device, entering the Master
//! Password and passing 2FA are a one-time setup performed with `dcli` directly
//! (see `docs/dcli/`). Every call here needs an authenticated, unlocked `dcli`
//! and fails with a clear error otherwise.
//!
//! # What can be read
//!
//! Logins ([`Login`]) and secure notes ([`Note`]) — nothing else. Passkeys,
//! payments, IDs and personal info cannot be reached. Grouping is unavailable
//! for both types: a login carries no category, and a note's [`Note::category`]
//! does not reflect Dashlane's Collections.
//!
//! Note attachments can be listed but not fetched — see [`Attachment`].

mod client;
mod login;
mod note;

pub use client::{logins, notes, status, sync, Status};
pub use login::Login;
pub use note::{Attachment, Note, NO_CATEGORY};

/// Store name, used in errors and the CLI's `stores` listing.
pub const NAME: &str = "dashlane";

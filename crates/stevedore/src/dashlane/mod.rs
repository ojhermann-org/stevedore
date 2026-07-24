//! Source: read a Dashlane vault through the `dcli` command-line tool.
//!
//! Dashlane has no public read API, so stevedore drives Dashlane's own CLI,
//! keeping values **in-process** and access **read-only**. Nothing is exported
//! to disk: a vault export would be a plaintext copy of every secret, which is
//! exactly what this tool exists to avoid creating.
//!
//! stevedore **never authenticates**. Registering a device, entering the Master
//! Password and passing 2FA are a one-time setup performed with `dcli` directly
//! (see `docs/dcli/`); this module assumes an authenticated, unlocked `dcli` and
//! refuses to run otherwise.
//!
//! # What can be read
//!
//! Logins ([`Login`]) and secure notes ([`Note`]) — nothing else. Dashlane
//! stores passkeys, payments, IDs and personal info too, but `dcli` ships no
//! command for them and they are not reachable by path either. That is a
//! ceiling, not a missing feature. Grouping is lost for both types: logins
//! expose no category at all, and a note's `category` never reflects Dashlane's
//! modern Collections.
//!
//! # Shape of the data
//!
//! The two types are modelled separately and completely, exactly as `dcli`
//! reports them. They are deliberately *not* flattened into a shared record:
//! until a second store has been modelled with the same care, any common shape
//! would be a guess.

mod client;
mod login;
mod note;

pub use client::{logins, notes, status, sync, Status};
pub use login::Login;
pub use note::{Attachment, Note, NO_CATEGORY};

/// Store name, used in errors and the CLI's `stores` listing.
pub const NAME: &str = "dashlane";

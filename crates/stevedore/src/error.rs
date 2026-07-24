//! The library's error type.

/// Errors from stevedore's source readers.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("`{program}` was not found on PATH — see docs/dcli/ for setup")]
    CliMissing { program: &'static str },

    #[error("`{program}` is not logged in — see docs/dcli/ for setup")]
    CliNotLoggedIn { program: &'static str },

    #[error("the `{program}` vault is locked — unlock it and try again")]
    CliLocked { program: &'static str },

    #[error("`{program} {args}` failed ({status}): {stderr}")]
    CliFailed {
        program: &'static str,
        args: String,
        status: String,
        stderr: String,
    },

    // No #[source] or {0}: serde_json quotes the offending value on a type
    // mismatch, and that value can be a secret.
    #[error("could not parse the {field} that `{program}` returned")]
    CliOutput {
        program: &'static str,
        field: &'static str,
    },

    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

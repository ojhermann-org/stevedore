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

    // This message deliberately drops the parser's own error. serde_json reports
    // the offending *value* on a type mismatch — `invalid type: string
    // "hunter2"` — so forwarding it would spill a password into whatever logs
    // the error. A caller gets the field name and nothing more. Do not "improve"
    // this by adding #[source] or {0}.
    #[error("could not parse the {field} that `{program}` returned")]
    CliOutput {
        program: &'static str,
        field: &'static str,
    },

    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

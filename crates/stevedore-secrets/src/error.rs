//! The library's error type.

/// A failure reading from or writing to a store.
///
/// These are store-neutral: any store can be unauthenticated, locked, return
/// data stevedore can't parse, or fail on I/O. Failures particular to *how* a
/// store is driven live in their own types — see [`CliError`] for stores driven
/// through an external command-line tool.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("the store is not authenticated")]
    NotAuthenticated,

    #[error("the store is locked")]
    Locked,

    // The message names only what failed to parse, never the parser's own
    // error: serde_json quotes the offending value on a type mismatch, and that
    // value can be a secret.
    #[error("could not parse the {what} the store returned")]
    Unparsable { what: &'static str },

    #[error(transparent)]
    Cli(#[from] CliError),

    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
}

/// A failure driving an external command-line tool a store is read through.
#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("`{program}` was not found on PATH")]
    NotFound { program: &'static str },

    #[error("`{program} {args}` failed ({status}): {stderr}")]
    Failed {
        program: &'static str,
        args: String,
        status: String,
        stderr: String,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

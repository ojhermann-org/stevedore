//! The library's error type, and the one safe bridge from store output into it.

use serde::de::DeserializeOwned;

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

    /// The store returned output stevedore couldn't parse.
    ///
    /// Carries the field name and the value-free parse position only — never the
    /// parser's own message, which would quote the offending value. Built solely
    /// by [`from_json`]; `what` is `&'static str` so a runtime value cannot be
    /// smuggled in.
    #[error("could not parse the {what} the store returned (line {line}, column {column})")]
    Unparsable {
        what: &'static str,
        line: usize,
        column: usize,
    },

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

/// The one bridge from untrusted store output to a typed value.
///
/// On a type mismatch `serde_json` quotes the offending value in its error
/// (`invalid type: string "hunter2", …`), and that value can be a secret. This
/// is the only function permitted to hold a `serde_json::Error`: it drops the
/// message, keeping just the caller's field name and the parse position, which
/// carries no value. Everything that parses store output goes through here, so
/// the "never forward a parser's message" rule lives in one auditable place.
pub(crate) fn from_json<T: DeserializeOwned>(bytes: &[u8], what: &'static str) -> Result<T> {
    serde_json::from_slice(bytes).map_err(|e| Error::Unparsable {
        what,
        line: e.line(),
        column: e.column(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Sample {
        #[allow(dead_code)]
        count: i64,
    }

    #[test]
    fn from_json_never_echoes_the_offending_value() {
        // A string where an integer is expected makes serde_json quote the value;
        // from_json must not carry that quote into the error, in Display or Debug.
        let err = from_json::<Sample>(br#"{"count":"SEKRET-MARKER"}"#, "sample").unwrap_err();
        assert!(
            !format!("{err}").contains("SEKRET-MARKER"),
            "Display: {err}"
        );
        assert!(
            !format!("{err:?}").contains("SEKRET-MARKER"),
            "Debug: {err:?}"
        );
    }
}

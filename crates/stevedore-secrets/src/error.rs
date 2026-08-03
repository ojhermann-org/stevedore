//! The library's error type, and the one safe bridge from store output into it.

use serde::de::DeserializeOwned;

/// A failure reading from or writing to a store.
///
/// These are store-neutral: any store can be unauthenticated, locked, return
/// data stevedore can't parse, or fail on I/O. Failures particular to *how* a
/// store is driven live in their own types — see [`CliError`] for stores driven
/// through an external command-line tool.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The store has no usable session. stevedore never authenticates; logging
    /// the store's CLI in is a separate, one-time setup.
    #[error("the store is not authenticated")]
    NotAuthenticated,

    /// The store is authenticated but sealed, so it will not serve a read.
    #[error("the store is locked")]
    Locked,

    /// The store returned output stevedore couldn't parse.
    ///
    /// Carries the field name and the value-free parse position only — never the
    /// parser's own message, which would quote the offending value. Built solely
    /// by `from_json`; `what` is `&'static str` so a runtime value cannot be
    /// smuggled in.
    #[error("could not parse the {what} the store returned (line {line}, column {column})")]
    Unparsable {
        /// What was being parsed, e.g. `"logins"`.
        what: &'static str,
        /// Line the parse stopped at.
        line: usize,
        /// Column the parse stopped at.
        column: usize,
    },

    /// stevedore could not build an item to hand to the store.
    ///
    /// Carries no detail, for the reason [`Unparsable`] carries so little: a
    /// serializer's message can quote the value that failed.
    ///
    /// [`Unparsable`]: Error::Unparsable
    #[error("could not build the item to send to the store")]
    Unserializable,

    /// The store has no vault by that name. A vault name is user-chosen
    /// metadata, never a secret.
    #[error("the store has no vault named `{name}`")]
    NoSuchVault {
        /// The vault name that was asked for.
        name: String,
    },

    /// A failure driving the store's command-line tool.
    #[error(transparent)]
    Cli(#[from] CliError),

    /// An I/O failure talking to the store's command-line tool.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// A failure driving an external command-line tool a store is worked through.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CliError {
    /// The tool is not installed, or not on `PATH`.
    #[error("`{program}` was not found on PATH")]
    NotFound {
        /// The command that was looked for.
        program: &'static str,
    },

    /// The tool ran and exited non-zero.
    #[error("`{program} {args}` failed ({status}): {stderr}")]
    Failed {
        /// The command that ran.
        program: &'static str,
        /// Its arguments, joined by spaces. Never carries a secret: values go
        /// on stdin.
        args: String,
        /// How it exited.
        status: String,
        /// Its stderr, stripped of escape codes. Its stdout is never included —
        /// that may be vault contents.
        stderr: String,
    },
}

/// The library's result type.
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
        #[expect(dead_code, reason = "exists only for serde to fail parsing into")]
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

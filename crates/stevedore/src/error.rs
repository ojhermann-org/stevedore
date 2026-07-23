//! The library's error type.

/// Errors returned by stevedore's source readers, sink writers, and migration
/// planning.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A source store cannot yet be read from.
    #[error("reading from {store} is not implemented yet (see ADR-0003)")]
    SourceUnsupported {
        /// The source store that was requested.
        store: &'static str,
    },

    /// A sink store cannot yet be written to.
    #[error("writing to {store} is not implemented yet (see ADR-0003)")]
    SinkUnsupported {
        /// The sink store that was requested.
        store: &'static str,
    },

    /// An underlying I/O failure (reading an export file, etc.).
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
}

/// Convenience alias for results in this crate.
pub type Result<T> = std::result::Result<T, Error>;

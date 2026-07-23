//! The library's error type.

/// Errors from stevedore's source readers, sink writers, and migration planning.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("reading from {store} is not implemented yet (see ADR-0003)")]
    SourceUnsupported { store: &'static str },

    #[error("writing to {store} is not implemented yet (see ADR-0003)")]
    SinkUnsupported { store: &'static str },

    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

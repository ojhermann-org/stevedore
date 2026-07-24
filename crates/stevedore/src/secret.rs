//! The redaction contract every source shares.

use std::fmt;

use serde::Deserialize;

/// A secret value that never reveals itself through `Debug` or `Display`.
///
/// Since this tool's whole job is moving secret *values*, the value type is
/// redacting by construction: logging a record that contains one can't spill it.
/// Read the bytes deliberately — and greppably — with [`SecretValue::expose`].
///
/// This is the one type shared across sources. It isn't a guess at what stores
/// have in common; it encodes the safety rule, which is the same everywhere.
///
/// `Deserialize` is derived so a source can parse straight into a redacting
/// type, never through an intermediate `String`. `Serialize` is deliberately
/// **not** implemented — a secret should never be written out by accident.
#[derive(Clone, PartialEq, Eq, Deserialize)]
#[serde(transparent)]
pub struct SecretValue(String);

impl SecretValue {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Borrow the underlying secret. Named `expose` so read sites are greppable.
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for SecretValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecretValue(<redacted>)")
    }
}

impl fmt::Display for SecretValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<redacted>")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redacts_in_debug_and_display() {
        let v = SecretValue::new("hunter2");
        assert_eq!(format!("{v}"), "<redacted>");
        assert_eq!(format!("{v:?}"), "SecretValue(<redacted>)");
        assert!(!format!("{v:?}").contains("hunter2"));
        assert_eq!(v.expose(), "hunter2");
    }

    #[test]
    fn deserializes_without_an_intermediate_string() {
        let v: SecretValue = serde_json::from_str(r#""hunter2""#).unwrap();
        assert_eq!(v.expose(), "hunter2");
        assert!(!format!("{v:?}").contains("hunter2"));
    }
}

//! The unit of cargo: a named secret and the metadata a vault keeps beside it.

use std::fmt;

/// A secret value that never reveals itself through `Debug` or `Display`.
///
/// Since this tool's whole job is moving secret *values*, the value type is
/// redacting by construction: logging a [`SecretRecord`] can't spill the secret.
/// Read the bytes deliberately — and greppably — with [`SecretValue::expose`].
#[derive(Clone, PartialEq, Eq)]
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

/// A named secret plus the metadata a vault keeps alongside it.
///
/// Fields beyond `name`/`value` are best-effort — not every store carries every
/// one, so they are optional and preserved when the source provides them.
#[derive(Clone, Debug)]
pub struct SecretRecord {
    pub name: String,
    pub value: SecretValue,
    pub folder: Option<String>,
    pub username: Option<String>,
    pub url: Option<String>,
    pub note: Option<String>,
}

impl SecretRecord {
    pub fn new(name: impl Into<String>, value: SecretValue) -> Self {
        Self {
            name: name.into(),
            value,
            folder: None,
            username: None,
            url: None,
            note: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secret_value_redacts_in_debug_and_display() {
        let v = SecretValue::new("hunter2");
        assert_eq!(format!("{v}"), "<redacted>");
        assert_eq!(format!("{v:?}"), "SecretValue(<redacted>)");
        assert!(!format!("{v:?}").contains("hunter2"));
        assert_eq!(v.expose(), "hunter2");
    }

    #[test]
    fn record_debug_does_not_leak_the_value() {
        let r = SecretRecord::new("api-key", SecretValue::new("s3cr3t"));
        let shown = format!("{r:?}");
        assert!(
            !shown.contains("s3cr3t"),
            "record Debug leaked the secret: {shown}"
        );
        assert!(shown.contains("api-key"));
    }
}

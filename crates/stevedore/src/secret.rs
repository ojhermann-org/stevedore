//! The unit of cargo: a named secret and the metadata a vault keeps beside it.

use std::fmt;

/// A secret value that never reveals itself through `Debug` or `Display`.
///
/// The whole point of this tool is to move secret *values* between stores, so
/// the value type is redacting by construction: logging a [`SecretRecord`], or
/// the value alone, prints `<redacted>` rather than the secret. Read the bytes
/// deliberately — and greppably — with [`SecretValue::expose`].
#[derive(Clone, PartialEq, Eq)]
pub struct SecretValue(String);

impl SecretValue {
    /// Wrap a secret value.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Borrow the underlying secret. Named `expose` so every read site is easy
    /// to audit with a grep.
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

/// One item moved between stores: a named secret plus the metadata a vault keeps
/// alongside it.
///
/// Fields beyond `name`/`value` are best-effort — not every store carries every
/// field, so they are optional and preserved when the source provides them.
#[derive(Clone, Debug)]
pub struct SecretRecord {
    /// Human-facing name / title of the entry.
    pub name: String,
    /// The secret itself (redacting — see [`SecretValue`]).
    pub value: SecretValue,
    /// Folder / group path in the source store, if any.
    pub folder: Option<String>,
    /// Associated login / username, if any.
    pub username: Option<String>,
    /// Associated URL, if any.
    pub url: Option<String>,
    /// Free-form note attached to the entry, if any.
    pub note: Option<String>,
}

impl SecretRecord {
    /// A minimal record: just a name and its value. Metadata can be filled in
    /// afterward via the public fields.
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
        // The value is still readable deliberately.
        assert_eq!(v.expose(), "hunter2");
    }

    #[test]
    fn record_debug_does_not_leak_the_value() {
        let r = SecretRecord::new("api-key", SecretValue::new("s3cr3t"));
        let shown = format!("{r:?}");
        assert!(!shown.contains("s3cr3t"), "record Debug leaked the secret: {shown}");
        assert!(shown.contains("api-key"));
    }
}

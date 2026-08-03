//! The items stevedore creates, shaped as `pass-cli`'s own item templates
//! (`pass-cli item create <kind> --get-template`).

use serde::{Serialize, Serializer};

use crate::secret::SecretValue;

/// A login to create in a Proton Pass vault.
///
/// Only `title` is required; Proton accepts an item with every other field
/// empty.
#[derive(Debug, Clone, Serialize)]
pub struct NewLogin {
    pub title: String,

    pub username: Option<String>,
    pub email: Option<String>,

    #[serde(serialize_with = "expose")]
    pub password: Option<SecretValue>,

    /// A 2FA token as an `otpauth://` URI. Secret despite the name — the TOTP
    /// seed is in the query string.
    #[serde(serialize_with = "expose")]
    pub totp_uri: Option<SecretValue>,

    pub urls: Vec<String>,
}

impl NewLogin {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            username: None,
            email: None,
            password: None,
            totp_uri: None,
            urls: Vec::new(),
        }
    }
}

/// A secure note to create in a Proton Pass vault.
#[derive(Debug, Clone, Serialize)]
pub struct NewNote {
    pub title: String,

    /// The note body — the secret this item exists to hold.
    #[serde(serialize_with = "expose")]
    pub note: Option<SecretValue>,
}

impl NewNote {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            note: None,
        }
    }
}

/// The one place a secret is written out.
///
/// `SecretValue` has no `Serialize` impl by design, so every serialization of a
/// secret goes through this function — and its output goes only to a `pass-cli`
/// stdin pipe, never to a file, an argument or a log.
#[expect(
    clippy::ref_option,
    reason = "serde's `serialize_with` fixes this signature; `Option<&T>` is not available"
)]
fn expose<S: Serializer>(secret: &Option<SecretValue>, serializer: S) -> Result<S::Ok, S::Error> {
    match secret {
        Some(secret) => serializer.serialize_str(secret.expose()),
        None => serializer.serialize_none(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `pass-cli item create login --get-template`, verbatim.
    const LOGIN_TEMPLATE: &str = r#"{
        "title": "",
        "username": null,
        "email": null,
        "password": null,
        "totp_uri": null,
        "urls": []
    }"#;

    /// `pass-cli item create note --get-template`, verbatim.
    const NOTE_TEMPLATE: &str = r#"{
        "title": "",
        "note": null
    }"#;

    fn keys(json: &serde_json::Value) -> Vec<&String> {
        json.as_object()
            .expect("an item is a JSON object")
            .keys()
            .collect()
    }

    fn login() -> NewLogin {
        let mut login = NewLogin::new("example");
        login.username = Some("otto".to_owned());
        login.password = Some(SecretValue::new("hunter2"));
        login.totp_uri = Some(SecretValue::new("otpauth://totp/?secret=jbswy3dpehpk3pxp"));
        login.urls = vec!["https://example.test".to_owned()];
        login
    }

    #[test]
    fn a_login_serializes_to_the_pass_cli_template() {
        let json = serde_json::to_value(login()).unwrap();
        let template: serde_json::Value = serde_json::from_str(LOGIN_TEMPLATE).unwrap();
        assert_eq!(keys(&json), keys(&template));
        assert_eq!(json["password"], "hunter2");
        assert_eq!(json["email"], serde_json::Value::Null);
    }

    #[test]
    fn a_note_serializes_to_the_pass_cli_template() {
        let mut note = NewNote::new("example note");
        note.note = Some(SecretValue::new("the body"));
        let json = serde_json::to_value(note).unwrap();
        let template: serde_json::Value = serde_json::from_str(NOTE_TEMPLATE).unwrap();
        assert_eq!(keys(&json), keys(&template));
        assert_eq!(json["note"], "the body");
    }

    #[test]
    fn an_empty_item_serializes_its_absent_fields_as_null() {
        let json: serde_json::Value = serde_json::to_value(NewLogin::new("bare")).unwrap();
        assert_eq!(json["password"], serde_json::Value::Null);
        assert_eq!(json["totp_uri"], serde_json::Value::Null);
        assert_eq!(json["urls"], serde_json::json!([]));
    }

    #[test]
    fn debug_leaks_neither_password_nor_otp_seed() {
        let shown = format!("{:?}", login());
        assert!(!shown.contains("hunter2"), "leaked password: {shown}");
        assert!(
            !shown.contains("jbswy3dpehpk3pxp"),
            "leaked OTP seed: {shown}"
        );
        assert!(shown.contains("example"), "should still show metadata");
    }
}

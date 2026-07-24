//! A Dashlane login, exactly as `dcli password -o json` reports it.

use serde::Deserialize;

use crate::secret::SecretValue;

/// One login from a Dashlane vault.
///
/// Every field Dashlane emits is modelled, including the ones that only mean
/// something inside Dashlane's own UI.
///
/// **Everything is a string.** Dashlane serialises booleans (`auto_login`),
/// numbers (`number_use`, `strength`) and epoch timestamps
/// (`creation_datetime`) all as JSON strings, so they are modelled as strings
/// and left unparsed. Typing `auto_login` as `bool` fails to deserialize.
///
/// Only [`Login::id`] is guaranteed present; every other field is optional
/// because real vaults omit them — `title` and `url` included.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Login {
    /// Dashlane's identifier, unique per record and present on every one.
    ///
    /// Emitted wrapped in braces (`{D47734C4-…}`). A `dl://` path only resolves
    /// with the braces **stripped**, so this value cannot be pasted into one
    /// unchanged — see [`Login::bare_id`].
    pub id: String,

    pub title: Option<String>,
    pub url: Option<String>,
    pub user_selected_url: Option<String>,
    pub use_fixed_url: Option<String>,
    pub subdomain_only: Option<String>,

    /// The primary username. Present on far fewer records than `email`.
    pub login: Option<String>,
    pub email: Option<String>,
    pub secondary_login: Option<String>,

    pub password: Option<SecretValue>,

    /// A 2FA token as an `otpauth://` URI, when the login has one.
    ///
    /// **Secret material despite the name.** The TOTP seed sits in the query
    /// string, so this is redacted like a password — anything that logs "just
    /// the URL fields" would otherwise leak a second factor.
    ///
    /// Dashlane's form is non-standard in two ways worth knowing before anyone
    /// hands it to another tool: the label is **empty**
    /// (`otpauth://totp/?secret=…` rather than `otpauth://totp/Issuer:account?…`),
    /// and it carries a Dashlane-specific `lock` parameter. The seed also comes
    /// back lower-cased, which is legal base32 but not universally accepted.
    pub otp_url: Option<SecretValue>,

    /// Free-text note attached to the login. Redacted: users keep recovery
    /// codes and secondary passwords here.
    pub note: Option<SecretValue>,

    // No `category` field exists on a login. Dashlane stores login categories
    // (and Collections) elsewhere and `dcli` exposes neither, so a login's
    // grouping is simply not readable — don't add a field hoping it appears.
    pub status: Option<String>,
    pub strength: Option<String>,
    pub is_favorite: Option<String>,
    pub auto_login: Option<String>,
    pub auto_protected: Option<String>,
    pub checked: Option<String>,
    pub number_use: Option<String>,
    pub anon_id: Option<String>,
    pub locale_format: Option<String>,

    /// Nested JSON *inside* a JSON string, e.g. `"{\"associated_domains\":[]}"`.
    ///
    /// Left as the raw string on purpose: the only example ever observed was
    /// empty, so parsing it would mean inventing a shape from no evidence.
    pub linked_services: Option<String>,

    pub creation_datetime: Option<String>,
    pub modification_datetime: Option<String>,
    pub user_modification_datetime: Option<String>,
    pub last_backup_time: Option<String>,
    pub last_use: Option<String>,
}

impl Login {
    /// The id with Dashlane's surrounding braces removed.
    ///
    /// `dcli read dl://<id>/…` rejects the braced form and reports the record as
    /// missing rather than malformed, which reads as data loss. Filters
    /// (`dcli password id=…`) accept either.
    pub fn bare_id(&self) -> &str {
        self.id.trim_start_matches('{').trim_end_matches('}')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Fabricated. Never paste real vault output into a test.
    const FULL: &str = r#"{
        "id": "{D47734C4-0ABE-423A-8633-6B9F10A38905}",
        "title": "example",
        "url": "https://example.test",
        "login": "otto",
        "email": "otto@example.test",
        "secondaryLogin": "me",
        "password": "hunter2",
        "otpUrl": "otpauth://totp/?secret=jbswy3dpehpk3pxp&lock=false",
        "note": "recovery code 12345",
        "autoLogin": "false",
        "numberUse": "7",
        "linkedServices": "{\"associated_domains\":[]}",
        "creationDatetime": "1784841592",
        "unknownFutureField": "ignored"
    }"#;

    fn full() -> Login {
        serde_json::from_str(FULL).expect("fixture should parse")
    }

    #[test]
    fn parses_every_modelled_field() {
        let l = full();
        assert_eq!(l.title.as_deref(), Some("example"));
        assert_eq!(l.login.as_deref(), Some("otto"));
        assert_eq!(l.secondary_login.as_deref(), Some("me"));
        assert_eq!(l.password.as_ref().unwrap().expose(), "hunter2");
        assert_eq!(l.number_use.as_deref(), Some("7"));
    }

    #[test]
    fn ignores_fields_we_do_not_model() {
        assert_eq!(full().id, "{D47734C4-0ABE-423A-8633-6B9F10A38905}");
    }

    #[test]
    fn absent_fields_become_none() {
        let l: Login = serde_json::from_str(r#"{"id": "{A}"}"#).unwrap();
        assert!(l.title.is_none());
        assert!(l.password.is_none());
        assert!(l.otp_url.is_none());
    }

    #[test]
    fn a_record_without_an_id_is_rejected() {
        assert!(serde_json::from_str::<Login>(r#"{"title": "x"}"#).is_err());
    }

    #[test]
    fn bare_id_strips_the_braces_dl_paths_reject() {
        assert_eq!(full().bare_id(), "D47734C4-0ABE-423A-8633-6B9F10A38905");
    }

    #[test]
    fn debug_leaks_no_password_otp_seed_or_note() {
        let shown = format!("{:?}", full());
        assert!(!shown.contains("hunter2"), "leaked password: {shown}");
        assert!(
            !shown.contains("jbswy3dpehpk3pxp"),
            "leaked OTP seed: {shown}"
        );
        assert!(!shown.contains("12345"), "leaked note: {shown}");
        assert!(shown.contains("example"), "should still show metadata");
    }
}

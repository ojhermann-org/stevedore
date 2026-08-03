//! A Dashlane login, exactly as `dcli password -o json` reports it.

use serde::Deserialize;

use crate::secret::SecretValue;

/// One login from a Dashlane vault, with every field Dashlane emits.
///
/// Every value is a JSON string — booleans, numbers and epoch timestamps
/// included — so all are modelled as strings and left unparsed. Only [`id`] is
/// guaranteed present; every other field is optional, `title` and `url`
/// included.
///
/// [`id`]: Login::id
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Login {
    /// Dashlane's per-record identifier, present on every one.
    ///
    /// Emitted in braces (`{D47734C4-…}`); see [`Login::bare_id`].
    pub id: String,

    /// The login's name in Dashlane. A record can be untitled.
    pub title: Option<String>,

    /// The site the login is for.
    pub url: Option<String>,

    /// URL-matching settings. Dashlane documents none of their semantics, so
    /// stevedore carries them without interpreting them.
    pub user_selected_url: Option<String>,
    /// See [`Login::user_selected_url`].
    pub use_fixed_url: Option<String>,
    /// See [`Login::user_selected_url`].
    pub subdomain_only: Option<String>,

    /// The primary username. Present on far fewer records than `email`.
    pub login: Option<String>,

    /// The account email.
    pub email: Option<String>,

    /// An alternate username Dashlane can fill instead of [`Login::login`].
    pub secondary_login: Option<String>,

    /// The login's password. Redacted.
    pub password: Option<SecretValue>,

    /// A 2FA token as an `otpauth://` URI, when the login has one.
    ///
    /// Secret material despite the name — the TOTP seed is in the query string,
    /// so it is redacted. Dashlane's form is non-standard: empty label, a `lock`
    /// parameter, and a lower-cased seed.
    pub otp_url: Option<SecretValue>,

    /// Free-text note attached to the login. Redacted.
    pub note: Option<SecretValue>,

    // dcli exposes no category on a login; don't add a field hoping it appears.
    /// Dashlane's own status marker for the record.
    pub status: Option<String>,

    /// Dashlane's assessment of the password's strength.
    pub strength: Option<String>,

    /// `"true"` when the login is starred in Dashlane's UI.
    pub is_favorite: Option<String>,

    /// Whether Dashlane submits the login form automatically.
    pub auto_login: Option<String>,

    /// Whether Dashlane requires the Master Password before filling this login.
    pub auto_protected: Option<String>,

    /// Reported by `dcli` with no documented meaning.
    pub checked: Option<String>,

    /// How many times the login has been used, as a string.
    pub number_use: Option<String>,

    /// An opaque per-record identifier, distinct from [`Login::id`].
    pub anon_id: Option<String>,

    /// Dashlane's locale marker for the record, e.g. `"UNIVERSAL"`.
    pub locale_format: Option<String>,

    /// Nested JSON inside a JSON string (e.g. `"{\"associated_domains\":[]}"`),
    /// left unparsed.
    pub linked_services: Option<String>,

    /// When the login was created — epoch seconds, as a string.
    pub creation_datetime: Option<String>,
    /// When the record last changed, Dashlane's own edits included.
    pub modification_datetime: Option<String>,
    /// When the user last changed the record.
    pub user_modification_datetime: Option<String>,
    /// When Dashlane last backed the record up.
    pub last_backup_time: Option<String>,
    /// When the login was last used.
    pub last_use: Option<String>,
}

impl Login {
    /// The id with Dashlane's surrounding braces removed.
    ///
    /// A `dl://<id>` path rejects the braced form; `dcli password id=…` accepts
    /// either.
    #[must_use]
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
        serde_json::from_str::<Login>(r#"{"title": "x"}"#).unwrap_err();
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

//! A Dashlane secret, exactly as `dcli secret -o json` reports it.

use serde::Deserialize;

use crate::secret::SecretValue;

/// One secret from a Dashlane vault, with every field Dashlane emits.
///
/// Its secret is [`Secret::content`]. As with a login or a note, every value is
/// a string.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Secret {
    /// Braced identifier, as on a login. See [`Secret::bare_id`].
    pub id: String,

    pub title: Option<String>,

    /// The secret's body — the value this record exists to hold.
    pub content: Option<SecretValue>,

    /// `"true"` when the secret is marked *protected* in Dashlane's UI, as on a
    /// note. See [`Secret::is_secured`].
    pub secured: Option<String>,

    pub category: Option<String>,
    pub anon_id: Option<String>,
    pub space_id: Option<String>,
    pub shared_object: Option<String>,
    pub locale_format: Option<String>,

    /// Dashlane's `type` field. Its meaning is undocumented, and unlike a note's
    /// it is not known to carry a colour.
    #[serde(rename = "type")]
    pub kind: Option<String>,

    pub creation_date: Option<String>,
    pub creation_date_time: Option<String>,
    pub update_date: Option<String>,
    pub last_backup_time: Option<String>,
    pub user_modification_datetime: Option<String>,
}

impl Secret {
    /// The id with Dashlane's surrounding braces removed, when it has them.
    pub fn bare_id(&self) -> &str {
        self.id.trim_start_matches('{').trim_end_matches('}')
    }

    /// Whether Dashlane marks this secret as protected. `dcli` returns the
    /// content either way, so this does not imply it was withheld.
    pub fn is_secured(&self) -> bool {
        self.secured.as_deref() == Some("true")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Fabricated. Never paste real vault output into a test.
    const FULL: &str = r#"{
        "id": "{9C1E0B4A-77D2-4E31-9F55-2A1C8B6D3E04}",
        "title": "deploy key",
        "content": "the secret body",
        "secured": "true",
        "category": "noCategory",
        "anonId": "8f2c1d",
        "spaceId": "",
        "localeFormat": "UNIVERSAL",
        "type": "SECRET",
        "creationDate": "1784841592",
        "creationDateTime": "1784841592",
        "lastBackupTime": "1784841600",
        "unknownFutureField": "ignored"
    }"#;

    fn full() -> Secret {
        serde_json::from_str(FULL).expect("fixture should parse")
    }

    #[test]
    fn parses_every_modelled_field() {
        let s = full();
        assert_eq!(s.title.as_deref(), Some("deploy key"));
        assert_eq!(s.content.as_ref().unwrap().expose(), "the secret body");
        assert_eq!(s.anon_id.as_deref(), Some("8f2c1d"));
        assert_eq!(s.locale_format.as_deref(), Some("UNIVERSAL"));
        assert_eq!(s.kind.as_deref(), Some("SECRET"));
        assert!(s.is_secured());
    }

    #[test]
    fn ignores_fields_we_do_not_model() {
        assert_eq!(full().id, "{9C1E0B4A-77D2-4E31-9F55-2A1C8B6D3E04}");
    }

    #[test]
    fn absent_fields_become_none() {
        let s: Secret = serde_json::from_str(r#"{"id": "{A}"}"#).unwrap();
        assert!(s.title.is_none());
        assert!(s.content.is_none());
        assert!(!s.is_secured());
    }

    #[test]
    fn a_record_without_an_id_is_rejected() {
        assert!(serde_json::from_str::<Secret>(r#"{"title": "x"}"#).is_err());
    }

    #[test]
    fn bare_id_strips_the_braces_dl_paths_reject() {
        assert_eq!(full().bare_id(), "9C1E0B4A-77D2-4E31-9F55-2A1C8B6D3E04");
    }

    #[test]
    fn debug_leaks_no_content() {
        let shown = format!("{:?}", full());
        assert!(
            !shown.contains("the secret body"),
            "leaked content: {shown}"
        );
        assert!(shown.contains("deploy key"), "should still show metadata");
    }
}

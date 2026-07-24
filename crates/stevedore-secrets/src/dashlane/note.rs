//! A Dashlane secure note, exactly as `dcli note -o json` reports it.

use serde::{de, Deserialize, Deserializer};

use crate::secret::SecretValue;

/// One secure note from a Dashlane vault, with every field Dashlane emits.
///
/// Its secret is [`Note::content`]. As with a login, every value is a string.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    /// Braced identifier, as on a login. See [`Note::bare_id`].
    pub id: String,

    pub title: Option<String>,

    /// The note body — the secret this record exists to hold.
    pub content: Option<SecretValue>,

    /// Legacy grouping field. It does not reflect Dashlane's Collections, and
    /// reports the [`NO_CATEGORY`] sentinel when a note is ungrouped — see
    /// [`Note::is_ungrouped`].
    pub category: Option<String>,

    /// `"true"` when the note is marked *protected* in Dashlane's UI. Reported
    /// but not enforced — see [`Note::is_secured`].
    pub secured: Option<String>,

    /// The note's colour in Dashlane's UI (e.g. `"GRAY"`).
    #[serde(rename = "type")]
    pub colour: Option<String>,

    /// Files attached to the note. See [`Attachment`].
    #[serde(default, deserialize_with = "attachments")]
    pub attachments: Vec<Attachment>,

    pub locale_format: Option<String>,
    pub creation_date: Option<String>,
    pub update_date: Option<String>,
    pub creation_datetime: Option<String>,
    pub user_modification_datetime: Option<String>,
    pub last_backup_time: Option<String>,
    pub last_use: Option<String>,
}

impl Note {
    /// The id with Dashlane's surrounding braces removed.
    pub fn bare_id(&self) -> &str {
        self.id.trim_start_matches('{').trim_end_matches('}')
    }

    /// Whether Dashlane marks this note as protected. `dcli` returns the
    /// content either way, so this does not imply it was withheld.
    pub fn is_secured(&self) -> bool {
        self.secured.as_deref() == Some("true")
    }

    /// Whether the note belongs to no category, accounting for the sentinel.
    pub fn is_ungrouped(&self) -> bool {
        matches!(
            self.category.as_deref(),
            None | Some("") | Some(NO_CATEGORY)
        )
    }
}

/// Dashlane's stand-in for "no category" — a literal string, not an omission.
pub const NO_CATEGORY: &str = "noCategory";

/// A file attached to a secure note — metadata only.
///
/// The file itself cannot be read: the payload lives on Dashlane's servers and
/// `dcli` has no command to fetch or decrypt it. Its numbers are real JSON
/// numbers, unlike everywhere else.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub id: Option<String>,
    pub filename: Option<String>,

    /// MIME type, e.g. `"application/pdf"`.
    #[serde(rename = "type")]
    pub mime_type: Option<String>,

    /// The key the file is encrypted with. Redacted.
    pub crypto_key: Option<SecretValue>,

    /// Where the payload lives on Dashlane's servers. Redacted.
    pub download_key: Option<SecretValue>,

    /// The account the file belongs to — an email address.
    pub owner: Option<String>,

    pub local_size: Option<i64>,
    pub remote_size: Option<i64>,
    pub version: Option<i64>,
    pub space_id: Option<String>,
    pub locale_format: Option<String>,
    pub creation_datetime: Option<i64>,
    pub user_modification_datetime: Option<i64>,
    pub last_backup_time: Option<i64>,
    pub last_use: Option<i64>,

    #[serde(rename = "__type__")]
    pub dashlane_type: Option<String>,
}

/// Parse the `attachments` field, which Dashlane double-encodes as JSON text
/// inside a JSON string.
fn attachments<'de, D>(deserializer: D) -> Result<Vec<Attachment>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = Option::<String>::deserialize(deserializer)?;
    match raw.as_deref().map(str::trim) {
        None | Some("") => Ok(Vec::new()),
        // Discard serde's message: it quotes the offending value, a cryptoKey.
        Some(text) => serde_json::from_str(text)
            .map_err(|_| de::Error::custom("attachments is not the expected JSON")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Fabricated — including the key, which is a plausible-looking fake.
    const WITH_ATTACHMENT: &str = r#"{
        "id": "{341959E0-E3EB-4873-B358-99F78699AAE7}",
        "title": "example note",
        "content": "the body",
        "category": "noCategory",
        "secured": "true",
        "type": "GRAY",
        "attachments": "[{\"cryptoKey\":\"AAAABBBBCCCCDDDD=\",\"downloadKey\":\"12345/abcdef\",\"filename\":\"paper.pdf\",\"type\":\"application/pdf\",\"localSize\":151206,\"owner\":\"otto@example.test\",\"__type__\":\"KWSecureFileInfo\"}]"
    }"#;

    #[test]
    fn parses_a_note() {
        let n: Note = serde_json::from_str(WITH_ATTACHMENT).unwrap();
        assert_eq!(n.title.as_deref(), Some("example note"));
        assert_eq!(n.content.as_ref().unwrap().expose(), "the body");
        assert_eq!(n.colour.as_deref(), Some("GRAY"));
        assert!(n.is_secured());
    }

    #[test]
    fn the_no_category_sentinel_counts_as_ungrouped() {
        let n: Note = serde_json::from_str(WITH_ATTACHMENT).unwrap();
        assert_eq!(n.category.as_deref(), Some("noCategory"));
        assert!(n.is_ungrouped(), "the sentinel is not a real category name");
    }

    #[test]
    fn parses_double_encoded_attachments() {
        let n: Note = serde_json::from_str(WITH_ATTACHMENT).unwrap();
        let a = &n.attachments[0];
        assert_eq!(a.filename.as_deref(), Some("paper.pdf"));
        assert_eq!(a.local_size, Some(151_206));
        assert_eq!(a.crypto_key.as_ref().unwrap().expose(), "AAAABBBBCCCCDDDD=");
    }

    #[test]
    fn an_empty_attachment_list_parses() {
        let n: Note = serde_json::from_str(r#"{"id":"{A}","attachments":"[]"}"#).unwrap();
        assert!(n.attachments.is_empty());
    }

    #[test]
    fn a_malformed_attachment_never_echoes_its_contents() {
        // localSize is an integer; a string value forces serde_json to quote it.
        // The custom deserializer must discard that message, not forward it —
        // the value could be a real cryptoKey.
        let json = r#"{"id":"{A}","attachments":"[{\"localSize\":\"SEKRET-MARKER\"}]"}"#;
        let err = serde_json::from_str::<Note>(json).unwrap_err();
        assert!(
            !format!("{err}").contains("SEKRET-MARKER"),
            "Display: {err}"
        );
        assert!(
            !format!("{err:?}").contains("SEKRET-MARKER"),
            "Debug: {err:?}"
        );
    }

    #[test]
    fn a_missing_attachment_field_parses() {
        let n: Note = serde_json::from_str(r#"{"id":"{A}"}"#).unwrap();
        assert!(n.attachments.is_empty());
        assert!(!n.is_secured());
        assert!(n.is_ungrouped());
    }

    #[test]
    fn debug_leaks_neither_content_nor_crypto_key() {
        let n: Note = serde_json::from_str(WITH_ATTACHMENT).unwrap();
        let shown = format!("{n:?}");
        assert!(!shown.contains("the body"), "leaked content: {shown}");
        assert!(!shown.contains("AAAABBBBCCCCDDDD"), "leaked key: {shown}");
        assert!(
            !shown.contains("12345/abcdef"),
            "leaked download key: {shown}"
        );
        assert!(shown.contains("paper.pdf"), "should still show metadata");
    }
}

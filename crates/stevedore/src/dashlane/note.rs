//! A Dashlane secure note, exactly as `dcli note -o json` reports it.

use serde::{de, Deserialize, Deserializer};

use crate::secret::SecretValue;

/// One secure note from a Dashlane vault.
///
/// A note shares almost nothing with a login: no username, no url, no password.
/// Its secret is [`Note::content`].
///
/// As with logins, every value Dashlane emits is a string.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    /// Braced identifier, as on a login. See [`Note::bare_id`].
    pub id: String,

    pub title: Option<String>,

    /// The note body — the secret this record exists to hold.
    pub content: Option<SecretValue>,

    /// Legacy grouping field. **Not** Dashlane's modern *Collections*: a note
    /// placed in a collection still reports the sentinel `"noCategory"` here,
    /// so collection membership is not readable at all.
    ///
    /// The sentinel is a string, not an absent field — treat `"noCategory"` as
    /// "ungrouped" rather than passing it along as a name.
    pub category: Option<String>,

    /// `"true"` when the note is marked *protected* in Dashlane's UI.
    ///
    /// The flag is reported but not enforced: `dcli` returns `content` for a
    /// protected note with no prompt and no error. A user who ticks this box
    /// gains nothing against anyone holding an unlocked `dcli`.
    pub secured: Option<String>,

    /// The note's colour in Dashlane's UI (e.g. `"GRAY"`).
    #[serde(rename = "type")]
    pub colour: Option<String>,

    /// Files attached to the note.
    ///
    /// Arrives as JSON *inside* a JSON string and is parsed here, so the
    /// [`Attachment::crypto_key`] inside can be given a redacting type instead
    /// of sitting in a plain `String`.
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

    /// Whether Dashlane marks this note as protected.
    ///
    /// Reports the flag only. It does not imply the content was withheld —
    /// `dcli` hands it over regardless.
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

/// A file attached to a secure note.
///
/// **Metadata only — the file itself cannot be read.** The payload lives on
/// Dashlane's servers behind [`Attachment::download_key`], and `dcli` ships no
/// command to fetch or decrypt it. An attachment can therefore be observed and
/// reported, never moved.
///
/// Unlike every other Dashlane record, the numbers in here are real JSON
/// numbers rather than strings.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub id: Option<String>,
    pub filename: Option<String>,

    /// MIME type, e.g. `"application/pdf"`.
    #[serde(rename = "type")]
    pub mime_type: Option<String>,

    /// The key the file is encrypted with — secret material, redacted.
    pub crypto_key: Option<SecretValue>,

    /// Where the payload lives on Dashlane's servers. Redacted: it is a
    /// capability to fetch the file, not a description of it.
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
        // The parser's own message is discarded rather than forwarded: a type
        // mismatch would quote the offending value, and that value can be a
        // cryptoKey.
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

//! Move secrets from Dashlane into a Proton Pass vault.
//!
//! A move is two steps: [`plan`] reads both stores and works out what would
//! change, and [`apply`] carries a plan out. Planning writes nothing, so it is
//! safe to run against any vault.
//!
//! # What is skipped
//!
//! Proton Pass items are created and never updated, so a move that ran twice
//! would otherwise leave two of everything. Planning lists the target vault and
//! drops any item whose title and kind already match a live item there.
//!
//! Items whose state `pass-cli` reports in terms this version does not know are
//! neither created nor counted as present — they land in [`Plan::unclassified`]
//! to be resolved by hand, because guessing either way is wrong in a way the
//! user would not see.

use crate::dashlane::{Login, Note, Secret};
use crate::proton::{self, Item, Kind, NewLogin, NewNote, State, Vault};
use crate::{dashlane, Error, Result};

/// An item a move would create in the sink.
#[derive(Debug)]
pub enum Planned {
    /// A login to create.
    Login(NewLogin),
    /// A secure note to create. Dashlane secrets land here too.
    Note(NewNote),
}

impl Planned {
    /// The title the item would be created with.
    #[must_use]
    pub fn title(&self) -> &str {
        match self {
            Self::Login(login) => &login.title,
            Self::Note(note) => &note.title,
        }
    }

    /// The Proton Pass kind it would be created as.
    #[must_use]
    pub fn kind(&self) -> Kind {
        match self {
            Self::Login(_) => Kind::Login,
            Self::Note(_) => Kind::Note,
        }
    }
}

/// An item already in the sink that a planned item matched.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Existing {
    /// The item's title in the sink.
    pub title: String,

    /// The kind it is stored as.
    pub kind: Kind,
}

/// What a move would do, worked out without writing anything.
#[derive(Debug, Default)]
pub struct Plan {
    /// Items that would be created.
    pub creates: Vec<Planned>,

    /// Items already live in the target vault under the same title and kind.
    pub skipped: Vec<Existing>,

    /// Items matching something in the vault whose state this version cannot
    /// read. Not created, and not counted as present.
    pub unclassified: Vec<Existing>,

    /// Titles of Dashlane logins carrying note text, which cannot travel:
    /// `pass-cli`'s login template has no field for it and discards one silently.
    /// Titles only — the note body is a secret and is never recorded here.
    pub notes_dropped: Vec<String>,
}

impl Plan {
    /// How many logins would be created.
    #[must_use]
    pub fn logins(&self) -> usize {
        self.creates
            .iter()
            .filter(|item| matches!(item, Planned::Login(_)))
            .count()
    }

    /// How many notes would be created. Dashlane secrets land here too.
    #[must_use]
    pub fn notes(&self) -> usize {
        self.creates
            .iter()
            .filter(|item| matches!(item, Planned::Note(_)))
            .count()
    }

    /// Whether carrying this plan out would change anything.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.creates.is_empty()
    }
}

/// What carrying a plan out actually did.
#[derive(Debug, Default)]
pub struct Report {
    /// How many items were created.
    pub created: usize,

    /// Items that could not be created, by title. A failure does not stop the
    /// move: the rest still travel, and a later run picks up what did not.
    pub failures: Vec<Failure>,
}

/// One item that could not be created.
#[derive(Debug)]
pub struct Failure {
    /// The title the item would have been created with.
    pub title: String,

    /// Why it could not be created.
    pub error: Error,
}

/// Work out what moving Dashlane's vault into `vault` would do.
///
/// Reads every Dashlane item and lists `vault`; writes nothing.
///
/// # Errors
///
/// As [`dashlane::logins`] and [`proton::items`] — either store being
/// unauthenticated, absent or unreadable fails the plan.
pub fn plan(vault: &Vault) -> Result<Plan> {
    let existing = proton::items(vault)?;
    Ok(plan_from(
        &dashlane::logins()?,
        &dashlane::notes()?,
        &dashlane::secrets()?,
        &existing,
    ))
}

/// The decision half of [`plan`], with both stores already read.
///
/// Separated so it can be tested without either store's CLI on the machine.
fn plan_from(logins: &[Login], notes: &[Note], secrets: &[Secret], existing: &[Item]) -> Plan {
    let mut plan = Plan::default();

    for login in logins {
        if login.note.is_some() {
            plan.notes_dropped.push(title_of(login));
        }
        place(&mut plan, Planned::Login(as_proton_login(login)), existing);
    }
    for note in notes {
        place(&mut plan, Planned::Note(as_proton_note(note)), existing);
    }
    for secret in secrets {
        place(
            &mut plan,
            Planned::Note(secret_as_proton_note(secret)),
            existing,
        );
    }

    plan
}

/// Carry `plan` out, creating its items in `vault`.
///
/// A failure is recorded and the move continues: partial progress is safe
/// because a later [`plan`] sees what already landed and does not repeat it.
///
/// # Errors
///
/// Never — per-item failures are collected into [`Report::failures`]. The
/// signature returns [`Result`] so a future sink that can fail wholesale may.
pub fn apply(vault: &Vault, plan: Plan) -> Result<Report> {
    let mut report = Report::default();

    for item in plan.creates {
        let title = item.title().to_owned();
        let outcome = match &item {
            Planned::Login(login) => proton::create_login(vault, login),
            Planned::Note(note) => proton::create_note(vault, note),
        };
        match outcome {
            Ok(()) => report.created += 1,
            Err(error) => report.failures.push(Failure { title, error }),
        }
    }

    Ok(report)
}

/// Sort one planned item into creates, skips, or the unclassified pile.
fn place(plan: &mut Plan, item: Planned, existing: &[Item]) {
    // An item of a kind stevedore never writes cannot be one stevedore wrote.
    let matches = |other: &&Item| other.title == item.title() && other.item_type == item.kind();

    if existing
        .iter()
        .filter(matches)
        .any(|other| other.state == State::Active)
    {
        plan.skipped.push(Existing {
            title: item.title().to_owned(),
            kind: item.kind(),
        });
        return;
    }

    if existing
        .iter()
        .filter(matches)
        .any(|other| other.state == State::Unknown)
    {
        plan.unclassified.push(Existing {
            title: item.title().to_owned(),
            kind: item.kind(),
        });
        return;
    }

    plan.creates.push(item);
}

fn as_proton_login(login: &Login) -> NewLogin {
    NewLogin {
        title: title_of(login),
        username: login.login.clone(),
        email: login.email.clone(),
        password: login.password.clone(),
        totp_uri: login.otp_url.clone(),
        urls: login.url.iter().cloned().collect(),
    }
}

fn as_proton_note(note: &Note) -> NewNote {
    NewNote {
        title: note
            .title
            .clone()
            .filter(|title| !title.is_empty())
            .unwrap_or_else(|| note.bare_id().to_owned()),
        note: note.content.clone(),
    }
}

/// A Dashlane secret becomes a Proton note: login and note are the only kinds
/// `pass-cli` creates from stdin, and a secret is not a login.
fn secret_as_proton_note(secret: &Secret) -> NewNote {
    NewNote {
        title: secret
            .title
            .clone()
            .filter(|title| !title.is_empty())
            .unwrap_or_else(|| secret.bare_id().to_owned()),
        note: secret.content.clone(),
    }
}

/// Proton requires a title; Dashlane does not. Fall back to the item's own id so
/// an untitled item still travels and stays identifiable.
fn title_of(login: &Login) -> String {
    login
        .title
        .clone()
        .filter(|title| !title.is_empty())
        .unwrap_or_else(|| login.bare_id().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SecretValue;

    fn login(title: &str) -> Login {
        serde_json::from_str(&format!(
            r#"{{"id":"{{ID-1}}","title":"{title}","login":"otto",
                 "email":"otto@example.test","password":"fabricated",
                 "url":"https://example.test"}}"#
        ))
        .unwrap()
    }

    fn item(title: &str, kind: &str, state: &str) -> Item {
        serde_json::from_str(&format!(
            r#"{{"id":"I","share_id":"S","vault_id":"V","title":"{title}",
                 "item_type":"{kind}","state":"{state}"}}"#
        ))
        .unwrap()
    }

    #[test]
    fn a_login_maps_onto_the_proton_template() {
        let mapped = as_proton_login(&login("GitHub"));
        assert_eq!(mapped.title, "GitHub");
        assert_eq!(mapped.username.as_deref(), Some("otto"));
        assert_eq!(mapped.email.as_deref(), Some("otto@example.test"));
        assert_eq!(mapped.urls, vec!["https://example.test".to_owned()]);
        assert!(mapped.password.is_some());
    }

    #[test]
    fn an_untitled_item_falls_back_to_its_id() {
        let untitled: Login = serde_json::from_str(r#"{"id":"{ID-1}"}"#).unwrap();
        assert_eq!(as_proton_login(&untitled).title, "ID-1");
    }

    #[test]
    fn an_empty_title_is_treated_as_no_title() {
        let untitled: Login = serde_json::from_str(r#"{"id":"{ID-1}","title":""}"#).unwrap();
        assert_eq!(as_proton_login(&untitled).title, "ID-1");
    }

    #[test]
    fn a_live_item_of_the_same_title_and_kind_is_skipped() {
        let mut plan = Plan::default();
        let existing = vec![item("GitHub", "login", "Active")];
        place(
            &mut plan,
            Planned::Login(as_proton_login(&login("GitHub"))),
            &existing,
        );
        assert!(plan.creates.is_empty());
        assert_eq!(plan.skipped.len(), 1);
        assert_eq!(plan.skipped[0].kind, Kind::Login);
    }

    #[test]
    fn a_trashed_item_does_not_count_as_present() {
        let mut plan = Plan::default();
        let existing = vec![item("GitHub", "login", "Trashed")];
        place(
            &mut plan,
            Planned::Login(as_proton_login(&login("GitHub"))),
            &existing,
        );
        assert_eq!(plan.creates.len(), 1, "a trashed item is not a live one");
        assert!(plan.skipped.is_empty());
    }

    #[test]
    fn a_matching_title_of_another_kind_does_not_count() {
        let mut plan = Plan::default();
        let existing = vec![item("GitHub", "note", "Active")];
        place(
            &mut plan,
            Planned::Login(as_proton_login(&login("GitHub"))),
            &existing,
        );
        assert_eq!(plan.creates.len(), 1);
    }

    /// stevedore writes only logins and notes, so an item of a kind it does not
    /// model cannot be one it wrote — it must not suppress a create.
    #[test]
    fn an_unknown_kind_never_matches() {
        let mut plan = Plan::default();
        let existing = vec![item("GitHub", "something-new", "Active")];
        place(
            &mut plan,
            Planned::Login(as_proton_login(&login("GitHub"))),
            &existing,
        );
        assert_eq!(plan.creates.len(), 1);
        assert!(plan.unclassified.is_empty());
    }

    #[test]
    fn an_unknown_state_is_neither_created_nor_skipped() {
        let mut plan = Plan::default();
        let existing = vec![item("GitHub", "login", "Archived")];
        place(
            &mut plan,
            Planned::Login(as_proton_login(&login("GitHub"))),
            &existing,
        );
        assert!(plan.creates.is_empty(), "guessing it is absent duplicates");
        assert!(plan.skipped.is_empty(), "guessing it is present loses data");
        assert_eq!(plan.unclassified.len(), 1);
    }

    /// A live item settles the question even when an unreadable one shares its
    /// title, so an odd neighbour cannot strand an item that is plainly present.
    #[test]
    fn a_live_match_wins_over_an_unreadable_one() {
        let mut plan = Plan::default();
        let existing = vec![
            item("GitHub", "login", "Archived"),
            item("GitHub", "login", "Active"),
        ];
        place(
            &mut plan,
            Planned::Login(as_proton_login(&login("GitHub"))),
            &existing,
        );
        assert_eq!(plan.skipped.len(), 1);
        assert!(plan.unclassified.is_empty());
    }

    #[test]
    fn the_plan_counts_logins_and_notes_apart() {
        let mut plan = Plan::default();
        plan.creates
            .push(Planned::Login(as_proton_login(&login("a"))));
        plan.creates.push(Planned::Note(NewNote::new("b")));
        plan.creates.push(Planned::Note(NewNote::new("c")));
        assert_eq!(plan.logins(), 1);
        assert_eq!(plan.notes(), 2);
        assert!(!plan.is_empty());
    }

    #[test]
    fn a_whole_plan_sorts_every_kind_of_item() {
        let note: Note = serde_json::from_str(r#"{"id":"{N1}","title":"Wifi","content":"psk"}"#)
            .expect("the note fixture should parse");
        let secret: Secret =
            serde_json::from_str(r#"{"id":"{S1}","title":"Deploy key","content":"kkk"}"#)
                .expect("the secret fixture should parse");
        let existing = vec![item("Already there", "login", "Active")];

        let plan = plan_from(
            &[login("GitHub"), login("Already there")],
            &[note],
            &[secret],
            &existing,
        );

        assert_eq!(plan.logins(), 1, "the present login should not be remade");
        assert_eq!(plan.notes(), 2, "a Dashlane secret becomes a Proton note");
        assert_eq!(plan.skipped.len(), 1);
        assert!(plan.unclassified.is_empty());
        assert!(
            plan.notes_dropped.is_empty(),
            "no fixture carries note text"
        );
    }

    /// A login already in the vault still reports its note text as left behind:
    /// skipping the create does not mean the text arrived on an earlier run.
    #[test]
    fn a_skipped_login_still_reports_its_dropped_note() {
        let mut carries_note = login("Already there");
        carries_note.note = Some(SecretValue::new("private text"));
        let existing = vec![item("Already there", "login", "Active")];

        let plan = plan_from(&[carries_note], &[], &[], &existing);

        assert!(plan.creates.is_empty());
        assert_eq!(plan.skipped.len(), 1);
        assert_eq!(plan.notes_dropped, vec!["Already there".to_owned()]);
        assert!(!format!("{plan:?}").contains("private text"));
    }

    #[test]
    fn a_dropped_note_records_a_title_and_never_the_body() {
        let mut with_note = login("GitHub");
        with_note.note = Some(SecretValue::new("private text"));
        let mut plan = Plan::default();
        if with_note.note.is_some() {
            plan.notes_dropped.push(title_of(&with_note));
        }
        assert_eq!(plan.notes_dropped, vec!["GitHub".to_owned()]);
        assert!(!format!("{plan:?}").contains("private text"));
    }
}

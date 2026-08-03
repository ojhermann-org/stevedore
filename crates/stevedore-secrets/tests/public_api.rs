//! What the library looks like from outside.
//!
//! Compiled as a separate crate, so it reaches only the public API. That is the
//! whole point: some breaking changes are **inert inside the defining crate**
//! and no unit test can see them. Adding `#[non_exhaustive]` to a public struct
//! is the clearest case — it blocks downstream literal construction while every
//! test in `src/` keeps passing.
//!
//! Two adjacent failures are already covered elsewhere, so this does not claim
//! them: a dropped `pub use` trips the denied `unreachable_pub` at build time,
//! and a field that stops being public breaks the unit tests that set it.
//!
//! Nothing here touches a store, so unlike the `*_live` suites it runs in CI.

use std::io::{Error as IoError, ErrorKind};

use stevedore_secrets::{
    CliError, Error, Result, SecretValue, dashlane, mover,
    mover::{Existing, Plan, Planned},
    proton,
    proton::{Item, Kind, NewLogin, NewNote, State, Vault},
};

/// Every path a consumer can name, named. Dropping a `pub use` from `lib.rs`
/// breaks this before it reaches anyone.
#[test]
fn the_public_surface_is_reachable() {
    assert_eq!(dashlane::NAME, "dashlane");
    assert_eq!(proton::NAME, "proton-pass");
    assert_eq!(dashlane::NO_CATEGORY, "noCategory");

    // Named so the compiler checks they are still exported and still public.
    let _: fn() -> Result<Vec<dashlane::Login>> = dashlane::logins;
    let _: fn() -> Result<Vec<dashlane::Note>> = dashlane::notes;
    let _: fn() -> Result<Vec<dashlane::Secret>> = dashlane::secrets;
    let _: fn() -> Result<dashlane::Status> = dashlane::status;
    let _: fn(&Vault) -> Result<Vec<Item>> = proton::items;
    let _: fn(&str) -> Result<Vault> = proton::vault;
    let _: fn(&Vault, &NewLogin) -> Result<()> = proton::create_login;
    let _: fn(&Vault, &NewNote) -> Result<()> = proton::create_note;
    let _: fn(&Vault) -> Result<Plan> = mover::plan;
    let _: fn(&Vault, Plan) -> Result<mover::Report> = mover::apply;
}

#[test]
fn a_secret_redacts_itself_through_the_public_api() {
    let secret = SecretValue::new("hunter2");

    assert_eq!(format!("{secret}"), "<redacted>");
    assert_eq!(format!("{secret:?}"), "SecretValue(<redacted>)");
    assert_eq!(
        secret.expose(),
        "hunter2",
        "expose is the deliberate way out"
    );
}

/// The redaction has to survive being nested in an item, because that is how a
/// consumer will actually hold it.
#[test]
fn an_item_holding_a_secret_does_not_leak_it_in_debug() {
    let mut login = NewLogin::new("GitHub");
    login.password = Some(SecretValue::new("hunter2"));
    login.totp_uri = Some(SecretValue::new("otpauth://totp/?secret=jbswy3dpehpk3pxp"));

    let shown = format!("{login:?}");
    assert!(!shown.contains("hunter2"), "leaked password: {shown}");
    assert!(!shown.contains("jbswy3dpehpk3pxp"), "leaked seed: {shown}");
    assert!(shown.contains("GitHub"), "metadata should still show");

    let mut note = NewNote::new("Wifi");
    note.note = Some(SecretValue::new("the psk"));
    assert!(!format!("{note:?}").contains("the psk"));
}

/// Every message is a fragment a reporter joins with `: `, so a capital or a
/// trailing period would read as a false sentence boundary.
#[test]
fn error_messages_are_lowercase_fragments() {
    let errors = [
        Error::NotAuthenticated,
        Error::Locked,
        Error::Unserializable,
        Error::Unparsable {
            what: "logins",
            line: 3,
            column: 9,
        },
        Error::NoSuchVault {
            name: "Personal".to_owned(),
        },
        Error::Cli(CliError::NotFound { program: "dcli" }),
        Error::Io {
            doing: "spawning",
            program: "dcli",
            source: IoError::from(ErrorKind::PermissionDenied),
        },
    ];

    for error in &errors {
        let message = error.to_string();
        assert!(!message.is_empty(), "an error must say something");
        assert!(
            !message.ends_with('.'),
            "a fragment carries no trailing period: {message}"
        );
        assert!(
            message.chars().next().is_some_and(|c| !c.is_uppercase()),
            "a fragment does not start with a capital: {message}"
        );
    }
}

/// The layer contributes only what it uniquely knows; the operating system's
/// wording stays in the source, reachable but never restated.
#[test]
fn an_io_error_does_not_restate_its_source() {
    let source = IoError::from(ErrorKind::PermissionDenied);
    let os_wording = source.to_string();
    let error = Error::Io {
        doing: "spawning",
        program: "dcli",
        source,
    };

    assert_eq!(error.to_string(), "spawning `dcli`");
    assert!(
        !error.to_string().contains(&os_wording),
        "the message must not repeat what the source already says"
    );
    assert!(
        std::error::Error::source(&error).is_some(),
        "the source must stay reachable for a reporter to join"
    );
}

/// A secret must never reach an error message, whatever the variant.
#[test]
fn a_vault_name_is_the_only_runtime_string_an_error_carries() {
    let error = Error::NoSuchVault {
        name: "Personal".to_owned(),
    };
    assert!(error.to_string().contains("Personal"), "names are metadata");
}

#[test]
fn a_plan_counts_logins_and_notes_apart() {
    let mut plan = Plan::default();
    assert!(plan.is_empty(), "a fresh plan would change nothing");

    plan.creates.push(Planned::Login(NewLogin::new("GitHub")));
    plan.creates.push(Planned::Note(NewNote::new("Wifi")));
    plan.skipped.push(Existing {
        title: "Already there".to_owned(),
        kind: Kind::Login,
    });

    assert_eq!(plan.logins(), 1);
    assert_eq!(plan.notes(), 1);
    assert_eq!(plan.skipped.len(), 1);
    assert!(!plan.is_empty());

    let planned = &plan.creates[0];
    assert_eq!(planned.title(), "GitHub");
    assert_eq!(planned.kind(), Kind::Login);
}

/// `Item`'s `Deserialize` is public surface too: a consumer can parse a listing
/// `pass-cli` produced without going through this crate's reader.
#[test]
fn a_listed_item_parses_from_the_outside() {
    let item: Item = serde_json::from_str(
        r#"{"id":"I","share_id":"S","vault_id":"V","title":"GitHub",
            "item_type":"login","state":"Active"}"#,
    )
    .expect("a pass-cli item should parse");

    assert_eq!(item.title, "GitHub");
    assert_eq!(item.item_type, Kind::Login);
    assert_eq!(item.state, State::Active);
    assert!(item.is_active());
}

/// A kind or state this version does not model must land on `Unknown` rather
/// than failing the parse, so one new Proton feature cannot break a whole move.
#[test]
fn an_unmodelled_kind_or_state_becomes_unknown() {
    let item: Item = serde_json::from_str(
        r#"{"id":"I","share_id":"S","vault_id":"V","title":"Odd",
            "item_type":"something-new","state":"Archived"}"#,
    )
    .expect("an unknown kind should still parse");

    assert_eq!(item.item_type, Kind::Unknown);
    assert_eq!(item.state, State::Unknown);
    assert!(!item.is_active(), "an unreadable state is not active");
}

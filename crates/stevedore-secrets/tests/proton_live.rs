//! Smoke tests against a real Proton Pass account.
//!
//! Ignored by default: they need `pass-cli` installed and logged in, so they
//! cannot run in CI. Run them deliberately when touching the Proton Pass writer:
//!
//! ```console
//! cargo test -p stevedore-secrets --test proton_live -- --ignored --nocapture
//! ```
//!
//! The write test **creates items in a real vault**, so it additionally needs
//! `STEVEDORE_PROTON_TEST_VAULT` to name the vault to write to. Point it at a
//! vault you are happy to have written to; every item it creates is moved to the
//! trash before it returns.
//!
//! **No secret value is ever printed.** The items these tests create hold
//! fabricated secrets, and even those are checked by presence, never echoed.

use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use stevedore_secrets::proton::{self, NewLogin, NewNote};
use stevedore_secrets::SecretValue;

const VAULT_ENV: &str = "STEVEDORE_PROTON_TEST_VAULT";

#[test]
#[ignore = "needs a logged-in pass-cli"]
fn reads_the_session_and_the_vault_list() {
    let session = proton::session().expect("pass-cli should report a session");
    println!(
        "session: email={} release_track={}",
        session.email.is_some(),
        session.release_track.as_deref().unwrap_or("unknown"),
    );

    let vaults = proton::vaults().expect("vaults should parse");
    println!("vaults={}", vaults.len());
    assert!(!vaults.is_empty(), "an account always has at least a vault");
    for vault in &vaults {
        assert!(!vault.share_id.is_empty(), "every vault carries a share id");
        assert!(!vault.vault_id.is_empty(), "every vault carries an id");
    }

    let missing = proton::vault("stevedore-no-such-vault").unwrap_err();
    assert!(
        format!("{missing}").contains("stevedore-no-such-vault"),
        "a missing vault should be named: {missing}"
    );
}

#[test]
#[ignore = "creates items in a real vault; needs STEVEDORE_PROTON_TEST_VAULT"]
fn creates_a_login_and_a_note() {
    let Ok(name) = std::env::var(VAULT_ENV) else {
        panic!("set {VAULT_ENV} to the vault these items may be created in");
    };
    let vault = proton::vault(&name).expect("the named vault should exist");
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("the clock should be past 1970")
        .as_nanos();

    let login_title = format!("stevedore live login {stamp}");
    let mut login = NewLogin::new(&login_title);
    login.username = Some("otto".to_owned());
    login.password = Some(SecretValue::new("fabricated-not-a-real-password"));
    login.totp_uri = Some(SecretValue::new(
        "otpauth://totp/stevedore?secret=jbswy3dpehpk3pxp",
    ));
    login.urls = vec!["https://example.test".to_owned()];

    let note_title = format!("stevedore live note {stamp}");
    let mut note = NewNote::new(&note_title);
    note.note = Some(SecretValue::new("fabricated note body"));

    proton::create_login(&vault, &login).expect("the login should be created");
    proton::create_note(&vault, &note).expect("the note should be created");

    let listed = titles(&vault.share_id);
    let found = [&login_title, &note_title].map(|title| listed.contains(title));

    for title in [&login_title, &note_title] {
        trash(&vault.share_id, title);
    }

    assert!(found[0], "the created login should be listed in `{name}`");
    assert!(found[1], "the created note should be listed in `{name}`");
    println!("created and trashed 2 items in `{name}`");
}

/// Item titles in a vault. Titles are metadata; secrets are not requested.
fn titles(share_id: &str) -> String {
    let out = Command::new("pass-cli")
        .args(["item", "list", "--share-id", share_id, "--output", "json"])
        .output()
        .expect("pass-cli should run");
    assert!(out.status.success(), "`pass-cli item list` should succeed");
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn trash(share_id: &str, title: &str) {
    let out = Command::new("pass-cli")
        .args([
            "item",
            "trash",
            "--share-id",
            share_id,
            "--item-title",
            title,
        ])
        .output()
        .expect("pass-cli should run");
    assert!(
        out.status.success(),
        "failed to clean up `{title}` — trash it by hand"
    );
}

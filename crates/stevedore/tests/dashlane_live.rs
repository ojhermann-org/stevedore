//! Smoke tests against a real Dashlane vault.
//!
//! Ignored by default: they need `dcli` installed, authenticated and unlocked,
//! so they cannot run in CI. Run them deliberately when touching the Dashlane
//! reader:
//!
//! ```console
//! cargo test -p stevedore --test dashlane_live -- --ignored --nocapture
//! ```
//!
//! The unit tests prove the types parse *fabricated* fixtures. These prove they
//! survive a real vault — which is a different claim, and the one that caught
//! nothing only because the shapes were probed first.
//!
//! **No secret value is ever printed.** Counts and presence flags only. The
//! redaction check compares against live secrets without ever emitting one.

use stevedore::dashlane;

#[test]
#[ignore = "needs an authenticated dcli and a real vault"]
fn reads_a_real_vault_without_leaking_it() {
    let status = dashlane::status().expect("dcli status should run");
    assert!(
        status.logged_in,
        "log in with `dcli sync` before running this"
    );
    assert!(!status.locked, "unlock the vault before running this");

    let logins = dashlane::logins().expect("logins should parse");
    let notes = dashlane::notes().expect("notes should parse");

    println!(
        "logins={} (password={} otp={} note={} title={} url={})",
        logins.len(),
        logins.iter().filter(|l| l.password.is_some()).count(),
        logins.iter().filter(|l| l.otp_url.is_some()).count(),
        logins.iter().filter(|l| l.note.is_some()).count(),
        logins.iter().filter(|l| l.title.is_some()).count(),
        logins.iter().filter(|l| l.url.is_some()).count(),
    );
    println!(
        "notes={} (secured={} ungrouped={} attachments={})",
        notes.len(),
        notes.iter().filter(|n| n.is_secured()).count(),
        notes.iter().filter(|n| n.is_ungrouped()).count(),
        notes.iter().map(|n| n.attachments.len()).sum::<usize>(),
    );

    for login in &logins {
        assert!(!login.id.is_empty(), "every login carries an id");
        assert!(
            !login.bare_id().contains(['{', '}']),
            "bare_id should strip the braces a dl:// path rejects"
        );
    }
    for note in &notes {
        assert!(!note.id.is_empty(), "every note carries an id");
    }

    // Redaction, checked against real secrets rather than fixtures. The failure
    // messages name the record's id, never the value that leaked.
    for login in &logins {
        let shown = format!("{login:?}");
        for (field, secret) in [
            ("password", login.password.as_ref()),
            ("otp_url", login.otp_url.as_ref()),
            ("note", login.note.as_ref()),
        ] {
            if let Some(secret) = secret {
                assert!(
                    !shown.contains(secret.expose()),
                    "login {} leaked its {field} through Debug",
                    login.id
                );
            }
        }
    }
    for note in &notes {
        let shown = format!("{note:?}");
        if let Some(content) = note.content.as_ref() {
            assert!(
                !shown.contains(content.expose()),
                "note {} leaked its content through Debug",
                note.id
            );
        }
        for attachment in &note.attachments {
            if let Some(key) = attachment.crypto_key.as_ref() {
                assert!(
                    !shown.contains(key.expose()),
                    "note {} leaked an attachment crypto key through Debug",
                    note.id
                );
            }
        }
    }
}

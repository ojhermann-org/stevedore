//! Smoke test for a move, against a real Dashlane vault and a real Proton vault.
//!
//! Ignored by default: it needs both `dcli` and `pass-cli` installed and logged
//! in, so it cannot run in CI.
//!
//! ```console
//! cargo test -p stevedore-secrets --test move_live -- --ignored --nocapture
//! ```
//!
//! **It plans only.** Planning writes nothing, so this never creates an item;
//! carrying a plan out against a real vault is a deliberate act for a person, not
//! something a test does. What it checks is that the two stores meet: that every
//! Dashlane item maps onto something Proton Pass will take, and that the vault
//! listing is understood well enough to decide each one.
//!
//! **No secret value is printed.** Only counts leave this test.

use stevedore_secrets::{mover, proton};

const VAULT_ENV: &str = "STEVEDORE_PROTON_TEST_VAULT";

#[test]
#[ignore = "needs a logged-in dcli and pass-cli"]
fn plans_a_move_without_writing_anything() {
    let Ok(name) = std::env::var(VAULT_ENV) else {
        panic!("set {VAULT_ENV} to the vault to plan against");
    };
    let vault = proton::vault(&name).expect("the named vault should exist");

    let before = proton::items(&vault).expect("the listing should parse");
    let plan = mover::plan(&vault).expect("a plan should be reachable");
    let after = proton::items(&vault).expect("the listing should parse");

    assert_eq!(
        before.len(),
        after.len(),
        "planning must not create or remove anything"
    );

    // Every planned item carries a title: Proton rejects one without.
    for item in &plan.creates {
        assert!(
            !item.title().is_empty(),
            "an item was planned with an empty title"
        );
    }

    println!(
        "plan: {} logins, {} notes, {} skipped, {} unclassified, {} logins with note text left behind",
        plan.logins(),
        plan.notes(),
        plan.skipped.len(),
        plan.unclassified.len(),
        plan.notes_dropped.len(),
    );
}

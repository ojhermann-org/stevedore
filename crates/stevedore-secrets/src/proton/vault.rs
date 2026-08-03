//! A Proton Pass vault, as `pass-cli vault list --output json` reports it.

use serde::Deserialize;

/// One vault in the account — the container an item is created in.
///
/// Names are chosen by the user and need not be unique, so writes address a
/// vault by [`share_id`], not by name.
///
/// [`share_id`]: Vault::share_id
#[derive(Debug, Clone, Deserialize)]
pub struct Vault {
    /// The vault's name, as shown in Proton Pass. Chosen by the user and not
    /// necessarily unique.
    pub name: String,

    /// Identifies the vault itself.
    pub vault_id: String,

    /// Identifies the caller's share of the vault. This is what `pass-cli`
    /// commands take as `--share-id`.
    pub share_id: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct VaultList {
    pub vaults: Vec<Vault>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIST: &str = r#"{
        "vaults": [
            {"name": "Personal", "vault_id": "V1==", "share_id": "S1=="},
            {"name": "Burners", "vault_id": "V2==", "share_id": "S2=="}
        ]
    }"#;

    #[test]
    fn parses_the_vault_list() {
        let list: VaultList = serde_json::from_str(LIST).unwrap();
        assert_eq!(list.vaults.len(), 2);
        assert_eq!(list.vaults[0].name, "Personal");
        assert_eq!(list.vaults[1].share_id, "S2==");
    }

    #[test]
    fn an_empty_account_parses() {
        let list: VaultList = serde_json::from_str(r#"{"vaults":[]}"#).unwrap();
        assert!(list.vaults.is_empty());
    }
}

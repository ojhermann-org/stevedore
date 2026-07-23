//! Planning a move.
//!
//! `stevedore` is a one-shot mover, so the safe default is to **plan** — read
//! the source, show what would move — and let a human confirm before anything is
//! written. [`Plan`] is that intermediate; building it touches only the source.

use crate::secret::SecretRecord;

/// What a migration would move, computed without touching the destination.
#[derive(Debug, Default)]
pub struct Plan {
    pub records: Vec<SecretRecord>,
}

impl Plan {
    pub fn from_records(records: Vec<SecretRecord>) -> Self {
        Self { records }
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::secret::SecretValue;

    #[test]
    fn plan_counts_records() {
        let plan = Plan::from_records(vec![
            SecretRecord::new("a", SecretValue::new("1")),
            SecretRecord::new("b", SecretValue::new("2")),
        ]);
        assert_eq!(plan.len(), 2);
        assert!(!plan.is_empty());
    }

    #[test]
    fn empty_plan_is_empty() {
        assert!(Plan::default().is_empty());
    }
}

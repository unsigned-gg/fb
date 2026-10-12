//! Zero-I/O domain models for `fb`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FbState {
    pub version: u32,
    pub id: String,
}

impl Default for FbState {
    fn default() -> Self {
        Self {
            version: 1,
            id: String::from("initial"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state() {
        let state = FbState::default();
        assert_eq!(state.version, 1);
        assert_eq!(state.id, "initial");
    }
}

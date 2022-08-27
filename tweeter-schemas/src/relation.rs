use serde::{Deserialize, Serialize};

use super::key::Key;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Relation {
    pub data: Key,
}

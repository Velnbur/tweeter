use serde::{Serialize, Deserialize};

use super::key::Key;

#[derive(Debug, Serialize, Deserialize)]
pub struct Relation {
    pub data: Key,
}
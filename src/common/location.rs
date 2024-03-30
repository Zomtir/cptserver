use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Location {
    pub id: u32,
    pub key: String,
    pub name: String,
    pub description: String,
}

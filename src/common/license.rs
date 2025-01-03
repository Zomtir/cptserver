use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct License {
    pub id: u32,
    pub number: String,
    pub name: String,
    pub expiration: chrono::NaiveDate,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct License {
    pub id: u32,
    pub number: u32,
    pub name: String,
    pub expiration: chrono::NaiveDate,
}
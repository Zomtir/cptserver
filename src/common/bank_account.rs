use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct BankAccount {
    pub id: u32,
    pub iban: String,
    pub bic: String,
    pub institute: String,
}

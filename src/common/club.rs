use crate::common::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Club {
    pub id: u64,
    pub key: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Term {
    pub id: u64,
    pub user: User,
    pub club: Club,
    pub begin: Option<chrono::NaiveDate>,
    pub end: Option<chrono::NaiveDate>,
}

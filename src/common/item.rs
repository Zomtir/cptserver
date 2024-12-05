use crate::common::{Club, User};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub category: Option<ItemCategory>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemCategory {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stock {
    pub id: u64,
    pub club: Club,
    pub item: Item,
    pub storage: String,
    pub owned: u32,
    pub loaned: u32,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Possession {
    pub id: u32,
    pub user: User,
    pub item: Item,
    pub acquisition_date: chrono::NaiveDate,
    pub owned: bool,
}

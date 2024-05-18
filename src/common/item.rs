use crate::common::{Club, User};
use mysql_common::chrono::NaiveDate;
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
    pub club: Club,
    pub item: Item,
    pub owned: u32,
    pub loaned: u32,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Possession {
    pub id: u32,
    pub user: User,
    pub item: Item,
    pub owned: bool,
    pub club: Option<Club>,
    pub transfer_date: Option<NaiveDate>,
}

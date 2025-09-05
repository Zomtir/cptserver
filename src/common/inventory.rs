use crate::common::{Club, Skill, User};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub category: Option<ItemCategory>,
}

impl Item {
    pub fn from_row(
        item_id: Option<u64>,
        item_name: Option<String>,
        category_id: Option<u64>,
        category_name: Option<String>,
    ) -> Option<Self> {
        Some(Self {
            id: item_id?,
            name: item_name?,
            category: ItemCategory::from_row(category_id, category_name),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemCategory {
    pub id: u64,
    pub name: String,
}

impl ItemCategory {
    pub fn from_row(category_id: Option<u64>, category_name: Option<String>) -> Option<Self> {
        Some(Self {
            id: category_id?,
            name: category_name?,
        })
    }
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

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Equipment {
    pub id: u64,
    pub user: User,
    pub skill: Skill,
    pub item: Item,
    pub count: u32,
}

use crate::common::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Organisation {
    pub id: u64,
    pub abbreviation: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Affiliation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organisation: Option<Organisation>,
    pub member_identifier: Option<String>,
    pub permission_solo_date: Option<chrono::NaiveDate>,
    pub permission_team_date: Option<chrono::NaiveDate>,
    pub residency_move_date: Option<chrono::NaiveDate>,
}

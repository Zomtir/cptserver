use crate::common::{Organisation, User};
use serde::{Deserialize, Serialize};

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

impl Affiliation {
    pub fn from_row(row: &mut mysql::Row) -> Affiliation {
        Affiliation {
            user: User::from_row(row),
            organisation: Organisation::from_row(row),
            member_identifier: row.take("member_identifier").unwrap(),
            permission_solo_date: row.take("permission_solo_date").unwrap(),
            permission_team_date: row.take("permission_team_date").unwrap(),
            residency_move_date: row.take("residency_move_date").unwrap(),
        }
    }
}

use crate::common::{Discipline, User};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Club {
    pub id: u64,
    pub key: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disciplines: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chairman: Option<String>,
}

impl Club {
    pub fn from_info(id: u64, key: String, name: String) -> Club {
        Club {
            id,
            key,
            name,
            description: None,
            disciplines: None,
            image_url: None,
            banner_url: None,
            chairman: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Term {
    pub id: u64,
    pub user: User,
    pub club: Club,
    pub begin: Option<chrono::NaiveDate>,
    pub end: Option<chrono::NaiveDate>,
    pub disciplines: Option<Vec<TermDiscipline>>,
}

impl Term {
    pub fn from_row(
        term_id: Option<u64>,
        user_id: Option<u64>,
        user_key: Option<String>,
        firstname: Option<String>,
        lastname: Option<String>,
        nickname: Option<Option<String>>,
        club_id: Option<u64>,
        club_key: Option<String>,
        club_name: Option<String>,
        begin: Option<Option<chrono::NaiveDate>>,
        end: Option<Option<chrono::NaiveDate>>,
    ) -> Option<Self> {
        Some(Self {
            id: term_id?,
            user: User::from_info(user_id?, user_key?, firstname?, lastname?, nickname?),
            club: Club::from_info(club_id?, club_key?, club_name?),
            begin: begin?,
            end: end?,
            disciplines: None,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TermDiscipline {
    pub id: u64,
    pub discipline: Discipline,
    pub begin: Option<u32>,
    pub end: Option<u32>,
}

impl TermDiscipline {
    #[allow(dead_code)]
    pub fn from_row(
        term_discipline_id: Option<u64>,
        discipline_id: Option<u16>,
        discipline_name: Option<String>,
        begin: Option<Option<u32>>,
        end: Option<Option<u32>>,
    ) -> Option<Self> {
        Some(Self {
            id: term_discipline_id?,
            discipline: Discipline {
                id: discipline_id?,
                name: discipline_name?,
            },
            begin: begin?,
            end: end?,
        })
    }
}

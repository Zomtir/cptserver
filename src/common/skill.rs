use crate::common::{Course, User};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Skill {
    pub id: u16,
    pub key: String,
    pub name: String,
    pub min: Option<u8>,
    pub max: Option<u8>,
}

impl Skill {
    pub fn from_row(
        skill_id: Option<u16>,
        skill_key: Option<String>,
        skill_name: Option<String>,
        skill_min: Option<u8>,
        skill_max: Option<u8>,
    ) -> Option<Self> {
        Some(Self {
            id: skill_id?,
            key: skill_key?,
            name: skill_name?,
            min: skill_min,
            max: skill_max,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Requirement {
    pub id: u32,
    pub course: Course,
    pub skill: Skill,
    pub rank: u8,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Competence {
    pub id: u32,
    pub user: User,
    pub skill: Skill,
    pub rank: u8,
    pub date: chrono::NaiveDate,
    pub judge: User,
}

impl Competence {
    pub fn from_row(row: &mut mysql::Row) -> Option<Competence> {
        Some(Competence {
            id: row.take("competence_id")?,
            user: User::from_info(
                row.take("user_id").unwrap(),
                row.take("user_key").unwrap(),
                row.take("user_firstname").unwrap(),
                row.take("user_lastname").unwrap(),
                row.take("user_nickname").unwrap(),
            ),
            skill: Skill::from_row(
                row.take("skill_id"),
                row.take("skill_key"),
                row.take("skill_name"),
                row.take("skill_min"),
                row.take("skill_max"),
            )?,
            rank: row.take("rank").unwrap(),
            date: row.take("date").unwrap(),
            judge: User::from_info(
                row.take("judge_id").unwrap(),
                row.take("judge_key").unwrap(),
                row.take("judge_firstname").unwrap(),
                row.take("judge_lastname").unwrap(),
                row.take("judge_nickname").unwrap(),
            ),
        })
    }
}

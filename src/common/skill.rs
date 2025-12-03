use crate::common::{Course, User};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Skill {
    pub id: u16,
    pub key: String,
    pub title: String,
    pub min: Option<u8>,
    pub max: Option<u8>,
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
    pub fn from_row(row: &mut mysql::Row) -> Competence {
        Competence {
            id: row.take("competence_id").unwrap(),
            user: User::from_info(
                row.take("user_id").unwrap(),
                row.take("user_key").unwrap(),
                row.take("user_firstname").unwrap(),
                row.take("user_lastname").unwrap(),
                row.take("user_nickname").unwrap(),
            ),
            skill: Skill {
                id: row.take("skill_id").unwrap(),
                key: row.take("skill_key").unwrap(),
                title: row.take("skill_title").unwrap(),
                min: row.take("skill_min").unwrap(),
                max: row.take("skill_max").unwrap(),
            },
            rank: row.take("rank").unwrap(),
            date: row.take("date").unwrap(),
            judge: User::from_info(
                row.take("judge_id").unwrap(),
                row.take("judge_key").unwrap(),
                row.take("judge_firstname").unwrap(),
                row.take("judge_lastname").unwrap(),
                row.take("judge_nickname").unwrap(),
            ),
        }
    }
}

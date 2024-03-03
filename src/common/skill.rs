use crate::common::{Course, User};
use mysql_common::chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Skill {
    pub id: u16,
    pub key: String,
    pub title: String,
}

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
    pub date: NaiveDate,
    pub judge: User,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Team {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub right: Option<Right>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Right {
    //pub admin_club: bool,
    pub admin_competence: bool,
    pub admin_courses: bool,
    pub admin_event: bool,
    pub admin_inventory: bool,
    pub admin_teams: bool,
    pub admin_term: bool,
    pub admin_users: bool,
}

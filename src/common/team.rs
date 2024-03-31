use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Team {
    pub id: u32,
    pub key: String,
    pub name: String,
    pub description: String,
    pub right: Option<Right>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Right {
    pub right_club_write: bool,
    pub right_club_read: bool,
    pub right_competence_write: bool,
    pub right_competence_read: bool,
    pub right_course_write: bool,
    pub right_course_read: bool,
    pub right_event_write: bool,
    pub right_event_read: bool,
    pub right_inventory_write: bool,
    pub right_inventory_read: bool,
    pub right_location_write: bool,
    pub right_location_read: bool,
    pub right_team_write: bool,
    pub right_team_read: bool,
    pub right_user_write: bool,
    pub right_user_read: bool,
}

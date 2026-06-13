use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Discipline {
    pub id: u16,
    pub name: String,
}

impl Discipline {
    #[allow(dead_code)]
    pub fn from_row(discipline_id: Option<u16>, discipline_name: Option<String>) -> Option<Self> {
        Some(Self {
            id: discipline_id?,
            name: discipline_name?,
        })
    }
}

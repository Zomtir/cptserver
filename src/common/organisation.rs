use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Organisation {
    pub id: u32,
    pub abbreviation: Option<String>,
    pub name: Option<String>,
}

impl Organisation {
    pub fn from_row(row: &mut mysql::Row) -> Option<Organisation> {
        row.take::<Option<u32>, &str>("organisation_id")
            .unwrap()
            .map(|organisation_id| Organisation {
                id: organisation_id,
                abbreviation: row.take("organisation_abbreviation").unwrap(),
                name: row.take("organisation_name").unwrap(),
            })
    }

    pub fn sql_map() -> impl Fn((u32, Option<String>, Option<String>)) -> Organisation {
        |(organisation_id, abbreviation, name)| Organisation {
            id: organisation_id,
            abbreviation,
            name,
        }
    }
}

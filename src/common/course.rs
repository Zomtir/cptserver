use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Course {
    pub id: u32,
    pub key: String,
    pub title: String,
    pub active: bool,
    pub public: bool,
}

#[allow(dead_code)]
impl Course {
    pub fn from_row(row: &mut mysql::Row) -> Option<Course> {
        row.take::<Option<u32>, &str>("course_id")
            .unwrap()
            .map(|course_id| Course {
                id: course_id,
                key: row.take("course_key").unwrap(),
                title: row.take("course_title").unwrap(),
                active: row.take("course_active").unwrap(),
                public: row.take("course_public").unwrap(),
            })
    }

    pub fn sql_map() -> impl Fn((u32, String, String, bool, bool)) -> Course {
        |(course_id, key, title, active, public)| Course {
            id: course_id,
            key,
            title,
            active,
            public,
        }
    }
}

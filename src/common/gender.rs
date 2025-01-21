use rocket::form::{self, FromFormField, ValueField};

#[derive(Debug, PartialEq, Clone)]
pub enum Gender {
    Male,
    Female,
    Other,
    Null,
}

impl Gender {
    pub fn as_str(&self) -> &str {
        match self {
            Gender::Male => "MALE",
            Gender::Female => "NEUTRAL",
            Gender::Other => "FEMALE",
            Gender::Null => "OTHER",
        }
    }
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Gender {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "MALE" => Some(Gender::Male),
            "FEMALE" => Some(Gender::Female),
            "OTHER" => Some(Gender::Other),
            "NULL" => Some(Gender::Null),
            _ => None,
        }
    }
}

impl core::convert::From<Gender> for mysql_common::Value {
    fn from(s: Gender) -> Self {
        match s {
            Gender::Null => mysql_common::Value::NULL,
            s => mysql_common::Value::Bytes(s.to_string().into_bytes()),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Gender {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match Gender::from_str(field.value) {
            Some(confirmation) => Ok(confirmation),
            None => Err(form::Errors::default()),
        }
    }
}

use rocket::form::{self, FromFormField, ValueField};

#[derive(Debug, PartialEq, Clone)]
pub enum Confirmation {
    Positive,
    Neutral,
    Negative,
    Null,
}

impl Confirmation {
    pub fn as_str(&self) -> &str {
        match self {
            Confirmation::Positive => "POSITIVE",
            Confirmation::Neutral => "NEUTRAL",
            Confirmation::Negative => "NEGATIVE",
            Confirmation::Null => "NULL",
        }
    }
}

impl std::fmt::Display for Confirmation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Confirmation {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "POSITIVE" => Some(Confirmation::Positive),
            "NEUTRAL" => Some(Confirmation::Neutral),
            "NEGATIVE" => Some(Confirmation::Negative),
            "NULL" => Some(Confirmation::Null),
            _ => None,
        }
    }
}

impl core::convert::From<Confirmation> for mysql_common::Value {
    fn from(s: Confirmation) -> Self {
        match s {
            Confirmation::Null => mysql_common::Value::NULL,
            s => mysql_common::Value::Bytes(s.to_string().into_bytes()),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Confirmation {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match Confirmation::from_str(field.value) {
            Some(confirmation) => Ok(confirmation),
            None => Err(form::Errors::default()),
        }
    }
}

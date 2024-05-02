use rocket::form::{self, FromFormField, ValueField};

#[derive(Debug, PartialEq, Clone)]
pub enum ConfirmationStatus {
    Positive,
    Neutral,
    Negative,
    Null,
}

impl ConfirmationStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "POSITIVE" => Some(ConfirmationStatus::Positive),
            "NEUTRAL" => Some(ConfirmationStatus::Neutral),
            "NEGATIVE" => Some(ConfirmationStatus::Negative),
            "NULL" => Some(ConfirmationStatus::Null),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            ConfirmationStatus::Positive => "POSITIVE",
            ConfirmationStatus::Neutral => "NEUTRAL",
            ConfirmationStatus::Negative => "NEGATIVE",
            ConfirmationStatus::Null => "NULL",
        }
    }
}

impl core::convert::From<ConfirmationStatus> for mysql_common::Value {
    fn from(s: ConfirmationStatus) -> Self {
        match s {
            ConfirmationStatus::Null => mysql_common::Value::NULL,
            s => mysql_common::Value::Bytes(s.to_str().to_string().into_bytes()),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for ConfirmationStatus {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match ConfirmationStatus::from_str(field.value) {
            Some(confirmation) => Ok(confirmation),
            None => Err(form::Errors::default()),
        }
    }
}

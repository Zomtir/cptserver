use rocket::form::{self, DataField, FromFormField, ValueField};
use rocket::form::error::{Errors, ErrorKind};

pub use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Acceptance {
    Draft,
    Pending,
    Accepted,
    Rejected,
}

impl Acceptance {
    pub fn as_str(&self) -> &str {
        match self {
            Acceptance::Draft => "DRAFT",
            Acceptance::Pending => "PENDING",
            Acceptance::Accepted => "ACCEPTED",
            Acceptance::Rejected => "REJECTED",
        }
    }
}

impl std::fmt::Display for Acceptance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Acceptance {
    type Err = crate::error::Error;

    fn from_str<'r>(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DRAFT" => Ok(Acceptance::Draft),
            "PENDING" => Ok(Acceptance::Pending),
            "ACCEPTED" => Ok(Acceptance::Accepted),
            "REJECTED" => Ok(Acceptance::Rejected),
            _ => Err(crate::error::Error::Parsing),
        }
    }
}

impl core::convert::From<Acceptance> for mysql_common::Value {
    fn from(a: Acceptance) -> Self {
        mysql_common::Value::Bytes(a.to_string().into_bytes())
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Acceptance {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Acceptance::from_str(field.value).map_err(|_| Errors::from(ErrorKind::Missing))
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        let web_string: String = crate::common::parse_field(field).await?;
        Acceptance::from_str(&web_string).map_err(|_| Errors::from(ErrorKind::Missing))
    }
}

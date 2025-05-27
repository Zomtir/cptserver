use rocket::form::error::{ErrorKind, Errors};
use rocket::form::{self, DataField, FromFormField, ValueField};

pub use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Occurrence {
    Occurring,
    Canceled,
    Voided,
}

impl Occurrence {
    pub fn as_str(&self) -> &str {
        match self {
            Occurrence::Occurring => "OCCURRING",
            Occurrence::Canceled => "CANCELED",
            Occurrence::Voided => "VOIDED",
        }
    }
}

impl std::fmt::Display for Occurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Occurrence {
    type Err = crate::error::ErrorKind;

    fn from_str<'r>(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OCCURRING" => Ok(Occurrence::Occurring),
            "CANCELED" => Ok(Occurrence::Canceled),
            "VOIDED" => Ok(Occurrence::Voided),
            _ => Err(crate::error::ErrorKind::Parsing),
        }
    }
}

impl core::convert::From<Occurrence> for mysql_common::Value {
    fn from(v: Occurrence) -> Self {
        mysql_common::Value::Bytes(v.to_string().into_bytes())
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Occurrence {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Occurrence::from_str(field.value).map_err(|_| Errors::from(ErrorKind::Missing))
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        let web_string: String = crate::common::parse_field(field).await?;
        Occurrence::from_str(&web_string).map_err(|_| Errors::from(ErrorKind::Missing))
    }
}

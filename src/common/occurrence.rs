use rocket::form::{self, DataField, FromFormField, ValueField};

#[derive(Debug, PartialEq, Clone)]
pub enum Occurrence {
    Occurring,
    Canceled,
    Voided,
}

impl Occurrence {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "OCCURRING" => Some(Occurrence::Occurring),
            "CANCELED" => Some(Occurrence::Canceled),
            "VOIDED" => Some(Occurrence::Voided),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Occurrence::Occurring => "OCCURRING",
            Occurrence::Canceled => "CANCELED",
            Occurrence::Voided => "VOIDED",
        }
    }

    pub fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}

impl core::convert::From<Occurrence> for mysql_common::Value {
    fn from(s: Occurrence) -> Self {
        mysql_common::Value::Bytes(s.to_str().to_string().into_bytes())
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Occurrence {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match Occurrence::from_str(field.value) {
            None => Err(form::Errors::default()),
            Some(event_status) => Ok(event_status),
        }
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        let web_string: String = crate::common::parse_field(field).await?;

        match Occurrence::from_str(&web_string) {
            None => return Err(form::Errors::default()),
            Some(event_status) => return Ok(event_status),
        }
    }
}

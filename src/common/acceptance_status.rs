use rocket::form::{self, DataField, FromFormField, ValueField};

#[derive(Debug, PartialEq, Clone)]
pub enum AcceptanceStatus {
    Draft,
    Pending,
    Occurring,
    Rejected,
    Canceled,
}

impl AcceptanceStatus {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "DRAFT" => Some(AcceptanceStatus::Draft),
            "PENDING" => Some(AcceptanceStatus::Pending),
            "OCCURRING" => Some(AcceptanceStatus::Occurring),
            "REJECTED" => Some(AcceptanceStatus::Rejected),
            "CANCELED" => Some(AcceptanceStatus::Canceled),
            _ => None,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            AcceptanceStatus::Draft => "DRAFT",
            AcceptanceStatus::Pending => "PENDING",
            AcceptanceStatus::Occurring => "OCCURRING",
            AcceptanceStatus::Rejected => "REJECTED",
            AcceptanceStatus::Canceled => "CANCELED",
        }
    }
}

impl core::convert::From<AcceptanceStatus> for mysql_common::Value {
    fn from(s: AcceptanceStatus) -> Self {
        mysql_common::Value::Bytes(s.to_str().to_string().into_bytes())
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for AcceptanceStatus {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match AcceptanceStatus::from_str(field.value) {
            None => Err(form::Errors::default()),
            Some(event_status) => Ok(event_status),
        }
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        let web_string: String = crate::common::parse_field(field).await?;

        match AcceptanceStatus::from_str(&web_string) {
            None => return Err(form::Errors::default()),
            Some(event_status) => return Ok(event_status),
        }
    }
}

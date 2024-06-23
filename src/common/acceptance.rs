use rocket::form::{self, DataField, FromFormField, ValueField};

#[derive(Debug, PartialEq, Clone)]
pub enum Acceptance {
    Draft,
    Pending,
    Accepted,
    Rejected,
}

impl Acceptance {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "DRAFT" => Some(Acceptance::Draft),
            "PENDING" => Some(Acceptance::Pending),
            "ACCEPTED" => Some(Acceptance::Accepted),
            "REJECTED" => Some(Acceptance::Rejected),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Acceptance::Draft => "DRAFT",
            Acceptance::Pending => "PENDING",
            Acceptance::Accepted => "ACCEPTED",
            Acceptance::Rejected => "REJECTED",
        }
    }

    pub fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}

impl core::convert::From<Acceptance> for mysql_common::Value {
    fn from(s: Acceptance) -> Self {
        mysql_common::Value::Bytes(s.to_str().to_string().into_bytes())
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Acceptance {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match Acceptance::from_str(field.value) {
            None => Err(form::Errors::default()),
            Some(event_status) => Ok(event_status),
        }
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        let web_string: String = crate::common::parse_field(field).await?;

        match Acceptance::from_str(&web_string) {
            None => return Err(form::Errors::default()),
            Some(event_status) => return Ok(event_status),
        }
    }
}

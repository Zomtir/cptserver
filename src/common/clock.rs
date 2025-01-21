use rocket::form::{self, DataField, FromFormField, ValueField};

/*
 * Time and chronology related stuff
 */

/// WebDate
pub struct WebDate(pub chrono::NaiveDate);

#[allow(dead_code)]
#[allow(clippy::needless_lifetimes)]
impl<'r> WebDate {
    fn from_str(s: &str) -> form::Result<'r, Self> {
        match chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            Err(..) => Err(form::Errors::default()),
            Ok(date) => Ok(WebDate(date)),
        }
    }

    pub fn to_naive(&self) -> chrono::NaiveDate {
        self.0
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for WebDate {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        WebDate::from_str(field.value)
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        let web_string: String = crate::common::parse_field(field).await?;
        WebDate::from_str(&web_string)
    }
}

/// WebDateTime
pub struct WebDateTime(pub chrono::NaiveDateTime);

#[allow(dead_code)]
#[allow(clippy::needless_lifetimes)]
impl<'r> WebDateTime {
    fn from_str(s: &str) -> form::Result<'r, Self> {
        match chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d-%H-%M") {
            Err(..) => Err(form::Errors::default()),
            Ok(datetime) => Ok(WebDateTime(datetime)),
        }
    }

    pub fn to_naive(&self) -> chrono::NaiveDateTime {
        self.0
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for WebDateTime {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        WebDateTime::from_str(field.value)
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        let web_string: String = crate::common::parse_field(field).await?;
        WebDateTime::from_str(&web_string)
    }
}

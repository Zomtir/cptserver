use rocket::data::ToByteUnit;
use rocket::form::{self, DataField, FromFormField, ValueField};

// Common module
mod clock;
mod club;
mod course;
mod event;
mod location;
mod math;
mod skill;
mod team;
mod user;

// Re-export
pub use clock::*;
pub use club::*;
pub use course::*;
pub use event::*;
pub use location::*;
pub use math::*;
pub use skill::*;
pub use team::*;
pub use user::*;

pub async fn parse_field<'r>(field: DataField<'r, '_>) -> form::Result<'r, String> {
    match field.data.open(200.bytes()).into_string().await {
        Err(..) => Err(form::Errors::default()),
        Ok(string) => Ok(string.into_inner()),
    }
}

pub struct WebBool(bool);

#[rocket::async_trait]
impl<'r> FromFormField<'r> for WebBool {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match field.value {
            "true" => Ok(WebBool(true)),
            "false" => Ok(WebBool(false)),
            _ => Err(form::Errors::default()),
        }
    }
}

impl<'r> WebBool {
    pub fn to_bool(&self) -> bool {
        self.0
    }
}

use rocket::data::ToByteUnit;
use rocket::form::{self, DataField, FromFormField, ValueField};

// Common module
mod acceptance;
mod bank_account;
mod clock;
mod club;
mod confirmation;
mod course;
mod event;
mod gender;
mod item;
mod license;
mod location;
mod math;
mod occurrence;
mod organisation;
mod skill;
mod team;
mod user;

// Re-export
pub use acceptance::*;
pub use bank_account::*;
pub use clock::*;
pub use club::*;
pub use confirmation::*;
pub use course::*;
pub use event::*;
#[allow(unused_imports)]
pub use gender::*;
pub use item::*;
pub use license::*;
pub use location::*;
pub use math::*;
pub use occurrence::*;
pub use organisation::*;
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

impl WebBool {
    pub fn to_bool(&self) -> bool {
        self.0
    }
}

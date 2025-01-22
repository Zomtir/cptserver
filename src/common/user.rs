use crate::common::{BankAccount, License};
use crate::error::Error;
use regex::Regex;
use serde::{Deserialize, Serialize};

/*
 * STRUCTS
 */

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u64,
    pub key: Option<String>,

    pub firstname: String,
    pub lastname: String,
    pub nickname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<BankAccount>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_main: Option<License>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_extra: Option<License>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl User {
    pub fn from_info(id: u64, key: String, firstname: String, lastname: String, nickname: Option<String>) -> User {
        User {
            id,
            key: Some(key),
            enabled: None,
            active: None,
            firstname,
            lastname,
            nickname,
            address: None,
            email: None,
            phone: None,
            birth_date: None,
            birth_location: None,
            nationality: None,
            gender: None,
            height: None,
            weight: None,
            bank_account: None,
            license_main: None,
            license_extra: None,
            note: None,
        }
    }

    pub fn from_row(row: &mut mysql::Row) -> Option<User> {
        row.take::<Option<u64>, &str>("user_id").unwrap().map(|user_id| {
            User::from_info(
                user_id,
                row.take("user_key").unwrap(),
                row.take("user_firstname").unwrap(),
                row.take("user_lastname").unwrap(),
                row.take("user_nickname").unwrap(),
            )
        })
    }
}

/*
 * METHODS
 */

pub fn check_user_key(key: &Option<String>) -> Result<String, Error> {
    let text = match key {
        None => return Err(Error::UserKeyMissing),
        Some(text) => text,
    };

    validate_user_key(text)?;
    Ok(text.into())
}

pub fn validate_user_key(text: &str) -> Result<(), Error> {
    if text.len() < 2 || text.len() > 20 {
        return Err(Error::UserKeyInvalid);
    };

    if !text.chars().all(|c| c.is_alphanumeric()) {
        return Err(Error::UserKeyInvalid);
    }

    Ok(())
}

pub fn check_user_email(email: &Option<String>) -> Result<String, Error> {
    let text = match email {
        None => return Err(Error::UserEmailMissing),
        Some(text) => text,
    };

    validate_user_email(text)?;
    Ok(text.into())
}

pub fn validate_user_email(text: &str) -> Result<(), Error> {
    match Regex::new(r"^([a-z0-9._\-]([a-z0-9._\-+]*)?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})") {
        Err(..) => Err(Error::RegexError),
        Ok(regex) => match regex.is_match(text) {
            false => Err(Error::UserEmailInvalid),
            true => Ok(()),
        },
    }
}

use crate::error::Error;
use regex::Regex;
use serde::{Deserialize, Serialize};

/*
 * STRUCTS
 */

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub key: Option<String>,
    pub enabled: Option<bool>,
    pub active: Option<bool>,
    pub firstname: String,
    pub lastname: String,
    pub nickname: Option<String>,
    pub address: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub iban: Option<String>,
    pub birthday: Option<chrono::NaiveDate>,
    pub birthlocation: Option<String>,
    pub nationality: Option<String>,
    pub gender: Option<String>,
    pub federationnumber: Option<i64>,
    pub federationpermissionsolo: Option<chrono::NaiveDate>,
    pub federationpermissionteam: Option<chrono::NaiveDate>,
    pub federationresidency: Option<chrono::NaiveDate>,
    pub datadeclaration: Option<u8>,
    pub datadisclaimer: Option<String>,
    pub note: Option<String>,
}

impl User {
    pub fn from_info(id: i64, key: String, firstname: String, lastname: String, nickname: Option<String>) -> User {
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
            iban: None,
            birthday: None,
            birthlocation: None,
            nationality: None,
            gender: None,
            federationnumber: None,
            federationpermissionsolo: None,
            federationpermissionteam: None,
            federationresidency: None,
            datadeclaration: None,
            datadisclaimer: None,
            note: None,
        }
    }
}

/*
 * METHODS
 */

pub fn validate_user_key(key: &Option<String>) -> Result<Option<String>, Error> {
    let text = match key {
        None => return Ok(None),
        Some(text) => text,
    };

    if text.is_empty() {
        return Ok(None);
    };

    if text.len() < 2 || text.len() > 20 {
        return Err(Error::UserKeyInvalid);
    };

    Ok(key.clone())
}

pub fn validate_email(email: &Option<String>) -> Result<Option<String>, Error> {
    let text = match email {
        None => return Ok(None),
        Some(text) => text,
    };

    if text.is_empty() {
        return Ok(None);
    };

    match Regex::new(r"^([a-z0-9._\-]([a-z0-9._\-+]*)?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})") {
        Err(..) => Err(Error::RegexError),
        Ok(regex) => match regex.is_match(text) {
            false => Err(Error::UserEmailInvalid),
            true => Ok(email.clone()),
        },
    }
}

use rand::Rng;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::Error;

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub key: Option<String>,
    pub enabled: Option<bool>,
    pub firstname: String,
    pub lastname: String,
    pub address: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub iban: Option<String>,
    pub birthday: Option<chrono::NaiveDate>,
    pub birthlocation: Option<String>,
    pub nationality: Option<String>,
    pub gender: Option<String>,
    pub federationNumber: Option<i64>,
    pub federationPermissionSolo: Option<chrono::NaiveDate>,
    pub federationPermissionTeam: Option<chrono::NaiveDate>,
    pub federationResidency: Option<chrono::NaiveDate>,
    pub dataDeclaration: Option<u8>,
    pub dataDisclaimer: Option<String>,
    pub note: Option<String>,
}

impl User {
    pub fn from_info(id: i64, key: String, firstname: String, lastname: String) -> User {
        User {
            id: id,
            key: Some(key),
            enabled: None,
            firstname: firstname,
            lastname: lastname,
            address: None,
            email: None,
            phone: None,
            iban: None,
            birthday: None,
            birthlocation: None,
            nationality: None,
            gender: None,
            federationNumber: None,
            federationPermissionSolo: None,
            federationPermissionTeam: None,
            federationResidency: None,
            dataDeclaration: None,
            dataDisclaimer: None,
            note: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Slot {
    pub id: i64,
    pub key: String,
    pub pwd: Option<String>,
    pub title: String,
    pub location: Location,
    #[serde(with = "crate::clock::datetime_format")]
    pub begin: chrono::NaiveDateTime,
    #[serde(with = "crate::clock::datetime_format")]
    pub end: chrono::NaiveDateTime,
    pub status: String,
    pub course_id: Option<u32>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Course {
    pub id: u32,
    pub key: String,
    pub title: String,
    pub branch: Branch,
    pub threshold: u8,
    pub active: bool,
    pub public: bool,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Right {
    pub admin_courses: bool,
    pub admin_inventory: bool,
    pub admin_rankings: bool,
    pub admin_event: bool,
    pub admin_teams: bool,
    pub admin_term: bool,
    pub admin_users: bool,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Team {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub right: Option<Right>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    pub id: u32,
    pub key: String,
    pub title: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Branch {
    pub id: u16,
    pub key: String,
    pub title: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ranking {
    pub id: u32,
    pub user: User,
    pub branch: Branch,
    pub rank: u8,
    pub date: chrono::NaiveDate,
    pub judge: User,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Term {
    pub id: i64,
    pub user: User,
    #[serde(with = "crate::clock::datetime_format")]
    pub begin: chrono::NaiveDateTime,
    #[serde(with = "crate::clock::datetime_format")]
    pub end: chrono::NaiveDateTime,
}

/*
 * METHODS
 */

pub fn random_string(size: usize) -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}

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

    match Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})") {
        Err(..) => Err(Error::RegexError),
        Ok(regex) => match regex.is_match(&text) {
            false => Err(Error::UserEmailInvalid),
            true => Ok(email.clone()),
        },
    }
}

pub fn validate_clear_password(password: String) -> Result<String, Error> {
    if password.len() < 6 || password.len() > 50 {
        return Err(Error::SlotPasswordInvalid);
    };

    Ok(password.to_string())
}

pub fn decode_hash256(hash: &str) -> Option<Vec<u8>> {
    match &hash.len() {
        // Sha256 is 64 chars long
        64 => match hex::decode(&hash) {
            Ok(bytes) => Some(bytes),
            _ => None,
        },
        _ => None,
    }
}

pub fn decode_hash128(hash: &str) -> Option<Vec<u8>> {
    match &hash.len() {
        // 128 bits are 32 chars long
        32 => match hex::decode(&hash) {
            Ok(bytes) => Some(bytes),
            _ => None,
        },
        _ => None,
    }
}

pub fn hash_sha256(meal: &Vec<u8>, pepper: &Vec<u8>) -> Vec<u8> {
    let spiced_meal: Vec<u8> = meal.iter().cloned().chain(pepper.iter().cloned()).collect();
    let digested_meal = Sha256::digest(&spiced_meal);

    // println!("spiced meal: {:?}", spiced_meal);
    // println!("digested meal: {:?}", digested_meal);

    return digested_meal.to_vec();
}

pub fn hash128_string(meal: &String) -> Vec<u8> {
    let digested_meal = Sha256::digest(meal.as_bytes());

    return digested_meal[..=15].to_vec();
}

pub fn random_bytes(size: usize) -> Vec<u8> {
    rand::thread_rng()
        .sample_iter(rand::distributions::Standard)
        .take(size)
        .collect()
}

pub fn is_slot_valid(slot: &Slot) -> bool {
    return slot.begin + crate::config::CONFIG_SLOT_WINDOW_MINIMUM() < slot.end;
}

pub fn validate_slot_dates(slot: &mut Slot) -> Result<(), Error> {
    slot.begin = crate::clock::duration_round(slot.begin, crate::config::CONFIG_SLOT_WINDOW_SNAP())?;

    slot.end = crate::clock::duration_round(slot.end, crate::config::CONFIG_SLOT_WINDOW_SNAP())?;

    let earliest_end = slot.begin + crate::config::CONFIG_SLOT_WINDOW_MINIMUM();

    if earliest_end > slot.end {
        slot.end = earliest_end;
    }

    Ok(())
}

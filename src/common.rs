use rand::Rng;
use regex::Regex;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub key: String,
    pub pwd: Option<String>,
    pub enabled: Option<bool>,
    pub firstname: String,
    pub lastname: String,
    pub iban: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub birthday: Option<chrono::NaiveDateTime>,
    pub gender: Option<String>,
    pub organization_id: Option<i64>,
}

impl User {
    pub fn from_info(id: i64, key: String, firstname: String, lastname: String) -> User {
        User {
            id: id,
            key: key,
            pwd: None,
            enabled: None,
            firstname: firstname,
            lastname: lastname,
            iban: None,
            email: None,
            phone: None,
            address: None,
            birthday: None,
            gender: None,
            organization_id: None,
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
    pub status: Option<String>,
    pub course_id: Option<u32>,
    pub owners: Option<Vec<User>>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Course {
    pub id: u32,
    pub key: String,
    pub title: String,
    pub branch: Branch,
    pub threshold: u8,
    pub access: Access,
    pub active: bool,
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
    pub right: Right,
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
pub struct Access {
    pub id: u8,
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

pub fn verify_email(email: &Option<String>) -> Option<bool> {
    let text = match email {
        None => return None,
        Some(text) => text,
    };

    if text.is_empty() { return None; };

    match Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})") {
        Err(..) => Some(false),
        Ok(regex) => Some(regex.is_match(&text)),
    }
}

pub fn random_string(size: usize) -> String {
    rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(size).map(char::from).collect()
}

pub fn validate_clear_password(pwd: Option<String>) -> Option<String> {
    let password = match pwd {
        None => return None,
        Some(password) => password,
    };

    if password.len() < 6 || password.len() > 50 {
        return None;
    };

    Some(password.to_string())
}

pub fn verify_hashed_password(password: & str) -> Option<Vec<u8>> {
    match &password.len() {
        // Sha256 is 64 chars long
        64 => match hex::decode(&password) {
            Ok(bytes) => Some(bytes),
            _ => None,
        },
        _ => None,
    }
}

pub fn hash_sha256(salt: &Vec<u8>, pepper: &Vec<u8>) -> Vec<u8> {
    let mut spice : Vec<u8> = salt.clone();
    spice.extend(pepper.iter().cloned());
    let sha_spice = Sha256::digest(&spice);

    // println!("spice bytes: {:?}", spice);
    // println!("spice hex: {:x}", sha_spice);

    return sha_spice.to_vec();
}

pub fn random_bytes(size: usize) -> Vec<u8> {
    rand::thread_rng().sample_iter(rand::distributions::Standard).take(size).collect()
}

pub fn is_slot_valid(slot: & Slot) -> bool {
    return slot.begin + crate::config::CONFIG_SLOT_WINDOW_MINIMUM() < slot.end;
}

pub fn validate_slot_dates(slot: &mut Slot) -> Option<()> {
    slot.begin = match crate::clock::duration_round(slot.begin, crate::config::CONFIG_SLOT_WINDOW_SNAP()) {
        Err(..) => return None,
        Ok(dt) => dt,
    };

    slot.end = match crate::clock::duration_round(slot.end, crate::config::CONFIG_SLOT_WINDOW_SNAP()) {
        Err(..) => return None,
        Ok(dt) => dt,
    };

    let earliest_end = slot.begin + crate::config::CONFIG_SLOT_WINDOW_MINIMUM();

    if earliest_end < slot.end {
        slot.end = earliest_end;
    }

    return Some(())
}

pub fn censor_user(user_id: i64, user: &mut User) {
    user.id = user_id;
    user.pwd = None;
    //user.enabled = None;
}

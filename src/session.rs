extern crate lazy_static;

use rocket::request::{self, Request, FromRequest};
use rocket::http::Status;
use request::Outcome;

use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use crate::db::get_pool_conn;

use std::sync::Mutex;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use regex::Regex;
use rand::Rng;
use sha2::{Sha256, Digest};

lazy_static::lazy_static! {
    pub static ref USERSESSIONS: Mutex<HashMap<String,UserSession>> = Mutex::new(HashMap::new());
    pub static ref SLOTSESSIONS: Mutex<HashMap<String,SlotSession>> = Mutex::new(HashMap::new()); 
}

/*
 * STRUCTS
 */

#[derive(Debug,Clone)]
pub struct UserSession {
    pub token: String,
    pub expiry: chrono::DateTime<chrono::Utc>,
    pub user: User,
}

impl<'a, 'r> FromRequest<'a, 'r> for UserSession {
    type Error = SessionError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let head_token = match request.headers().get_one("Token") {
            None => return Outcome::Failure((Status::Unauthorized,SessionError::MissingTokenHeader)),
            Some(token) => token,
        };

        let session : UserSession = match USERSESSIONS.lock().unwrap().get(&head_token.to_string()).cloned() {
            None => { return Outcome::Failure((Status::Unauthorized,SessionError::InvalidSession)); },
            Some(session) => session,
        };

        if session.token != head_token.to_string() {
            return Outcome::Failure((Status::Unauthorized,SessionError::InvalidSession));
        }

        if session.expiry < chrono::Utc::now() {
            USERSESSIONS.lock().unwrap().remove(&session.token);
            return Outcome::Failure((Status::Unauthorized,SessionError::TokenExpired));
        }
        
        return Outcome::Success(session);
    }
}

#[derive(Debug,Clone)]
pub struct SlotSession {
    pub token: String,
    pub expiry: chrono::DateTime<chrono::Utc>,
    pub slot_id: u32,
    pub slot_key: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for SlotSession {
    type Error = SessionError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let head_token = request.headers().get_one("Token");
        if head_token.is_none() { return Outcome::Failure((Status::Unauthorized,SessionError::MissingTokenHeader)); }

        let session : Option<SlotSession> = SLOTSESSIONS.lock().unwrap().get(&head_token.unwrap().to_string()).cloned();

        match session {
            None => { return Outcome::Failure((Status::Unauthorized,SessionError::InvalidSession)); },
            Some(session) => {
                if session.token != head_token.unwrap().to_string() {
                    return Outcome::Failure((Status::Unauthorized,SessionError::InvalidSession));
                }

                if session.expiry < chrono::Utc::now() {
                    SLOTSESSIONS.lock().unwrap().remove(&session.token);
                    return Outcome::Failure((Status::Unauthorized,SessionError::TokenExpired));
                }
                
                return Outcome::Success(session);
            }
        }
    }
}

#[derive(Debug)]
pub enum SessionError {
    MissingTokenHeader,
    InvalidSession,
    InsufficientRights,
    TokenExpired,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u32,
    pub key: String,
    pub pwd: Option<String>,
    pub firstname: String,
    pub lastname: String,
    pub enabled: bool,
    pub admin_users: bool,
    pub admin_rankings: bool,
    pub admin_reservations: bool,
    pub admin_courses: bool,
}

impl User {
    pub fn info(id: u32, key: String, firstname: String, lastname: String) -> User {
        User {
            id: id,
            key: key,
            pwd: None,
            firstname: firstname,
            lastname: lastname,
            enabled: false,
            admin_users: false,
            admin_rankings: false,
            admin_reservations: false,
            admin_courses: false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Slot {
    pub id: u32,
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
    pub user_id: Option<u32>,
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

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Member {
    pub id: u32,
    pub key: String,
    pub firstname: String,
    pub lastname: String,
}

impl Member {
    pub fn from_user(user: &User) -> Member {
        Member {
            id: user.id,
            key: user.key.to_string(),
            firstname: user.firstname.to_string(),
            lastname: user.lastname.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Team {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub admin_courses: bool,
    pub admin_rankings: bool,
    pub admin_reservations: bool,
    pub admin_users: bool,
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
    pub user: Member,
    pub branch: Branch,
    pub rank: u8,
    pub date: chrono::NaiveDate,
    pub judge: Member,
}

/*
 * METHODS
 */

pub fn verify_email(email: & str) -> bool {
    match Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})") {
        Err(..) => false,
        Ok(regex) => regex.is_match(email),
    }
}

pub fn random_string(size: usize) -> String {
    rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(size).map(char::from).collect()
}

pub fn verify_password(password: & str) -> Option<Vec<u8>> {
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

pub fn is_user_created(user_key: & str) -> Option<bool> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT COUNT(1) FROM users WHERE user_key = :user_key").ok()?;
    let count : Option<i32> = conn.exec_first(&stmt, mysql::params! { "user_key" => user_key }).ok()?;

    return Some(count.unwrap() == 1);
}

pub fn get_slot_info(slot_id : & u32) -> Option<Slot> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT slot_id, slot_key, s.title, l.location_id, l.location_key, l.title, s.begin, s.end, s.status, s.course_id, s.user_id
                          FROM slots s
                          JOIN locations l ON l.location_id = s.location_id
                          WHERE slot_id = :slot_id").unwrap();

    let params = mysql::params! { "slot_id" => slot_id };
    let map =
        | (slot_id, slot_key, slot_title, location_id, location_key, location_title, begin, end, status, course_id, user_id)
        : (u32, _, _, u32, _, _, _, _, String, Option<u32>, Option<u32>)
        | Slot {
            id: slot_id, key: slot_key, pwd: None, title: slot_title, begin, end, status: Some(status),
            location: Location {id: location_id, key: location_key, title: location_title},
            course_id: course_id, user_id: user_id};

    match conn.exec_map(&stmt, &params, &map) {
        Err(..) => return None,
        Ok(mut slots) => return Some(slots.remove(0)),
    }
}

pub fn is_slot_valid(slot: & Slot) -> bool {
    return slot.begin + crate::config::CONFIG_SLOT_WINDOW_MINIMUM() > slot.end;
}

// Perhaps the database should be locked between checking for a free slot and modifying the slot later
pub fn is_slot_free(slot: & Slot) -> Option<bool> {
    if !is_slot_valid(slot) {return Some(false)};

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT COUNT(1) FROM slots
                          WHERE location_id = :location_id
                          AND NOT (end <= :begin OR begin >= :end)
                          AND status = 'OCCURRING'").unwrap();
    
    let params = mysql::params! {
        "location_id" => &slot.location.id,
        "begin" => &slot.begin,
        "end" => &slot.end,
    };

    match conn.exec_first::<u32,_,_>(&stmt, &params){
        Err(..) => return None,
        Ok(None) => return None,
        Ok(Some(count)) => return Some(count == 0),
    };
}

pub fn round_slot_window(slot: &mut Slot) -> Option<()> {
    use crate::clock::NaiveDurationRound;
    slot.begin = match slot.begin.duration_round(crate::config::CONFIG_SLOT_WINDOW_SNAP()) {
        Err(..) => return None,
        Ok(dt) => dt,
    };
    slot.end = match slot.end.duration_round(crate::config::CONFIG_SLOT_WINDOW_SNAP()) {
        Err(..) => return None,
        Ok(dt) => dt,
    };

    return Some(())
}

pub fn set_slot_status(slot_id : u32, status_required : &str, status_update : &str) -> Option<()> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("UPDATE slots SET
        status = :status_update
        WHERE slot_id = :slot_id AND status = :status_required").unwrap();
    let params = mysql::params! {
        "slot_id" => slot_id,
        "status_required" => status_required,
        "status_update" => status_update,
    };

    match conn.exec::<String,_,_>(&stmt,&params){
        Err(..) => return None,
        Ok(..) => return Some(()),
    };
}
extern crate lazy_static;

use rocket::outcome::Outcome::Success;
use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::sync::Mutex;

use crate::common::{Right, User};
use crate::error::Error;

lazy_static::lazy_static! {
    pub static ref USERSESSIONS: Mutex<HashMap<String,UserSession>> = Mutex::new(HashMap::new());
    pub static ref SLOTSESSIONS: Mutex<HashMap<String,SlotSession>> = Mutex::new(HashMap::new());
}

/*
 * STRUCTS
 */

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credential {
    pub login: String,
    pub password: String,
    pub salt: String,
}

#[derive(Debug, Clone)]
pub struct UserSession {
    pub token: String,
    pub expiry: chrono::DateTime<chrono::Utc>,
    pub user: User,
    pub right: Right,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserSession {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Error> {
        let head_token = match request.headers().get_one("Token") {
            None => return Error::SessionTokenMissing.outcome(),
            Some(token) => token,
        };

        let session: UserSession = match USERSESSIONS.lock().unwrap().get(&head_token.to_string()).cloned() {
            None => {
                return Error::SessionTokenInvalid.outcome();
            }
            Some(session) => session,
        };

        if session.token != head_token.to_string() {
            return Error::SessionTokenInvalid.outcome();
        }

        if session.expiry < chrono::Utc::now() {
            USERSESSIONS.lock().unwrap().remove(&session.token);
            return Error::SessionTokenExpired.outcome();
        }

        Success(session)
    }
}

#[derive(Debug, Clone)]
pub struct SlotSession {
    pub token: String,
    pub expiry: chrono::DateTime<chrono::Utc>,
    pub slot_id: i64,
    pub slot_key: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SlotSession {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Error> {
        let head_token = match request.headers().get_one("Token") {
            None => return Error::SessionTokenMissing.outcome(),
            Some(token) => token,
        };

        let session: SlotSession = match SLOTSESSIONS.lock().unwrap().get(&head_token.to_string()).cloned() {
            None => {
                return Error::SessionTokenInvalid.outcome();
            }
            Some(session) => session,
        };

        if session.token != head_token.to_string() {
            return Error::SessionTokenInvalid.outcome();
        }

        if session.expiry < chrono::Utc::now() {
            SLOTSESSIONS.lock().unwrap().remove(&session.token);
            return Error::SessionTokenExpired.outcome();
        }

        Success(session)
    }
}

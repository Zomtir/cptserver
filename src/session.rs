extern crate lazy_static;

use rocket::request::{Request, FromRequest, Outcome};
use rocket::outcome::Outcome::{Success};

use std::sync::Mutex;
use std::collections::HashMap;

use crate::api::ApiError;
use crate::common::{User, Right};

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
    pub right: Right,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserSession {
    type Error = ApiError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self,ApiError> {
        let head_token = match request.headers().get_one("Token") {
            None => return ApiError::SESSION_TOKEN_MISSING.outcome(),
            Some(token) => token,
        };

        let session : UserSession = match USERSESSIONS.lock().unwrap().get(&head_token.to_string()).cloned() {
            None => { return ApiError::SESSION_TOKEN_INVALID.outcome(); },
            Some(session) => session,
        };

        if session.token != head_token.to_string() {
            return ApiError::SESSION_TOKEN_INVALID.outcome();
        }

        if session.expiry < chrono::Utc::now() {
            USERSESSIONS.lock().unwrap().remove(&session.token);
            return ApiError::SESSION_TOKEN_EXPIRED.outcome();
        }
        
        Success(session)
    }
}

#[derive(Debug,Clone)]
pub struct SlotSession {
    pub token: String,
    pub expiry: chrono::DateTime<chrono::Utc>,
    pub slot_id: u32,
    pub slot_key: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SlotSession {
    type Error = ApiError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self,ApiError> {
        let head_token = match request.headers().get_one("Token") {
            None => return ApiError::SESSION_TOKEN_MISSING.outcome(),
            Some(token) => token,
        };

        let session : SlotSession = match SLOTSESSIONS.lock().unwrap().get(&head_token.to_string()).cloned() {
            None => { return ApiError::SESSION_TOKEN_INVALID.outcome(); },
            Some(session) => session,
        };

        if session.token != head_token.to_string() {
            return ApiError::SESSION_TOKEN_INVALID.outcome();
        }

        if session.expiry < chrono::Utc::now() {
            SLOTSESSIONS.lock().unwrap().remove(&session.token);
            return ApiError::SESSION_TOKEN_EXPIRED.outcome();
        }
        
        Success(session)
    }
}

extern crate lazy_static;

use rocket::outcome::Outcome::Success;
use rocket::request::{FromRequest, Outcome, Request};

use std::collections::HashMap;
use std::sync::Mutex;

use crate::common::{Right, User};
use crate::error::{Error, ErrorKind};

lazy_static::lazy_static! {
    pub static ref ADMINSESSION: Mutex<Option<String>> = Mutex::new(None);
    pub static ref USERSESSIONS: Mutex<HashMap<String,UserSession>> = Mutex::new(HashMap::new());
    pub static ref EVENTSESSIONS: Mutex<HashMap<String,EventSession>> = Mutex::new(HashMap::new());
}

/*
 * STRUCTS
 */

#[derive(Debug, Clone)]
pub struct UserSession {
    pub expiry: chrono::DateTime<chrono::Utc>,
    pub user: User,
    pub right: Right,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserSession {
    type Error = crate::error::Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, crate::error::Error> {
        let head_token = match request.headers().get_one("Token") {
            None => return Error::new(ErrorKind::Missing, "Session token is missing").outcome(),
            Some(token) => token,
        };

        let session: UserSession = match USERSESSIONS.lock().unwrap().get(&head_token.to_string()).cloned() {
            None => {
                return Error::new(ErrorKind::Invalid, "Invalid session token").outcome();
            }
            Some(session) => session,
        };

        if session.expiry < chrono::Utc::now() {
            USERSESSIONS.lock().unwrap().remove(head_token);
            return Error::new(ErrorKind::Expired, "Session token has expired").outcome();
        }

        Success(session)
    }
}

impl UserSession {
    pub fn admin(user: &User) -> Self {
        UserSession {
            expiry: chrono::Utc::now() + chrono::Duration::hours(1),
            user: user.clone(),
            right: Right {
                right_club_write: true,
                right_club_read: true,
                right_competence_write: true,
                right_competence_read: true,
                right_course_write: true,
                right_course_read: true,
                right_event_write: true,
                right_event_read: true,
                right_inventory_write: true,
                right_inventory_read: true,
                right_location_write: true,
                right_location_read: true,
                right_organisation_write: true,
                right_organisation_read: true,
                right_team_write: true,
                right_team_read: true,
                right_user_write: true,
                right_user_read: true,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventSession {
    pub token: String,
    pub expiry: chrono::DateTime<chrono::Utc>,
    pub event_id: u64,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for EventSession {
    type Error = crate::error::Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, crate::error::Error> {
        let head_token = match request.headers().get_one("Token") {
            None => return Error::new(ErrorKind::Missing, "Session token is missing").outcome(),
            Some(token) => token,
        };

        let session: EventSession = match EVENTSESSIONS.lock().unwrap().get(&head_token.to_string()).cloned() {
            None => {
                return Error::new(ErrorKind::Invalid, "Invalid session token").outcome();
            }
            Some(session) => session,
        };

        // Wrong token, should not happen as the session is looked up by the token, but just in case
        if session.token != *head_token {
            return Error::new(ErrorKind::Mismatch, "Session token does not match internally").outcome();
        }

        if session.expiry < chrono::Utc::now() {
            EVENTSESSIONS.lock().unwrap().remove(&session.token);
            return Error::new(ErrorKind::Expired, "Session token has expired").outcome();
        }

        Success(session)
    }
}

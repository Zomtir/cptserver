extern crate lazy_static;

use rocket::outcome::Outcome::Success;
use rocket::request::{FromRequest, Outcome, Request};

use std::collections::HashMap;
use std::sync::Mutex;

use crate::common::{Right, User};
use crate::error::Error;

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

        if session.expiry < chrono::Utc::now() {
            USERSESSIONS.lock().unwrap().remove(head_token);
            return Error::SessionTokenExpired.outcome();
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
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Error> {
        let head_token = match request.headers().get_one("Token") {
            None => return Error::SessionTokenMissing.outcome(),
            Some(token) => token,
        };

        let session: EventSession = match EVENTSESSIONS.lock().unwrap().get(&head_token.to_string()).cloned() {
            None => {
                return Error::SessionTokenInvalid.outcome();
            }
            Some(session) => session,
        };

        if session.token != *head_token {
            return Error::SessionTokenInvalid.outcome();
        }

        if session.expiry < chrono::Utc::now() {
            EVENTSESSIONS.lock().unwrap().remove(&session.token);
            return Error::SessionTokenExpired.outcome();
        }

        Success(session)
    }
}

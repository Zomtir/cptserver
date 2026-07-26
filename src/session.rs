use axum::extract::FromRequestParts;
use axum::http::request::Parts;

use crate::common::{Right, User};
use crate::error::{Error, ErrorKind};

/*
 * STRUCTS
 */

#[derive(Debug, Clone)]
pub struct UserSession {
    pub expiry: chrono::DateTime<chrono::Utc>,
    pub user: User,
    pub right: Right,
}

impl FromRequestParts<crate::AppState> for UserSession {
    type Rejection = crate::error::Error;

    async fn from_request_parts(parts: &mut Parts, state: &crate::AppState) -> Result<Self, Self::Rejection> {
        let head_token = parts
            .headers
            .get("Token")
            .ok_or(Error::new(ErrorKind::Missing, "Session token is missing"))?
            .to_str()
            .map_err(|_| Error::new(ErrorKind::Invalid, "Invalid Token header"))?;

        let session = state
            .user_sessions
            .lock()
            .unwrap()
            .get(head_token)
            .cloned()
            .ok_or(Error::new(ErrorKind::Invalid, "Invalid session token"))?;

        if session.expiry < chrono::Utc::now() {
            state.user_sessions.lock().unwrap().remove(head_token);
            return Err(Error::new(ErrorKind::Expired, "Session token has expired"));
        }

        Ok(session)
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
                right_discipline_write: true,
                right_discipline_read: true,
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

impl FromRequestParts<crate::AppState> for EventSession {
    type Rejection = crate::error::Error;

    async fn from_request_parts(parts: &mut Parts, state: &crate::AppState) -> Result<Self, Self::Rejection> {
        let head_token = parts
            .headers
            .get("Token")
            .ok_or(Error::new(ErrorKind::Missing, "Session token is missing"))?
            .to_str()
            .map_err(|_| Error::new(ErrorKind::Invalid, "Invalid Token header"))?;

        let session = state
            .event_sessions
            .lock()
            .unwrap()
            .get(head_token)
            .cloned()
            .ok_or(Error::new(ErrorKind::Invalid, "Invalid session token"))?;

        if session.expiry < chrono::Utc::now() {
            state.event_sessions.lock().unwrap().remove(head_token);
            return Err(Error::new(ErrorKind::Expired, "Session token has expired"));
        }

        Ok(session)
    }
}

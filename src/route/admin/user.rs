use crate::common::{Credential, User, WebBool};
use crate::error::{Error, ErrorKind, Result};
use crate::session::UserSession;
use crate::AppState;
use axum::extract::State;
use axum::Json;

mod bank_account;
mod license;

pub use bank_account::*;
pub use license::*;

/* ROUTES */

pub fn user_list(
    State(state): State<AppState>,
    session: UserSession,
    active: Option<WebBool>,
) -> Result<Json<Vec<User>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_user_read)?;

    let users = crate::db::user::user_list(conn, active.map(|b| b.to_bool()))?;
    Ok(Json(users))
}

pub fn user_detailed(State(state): State<AppState>, session: UserSession, user_id: u64) -> Result<Json<User>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_user_read)?;

    let user = crate::db::user::user_detailed(conn, user_id)?;
    Ok(Json(user))
}

pub fn user_create(State(state): State<AppState>, session: UserSession, mut user: Json<User>) -> Result<String> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_user_write)?;

    let user_id = crate::db::user::user_create(conn, &mut user)?;

    Ok(user_id.to_string())
}

pub fn user_edit(
    State(state): State<AppState>,
    session: UserSession,
    user_id: u64,
    mut user: Json<User>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_user_write)?;

    crate::db::user::user_edit(conn, user_id, &mut user)?;
    Ok(())
}

pub fn user_delete(State(state): State<AppState>, session: UserSession, user_id: u64) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_user_write)?;

    crate::db::user::user_delete(conn, user_id)?;
    Ok(())
}

pub fn user_password_info(
    State(state): State<AppState>,
    session: UserSession,
    user_id: u64,
) -> Result<Json<Credential>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_user_read)?;

    let credit = match crate::db::user::user_password_info(conn, user_id)? {
        None => return Err(Error::new(ErrorKind::Missing, "User has no password set")),
        Some(cr) => cr,
    };

    Ok(Json(credit))
}

pub fn user_password_create(
    State(state): State<AppState>,
    session: UserSession,
    user_id: u64,
    credit: Json<Credential>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_user_write)?;

    let (hash, salt) = match (&credit.password, &credit.salt) {
        (Some(p), Some(s)) => (p, s),
        _ => return Err(Error::new(ErrorKind::Invalid, "Invalid password or salt")),
    };

    crate::db::user::user_password_create(conn, user_id, hash, salt)?;

    Ok(())
}

pub fn user_password_edit(
    State(state): State<AppState>,
    session: UserSession,
    user_id: u64,
    credit: Json<Credential>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_user_write)?;

    let (hash, salt) = match (&credit.password, &credit.salt) {
        (Some(p), Some(s)) => (p, s),
        _ => return Err(Error::new(ErrorKind::Invalid, "Invalid password or salt")),
    };

    crate::db::user::user_password_edit(conn, user_id, hash, salt)?;
    Ok(())
}

pub fn user_password_delete(State(state): State<AppState>, session: UserSession, user_id: u64) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_user_write)?;

    crate::db::user::user_password_delete(conn, user_id)?;
    Ok(())
}

use crate::common::{Credential, Right, User, WebBool};
use crate::error::{Error, ErrorKind, Result};
use crate::session::UserSession;
use crate::AppState;
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;

mod bank_account;
mod license;

pub use bank_account::*;
pub use license::*;

#[derive(Deserialize)]
pub struct UserListQuery {
    pub active: Option<WebBool>,
}

pub async fn user_list(
    State(state): State<AppState>,
    _session: UserSession,
    Query(query): Query<UserListQuery>,
) -> Result<Json<Vec<User>>> {
    let conn = &mut state.db.get_conn()?;

    let active = query.active.map(|active| active.to_bool());

    let users = crate::db::user::user_list(conn, active)?;
    Ok(Json(users))
}

pub async fn user_right(
    State(state): State<AppState>,
    session: UserSession,
    Path(user_id): Path<u64>,
) -> Result<Json<Right>> {
    if crate::permission::is_self(&session, user_id).is_ok() {
        return Ok(Json(session.right));
    }

    crate::permission::require_right(session.right.right_user_read)?;

    let conn = &mut state.db.get_conn()?;
    let right = crate::db::login::user_right(conn, user_id)?;
    Ok(Json(right))
}

pub fn user_info(State(state): State<AppState>, session: UserSession) -> Result<Json<User>> {
    let conn = &mut state.db.get_conn()?;
    let user = crate::db::user::user_info(conn, session.user.id)?;
    Ok(Json(user))
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
    crate::permission::require_right(session.right.right_user_read)
        .or(crate::permission::is_self(&session, user_id))?;

    let credit = match crate::db::user::user_password_info(conn, user_id)? {
        None => return Err(Error::new(ErrorKind::Missing, "User password is missing")),
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

// EX USER



pub fn user_password_set(State(state): State<AppState>, session: UserSession, credit: Json<Credential>) -> Result<()> {
    let conn = &mut state.db.get_conn()?;

    let (hash, salt) = match (&credit.password, &credit.salt) {
        (Some(p), Some(s)) => (p, s),
        _ => return Err(Error::new(ErrorKind::Invalid, "User password is invalid")),
    };

    crate::db::user::user_password_edit(conn, session.user.id, hash, salt)?;
    Ok(())
}

pub fn user_image(State(state): State<AppState>, user_id: u64) -> Result<Vec<u8>> {
    let conn = &mut state.db.get_conn()?;
    let image_url = crate::db::user::user_image(conn, user_id)?;

    if let Some(url) = image_url {
        let path = crate::fs::get_data_path().join(format!("users/{url}"));

        if let Ok(image) = std::fs::read(path) {
            return Ok(image);
        }
    }

    let placeholder = crate::fs::get_resources_path().join("user_image_placeholder.png");

    std::fs::read(placeholder).map_err(|_| Error::new(ErrorKind::Filesystem, "Failed to read user image"))
}

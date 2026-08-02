use crate::common::{Credential, Right, User};
use crate::error::{Error, ErrorKind, Result};
use crate::session::UserSession;
use crate::AppState;
use axum::extract::State;
use axum::Json;

/*
 * ROUTES
 */

pub fn user_info(State(state): State<AppState>, session: UserSession) -> Result<Json<User>> {
    let conn = &mut state.db.get_conn()?;
    let user = crate::db::user::user_info(conn, session.user.id)?;
    Ok(Json(user))
}

pub fn user_right(State(_state): State<AppState>, session: UserSession) -> Json<Right> {
    Json(session.right)
}

pub fn user_password_info(State(state): State<AppState>, session: UserSession) -> Result<Json<Credential>> {
    let conn = &mut state.db.get_conn()?;
    let credit = match crate::db::user::user_password_info(conn, session.user.id)? {
        None => return Err(Error::new(ErrorKind::Missing, "User password is missing")),
        Some(cr) => cr,
    };

    Ok(Json(credit))
}

pub fn user_password_set(State(state): State<AppState>, session: UserSession, credit: Json<Credential>) -> Result<()> {
    let conn = &mut state.db.get_conn()?;

    let (hash, salt) = match (&credit.password, &credit.salt) {
        (Some(p), Some(s)) => (p, s),
        _ => return Err(Error::new(ErrorKind::Invalid, "User password is invalid")),
    };

    crate::db::user::user_password_edit(conn, session.user.id, hash, salt)?;
    Ok(())
}

pub fn user_list(State(state): State<AppState>, _session: UserSession) -> Result<Json<Vec<User>>> {
    let conn = &mut state.db.get_conn()?;
    let users = crate::db::user::user_list(conn, Some(true))?;
    Ok(Json(users))
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

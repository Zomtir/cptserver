use axum::extract::State;
use axum::Json;

use crate::common::License;
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

pub fn user_license_main_create(
    State(state): State<AppState>,
    session: UserSession,
    user_id: u64,
    license: Json<License>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_user_write)?;

    crate::db::user::user_license_main_create(conn, user_id, &license)?;

    Ok(())
}

pub fn user_license_extra_create(
    State(state): State<AppState>,
    session: UserSession,
    user_id: u64,
    license: Json<License>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_user_write)?;

    crate::db::user::user_license_extra_create(conn, user_id, &license)?;

    Ok(())
}

pub fn user_license_main_edit(
    State(state): State<AppState>,
    session: UserSession,
    user_id: u64,
    license: Json<License>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_user_write)?;

    crate::db::user::user_license_main_edit(conn, user_id, &license)?;

    Ok(())
}

pub fn user_license_extra_edit(
    State(state): State<AppState>,
    session: UserSession,
    user_id: u64,
    license: Json<License>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_user_write)?;

    crate::db::user::user_license_extra_edit(conn, user_id, &license)?;

    Ok(())
}

pub fn user_license_main_delete(State(state): State<AppState>, session: UserSession, user_id: u64) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_user_write)?;

    crate::db::user::user_license_main_delete(conn, user_id)?;
    Ok(())
}

pub fn user_license_extra_delete(State(state): State<AppState>, session: UserSession, user_id: u64) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_user_write)?;

    crate::db::user::user_license_extra_delete(conn, user_id)?;
    Ok(())
}

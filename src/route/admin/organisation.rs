use axum::extract::State;
use axum::Json;

use crate::common::Organisation;
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

mod affiliation;
pub use affiliation::*;

pub fn organisation_list(State(state): State<AppState>, session: UserSession) -> Result<Json<Vec<Organisation>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_organisation_read)?;

    let organisations = crate::db::organisation::organisation_list(conn)?;
    Ok(Json(organisations))
}

pub fn organisation_info(
    State(state): State<AppState>,
    session: UserSession,
    organisation_id: u32,
) -> Result<Json<Organisation>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_organisation_read)?;

    let organisation = crate::db::organisation::organisation_info(conn, organisation_id)?;
    Ok(Json(organisation))
}

pub fn organisation_create(
    State(state): State<AppState>,
    session: UserSession,
    organisation: Json<Organisation>,
) -> Result<String> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_organisation_write)?;

    let id = crate::db::organisation::organisation_create(conn, &organisation)?;
    Ok(id.to_string())
}

pub fn organisation_edit(
    State(state): State<AppState>,
    session: UserSession,
    organisation_id: u32,
    organisation: Json<Organisation>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_organisation_write)?;

    crate::db::organisation::organisation_edit(conn, organisation_id, &organisation)?;
    Ok(())
}

pub fn organisation_delete(State(state): State<AppState>, session: UserSession, organisation_id: u32) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_organisation_write)?;

    crate::db::organisation::organisation_delete(conn, organisation_id)?;
    Ok(())
}

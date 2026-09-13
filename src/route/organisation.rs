use axum::extract::{Path, State};
use axum::Json;

use crate::common::Organisation;
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

pub async fn organisation_list(State(state): State<AppState>) -> Result<Json<Vec<Organisation>>> {
    let conn = &mut state.db.get_conn()?;
    let organisations = crate::db::organisation::organisation_list(conn)?;
    Ok(Json(organisations))
}

pub async fn organisation_info(
    State(state): State<AppState>,
    session: UserSession,
    Path(organisation_id): Path<u32>,
) -> Result<Json<Organisation>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_organisation_read)?;

    let organisation = crate::db::organisation::organisation_info(conn, organisation_id)?;
    Ok(Json(organisation))
}

pub async fn organisation_create(
    State(state): State<AppState>,
    session: UserSession,
    organisation: Json<Organisation>,
) -> Result<String> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_organisation_write)?;

    let id = crate::db::organisation::organisation_create(conn, &organisation)?;
    Ok(id.to_string())
}

pub async fn organisation_edit(
    State(state): State<AppState>,
    session: UserSession,
    Path(organisation_id): Path<u32>,
    organisation: Json<Organisation>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_organisation_write)?;

    crate::db::organisation::organisation_edit(conn, organisation_id, &organisation)?;
    Ok(())
}

pub async fn organisation_delete(
    State(state): State<AppState>,
    session: UserSession,
    Path(organisation_id): Path<u32>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_organisation_write)?;

    crate::db::organisation::organisation_delete(conn, organisation_id)?;
    Ok(())
}

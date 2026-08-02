use crate::common::Location;
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;
use axum::extract::State;
use axum::Json;

pub fn location_list(State(state): State<AppState>, session: UserSession) -> Result<Json<Vec<Location>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_location_read)?;

    let locations = crate::db::location::location_list(conn)?;
    Ok(Json(locations))
}

pub fn location_create(
    State(state): State<AppState>,
    session: UserSession,
    location: Json<Location>,
) -> Result<String> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_location_write)?;

    let id = crate::db::location::location_create(conn, &location)?;
    Ok(id.to_string())
}

pub fn location_edit(
    State(state): State<AppState>,
    session: UserSession,
    location_id: u32,
    location: Json<Location>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_location_write)?;

    crate::db::location::location_edit(conn, location_id, &location)?;
    Ok(())
}

pub fn location_delete(State(state): State<AppState>, session: UserSession, location_id: u32) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_location_write)?;

    crate::db::location::location_delete(conn, location_id)?;
    Ok(())
}

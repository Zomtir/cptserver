use crate::common::Team;
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;
use axum::extract::State;
use axum::Json;

/* ROUTES */

pub fn team_list(State(state): State<AppState>, _session: UserSession) -> Result<Json<Vec<Team>>> {
    let conn = &mut state.db.get_conn()?;
    let teams = crate::db::team::team_list(conn)?;
    Ok(Json(teams))
}

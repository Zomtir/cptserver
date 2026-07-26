use axum::Json;

use crate::common::Team;
use crate::error::Result;
use crate::session::UserSession;

/* ROUTES */

pub fn team_list(_session: UserSession) -> Result<Json<Vec<Team>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let teams = crate::db::team::team_list(conn)?;
    Ok(Json(teams))
}

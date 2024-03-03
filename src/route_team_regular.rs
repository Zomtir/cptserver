use rocket::serde::json::Json;

use crate::common::Team;
use crate::error::Error;
use crate::session::UserSession;

/* ROUTES */

#[rocket::get("/regular/team_list")]
pub fn team_list(_session: UserSession) -> Result<Json<Vec<Team>>, Error> {
    let teams = crate::db_team::list_teams()?;
    Ok(Json(teams))
}

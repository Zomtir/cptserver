use rocket::serde::json::Json;

use crate::common::{Team, User};
use crate::error::Error;
use crate::session::UserSession;

/* ROUTES */

#[rocket::get("/member/team_list")]
pub fn team_list(session: UserSession) -> Result<Json<Vec<Team>>, Error> {
    match crate::db_team::list_teams()? {
        teams => Ok(Json(teams)),
    }
}

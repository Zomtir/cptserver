use rocket::serde::json::Json;

use crate::common::{Branch, Ranking};
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/member/ranking_list")]
pub fn ranking_list(session: UserSession) -> Result<Json<Vec<Ranking>>, Error> {
    match crate::db_ranking::list_rankings(Some(session.user.id), None, 0, 10)? {
        rankings => Ok(Json(rankings)),
    }
}

#[rocket::get("/member/ranking_summary")]
pub fn ranking_summary(session: UserSession) -> Result<Json<Vec<(Branch, i16)>>, Error> {
    match crate::db_ranking::summarize_rankings(session.user.id)? {
        summary => Ok(Json(summary)),
    }
}

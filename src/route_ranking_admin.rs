use rocket::serde::json::Json;

use crate::api::ApiError;
use crate::session::{UserSession};
use crate::common::{Ranking};

#[rocket::get("/admin/ranking_list?<user_id>&<branch_id>&<min>&<max>")]
pub fn ranking_list(session: UserSession, user_id: Option<i64>, branch_id: Option<i64>, min: Option<i16>, max: Option<i16>) -> Result<Json<Vec<Ranking>>, ApiError> {
    if !session.right.admin_rankings {return Err(ApiError::RIGHT_NO_RANKINGS)};

    match crate::db_ranking::list_rankings(user_id, branch_id, min.unwrap_or(0), max.unwrap_or(10)) {
        None => Err(ApiError::DB_CONFLICT),
        Some(rankings) => Ok(Json(rankings)),
    }
}

#[rocket::post("/admin/ranking_create", format = "application/json", data = "<ranking>")]
pub fn ranking_create(session: UserSession, ranking: Json<Ranking>) -> Result<String, ApiError> {
    if !session.right.admin_rankings {return Err(ApiError::RIGHT_NO_RANKINGS)};

    match crate::db_ranking::create_ranking(&ranking) {
        None => Err(ApiError::DB_CONFLICT),
        Some(id) => Ok(id.to_string()),
    }
}

#[rocket::post("/admin/ranking_edit?<ranking_id>", format = "application/json", data = "<ranking>")]
pub fn ranking_edit(session: UserSession, ranking_id: i64, ranking: Json<Ranking>) -> Result<(), ApiError> {
    if !session.right.admin_rankings {return Err(ApiError::RIGHT_NO_RANKINGS)};

    match crate::db_ranking::edit_ranking(ranking_id, &ranking) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}

#[rocket::head("/admin/ranking_delete?<ranking_id>")]
pub fn ranking_delete(session: UserSession, ranking_id: i64) -> Result<(), ApiError> {
    if !session.right.admin_rankings {return Err(ApiError::RIGHT_NO_RANKINGS)};

    match crate::db_ranking::delete_ranking(ranking_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}

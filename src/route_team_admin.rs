use rocket::serde::json::Json;

use crate::api::ApiError;
use crate::session::UserSession;
use crate::common::{Team, User};

/* ROUTES */

#[rocket::get("/admin/team_list")]
pub fn team_list(session: UserSession) -> Result<Json<Vec<Team>>, ApiError> {
    if !session.right.admin_teams {return Err(ApiError::RIGHT_NO_TEAM)};

    match crate::db_team::list_teams() {
        None => Err(ApiError::DB_CONFLICT),
        Some(teams) => Ok(Json(teams)),
    }
}

#[rocket::post("/admin/team_create", format = "application/json", data = "<team>")]
pub fn team_create(session: UserSession, team: Json<Team>) -> Result<String, ApiError> {
    if !session.right.admin_teams {return Err(ApiError::RIGHT_NO_TEAM)};

    match crate::db_team::create_team(&team) {
        None => Err(ApiError::DB_CONFLICT),
        Some(team_id) => Ok(team_id.to_string()),
    }
}

#[rocket::post("/admin/team_edit?<team_id>", format = "application/json", data = "<team>")]
pub fn team_edit(session: UserSession, team_id: u32, team: Json<Team>) -> Result<(), ApiError> {
    if !session.right.admin_teams {return Err(ApiError::RIGHT_NO_TEAM)};

    match crate::db_team::edit_team(&team_id, &team) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}

#[rocket::head("/admin/team_delete?<team_id>")]
pub fn team_delete(session: UserSession, team_id: u32) -> Result<(), ApiError> {
    if !session.right.admin_teams {return Err(ApiError::RIGHT_NO_TEAM)};

    match crate::db_team::delete_team(&team_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}

#[rocket::get("/admin/team_member_list?<team_id>")]
pub fn team_member_list(session: UserSession, team_id: u32) -> Result<Json<Vec<User>>, ApiError> {
    if !session.right.admin_teams {return Err(ApiError::RIGHT_NO_TEAM)};

    match crate::db_team::list_team_members(team_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(users) => Ok(Json(users)),
    }
}

#[rocket::head("/admin/team_member_add?<team_id>&<user_id>")]
pub fn team_member_add(session: UserSession, team_id: u32, user_id: u32) -> Result<(), ApiError> {
    if !session.right.admin_teams {return Err(ApiError::RIGHT_NO_TEAM)};

    match crate::db_team::add_team_member(&team_id, &user_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }

    // TODO: remove/add permissions of currently logged-in users
}

#[rocket::head("/admin/team_member_remove?<team_id>&<user_id>")]
pub fn team_member_remove(session: UserSession, team_id: u32, user_id: u32) -> Result<(), ApiError> {
    if !session.right.admin_teams {return Err(ApiError::RIGHT_NO_TEAM)};

    match crate::db_team::remove_team_member(&team_id, &user_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }

    // TODO: remove/add permissions of currently logged-in users
}

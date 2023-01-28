use rocket::serde::json::Json;

use crate::api::ApiError;
use crate::session::UserSession;
use crate::common::{User, Right};

/*
 * ROUTES
 */

#[rocket::get("/member/user_info")]
pub fn user_info(session: UserSession) -> Json<User> {
    Json(User::from_info(
        session.user.id,
        session.user.key,
        session.user.firstname,
        session.user.lastname,
    ))
}

#[rocket::get("/member/user_right")]
pub fn user_right(session: UserSession) -> Json<Right> {
    Json(Right{
        admin_courses: session.right.admin_courses,
        admin_inventory: session.right.admin_inventory,
        admin_rankings: session.right.admin_rankings,
        admin_event: session.right.admin_event,
        admin_teams: session.right.admin_teams,
        admin_term: session.right.admin_term,
        admin_users: session.right.admin_users,
    })
}

#[rocket::post("/member/user_password", format = "text/plain", data = "<password>")]
pub fn user_password(session: UserSession, password: String) -> Result<(), ApiError> {
    match crate::db_user::edit_user_password(session.user.id, password) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}

// TODO only active members
#[rocket::get("/member/user_list")]
pub fn user_list(_session: UserSession) -> Result<Json<Vec<User>>,ApiError> {
    match crate::db_user::list_user(Some(true)) {
        None => Err(ApiError::DB_CONFLICT),
        Some(users) => Ok(Json(users)),
    }
}

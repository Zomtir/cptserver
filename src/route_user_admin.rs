use rocket::serde::json::Json;

use crate::api::ApiError;
use crate::session::{UserSession};
use crate::common::{User};

/* ROUTES */

#[rocket::get("/admin/user_list?<enabled>")]
pub fn user_list(session: UserSession, enabled: Option<bool>) -> Result<Json<Vec<User>>, ApiError> {
    if !session.right.admin_users {return Err(ApiError::RIGHT_NO_USER)};

    match crate::db_user::list_user(enabled) {
        None => Err(ApiError::DB_CONFLICT),
        Some(users) => Ok(Json(users)),
    }
}

#[rocket::post("/admin/user_create", format = "application/json", data = "<user>")]
pub fn user_create(session: UserSession, user: Json<User>) -> Result<String, ApiError> {
    if !session.right.admin_users {return Err(ApiError::RIGHT_NO_USER)};

    match crate::db_user::create_user(&user) {
        None => Err(ApiError::DB_CONFLICT),
        Some(user_id) => Ok(user_id.to_string()),
    }
}

#[rocket::post("/admin/user_edit?<user_id>", format = "application/json", data = "<user>")]
pub fn user_edit(session: UserSession, user_id: u32, user: Json<User>) -> Result<(),ApiError> {
    if !session.right.admin_users {return Err(ApiError::RIGHT_NO_USER)};

    match crate::db_user::edit_user(user_id, &user) {
        None => Err(ApiError::DB_CONFLICT),
        Some(()) => Ok(()),
    }
}

#[rocket::post("/admin/user_edit_password?<user_id>", format = "text/plain", data = "<password>")]
pub fn user_edit_password(session: UserSession, user_id: u32, password: String) -> Result<(), ApiError> {
    if !session.right.admin_users {return Err(ApiError::RIGHT_NO_USER)};

    match crate::db_user::edit_user_password(user_id, password) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}

#[rocket::head("/admin/user_delete?<user_id>")]
pub fn user_delete(user_id: u32, session: UserSession) -> Result<(),ApiError> {
    if !session.right.admin_users {return Err(ApiError::RIGHT_NO_USER)};

    match crate::db_user::delete_user(&user_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(()) => Ok(()),
    }
}

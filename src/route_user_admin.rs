use rocket::serde::json::Json;

use crate::api::{ApiError};
use crate::session::{Credential, UserSession};
use crate::common::{User};

/* ROUTES */

#[rocket::get("/admin/user_list?<enabled>")]
pub fn user_list(session: UserSession, enabled: Option<bool>) -> Result<Json<Vec<User>>, ApiError> {
    if !session.right.admin_users {return Err(ApiError::RIGHT_NO_USER)};

    let users = crate::db_user::list_user(enabled)?;
    Ok(Json(users))
}

#[rocket::get("/admin/user_detailed?<user_id>")]
pub fn user_detailed(session: UserSession, user_id: i64) -> Result<Json<User>, ApiError>{
    if !session.right.admin_users {return Err(ApiError::RIGHT_NO_USER)};

    let user = crate::db_user::get_user_detailed(user_id)?;
    Ok(Json(user))
}

#[rocket::post("/admin/user_create", format = "application/json", data = "<user>")]
pub fn user_create(session: UserSession, mut user: Json<User>) -> Result<String, ApiError> {
    if !session.right.admin_users {return Err(ApiError::RIGHT_NO_USER)};

    let user_id = crate::db_user::create_user(&mut user)?;
    Ok(user_id.to_string())
}

#[rocket::post("/admin/user_edit?<user_id>", format = "application/json", data = "<user>")]
pub fn user_edit(session: UserSession, user_id: i64, mut user: Json<User>) -> Result<(),ApiError> {
    if !session.right.admin_users {return Err(ApiError::RIGHT_NO_USER)};

    crate::db_user::edit_user(user_id, &mut user)?;
    Ok(())
}

#[rocket::post("/admin/user_edit_password?<user_id>", format = "application/json", data = "<credit>")]
pub fn user_edit_password(session: UserSession, user_id: i64, credit: Json<Credential>) -> Result<(), ApiError> {
    if !session.right.admin_users {return Err(ApiError::RIGHT_NO_USER)};

    match crate::db_user::edit_user_password(user_id, &credit.password, &credit.salt) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}

#[rocket::head("/admin/user_delete?<user_id>")]
pub fn user_delete(session: UserSession, user_id: i64) -> Result<(),ApiError> {
    if !session.right.admin_users {return Err(ApiError::RIGHT_NO_USER)};

    match crate::db_user::delete_user(user_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(()) => Ok(()),
    }
}

use rocket::serde::json::Json;

use crate::common::User;
use crate::error::Error;
use crate::session::{Credential, UserSession};

/* ROUTES */

#[rocket::get("/admin/user_list?<enabled>")]
pub fn user_list(session: UserSession, enabled: Option<bool>) -> Result<Json<Vec<User>>, Error> {
    if !session.right.admin_users {
        return Err(Error::RightUserMissing);
    };

    let users = crate::db_user::list_user(enabled)?;
    Ok(Json(users))
}

#[rocket::get("/admin/user_detailed?<user_id>")]
pub fn user_detailed(session: UserSession, user_id: i64) -> Result<Json<User>, Error> {
    if !session.right.admin_users {
        return Err(Error::RightUserMissing);
    };

    let user = crate::db_user::get_user_detailed(user_id)?;
    Ok(Json(user))
}

#[rocket::post("/admin/user_create", format = "application/json", data = "<user>")]
pub fn user_create(session: UserSession, mut user: Json<User>) -> Result<String, Error> {
    if !session.right.admin_users {
        return Err(Error::RightUserMissing);
    };

    let user_id = crate::db_user::create_user(&mut user)?;

    Ok(user_id.to_string())
}

#[rocket::post("/admin/user_edit?<user_id>", format = "application/json", data = "<user>")]
pub fn user_edit(session: UserSession, user_id: i64, mut user: Json<User>) -> Result<(), Error> {
    if !session.right.admin_users {
        return Err(Error::RightUserMissing);
    };

    crate::db_user::edit_user(user_id, &mut user)?;
    Ok(())
}

#[rocket::post(
    "/admin/user_edit_password?<user_id>",
    format = "application/json",
    data = "<credit>"
)]
pub fn user_edit_password(session: UserSession, user_id: i64, credit: Json<Credential>) -> Result<(), Error> {
    if !session.right.admin_users {
        return Err(Error::RightUserMissing);
    };

    crate::db_user::edit_user_password(user_id, &credit.password, &credit.salt)?;
    Ok(())
}

#[rocket::head("/admin/user_delete?<user_id>")]
pub fn user_delete(session: UserSession, user_id: i64) -> Result<(), Error> {
    if !session.right.admin_users {
        return Err(Error::RightUserMissing);
    };

    crate::db_user::delete_user(user_id)?;
    Ok(())
}

use rocket::serde::json::Json;

use crate::common::{User, WebBool};
use crate::error::Error;
use crate::session::{Credential, UserSession};

/* ROUTES */

#[rocket::get("/admin/user_list?<active>")]
pub fn user_list(session: UserSession, active: Option<WebBool>) -> Result<Json<Vec<User>>, Error> {
    if !session.right.right_user_read {
        return Err(Error::RightUserMissing);
    };

    let users = crate::db::user::user_list(active.map(|b| b.to_bool()))?;
    Ok(Json(users))
}

#[rocket::get("/admin/user_detailed?<user_id>")]
pub fn user_detailed(session: UserSession, user_id: u64) -> Result<Json<User>, Error> {
    if !session.right.right_user_read {
        return Err(Error::RightUserMissing);
    };

    let user = crate::db::user::user_info(user_id)?;
    Ok(Json(user))
}

#[rocket::post("/admin/user_create", format = "application/json", data = "<user>")]
pub fn user_create(session: UserSession, mut user: Json<User>) -> Result<String, Error> {
    if !session.right.right_user_write {
        return Err(Error::RightUserMissing);
    };

    let user_id = crate::db::user::user_create(&mut user)?;

    Ok(user_id.to_string())
}

#[rocket::post("/admin/user_edit?<user_id>", format = "application/json", data = "<user>")]
pub fn user_edit(session: UserSession, user_id: u64, mut user: Json<User>) -> Result<(), Error> {
    if !session.right.right_user_write {
        return Err(Error::RightUserMissing);
    };

    crate::db::user::user_edit(user_id, &mut user)?;
    Ok(())
}

#[rocket::post(
    "/admin/user_edit_password?<user_id>",
    format = "application/json",
    data = "<credit>"
)]
pub fn user_edit_password(session: UserSession, user_id: u64, credit: Json<Credential>) -> Result<(), Error> {
    if !session.right.right_user_write {
        return Err(Error::RightUserMissing);
    };

    crate::db::user::user_password_edit(user_id, &credit.password, &credit.salt)?;
    Ok(())
}

#[rocket::head("/admin/user_delete?<user_id>")]
pub fn user_delete(session: UserSession, user_id: u64) -> Result<(), Error> {
    if !session.right.right_user_write {
        return Err(Error::RightUserMissing);
    };

    crate::db::user::user_delete(user_id)?;
    Ok(())
}

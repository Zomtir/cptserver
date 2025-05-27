use rocket::serde::json::Json;

use crate::common::{Credential, User, WebBool};
use crate::error::{ErrorKind, Result};
use crate::session::UserSession;

/* ROUTES */

#[rocket::get("/admin/user_list?<active>")]
pub fn user_list(session: UserSession, active: Option<WebBool>) -> Result<Json<Vec<User>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_user_read {
        return Err(ErrorKind::RightUserMissing);
    };

    let users = crate::db::user::user_list(conn, active.map(|b| b.to_bool()))?;
    Ok(Json(users))
}

#[rocket::get("/admin/user_detailed?<user_id>")]
pub fn user_detailed(session: UserSession, user_id: u64) -> Result<Json<User>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_user_read {
        return Err(ErrorKind::RightUserMissing);
    };

    let user = crate::db::user::user_detailed(conn, user_id)?;
    Ok(Json(user))
}

#[rocket::post("/admin/user_create", format = "application/json", data = "<user>")]
pub fn user_create(session: UserSession, mut user: Json<User>) -> Result<String> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_user_write {
        return Err(ErrorKind::RightUserMissing);
    };

    let user_id = crate::db::user::user_create(conn, &mut user)?;

    Ok(user_id.to_string())
}

#[rocket::post("/admin/user_edit?<user_id>", format = "application/json", data = "<user>")]
pub fn user_edit(session: UserSession, user_id: u64, mut user: Json<User>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_user_write {
        return Err(ErrorKind::RightUserMissing);
    };

    crate::db::user::user_edit(conn, user_id, &mut user)?;
    Ok(())
}

#[rocket::head("/admin/user_delete?<user_id>")]
pub fn user_delete(session: UserSession, user_id: u64) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_user_write {
        return Err(ErrorKind::RightUserMissing);
    };

    crate::db::user::user_delete(conn, user_id)?;
    Ok(())
}

#[rocket::get("/admin/user_password_info?<user_id>")]
pub fn user_password_info(session: UserSession, user_id: u64) -> Result<Json<Credential>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_user_read {
        return Err(ErrorKind::RightUserMissing);
    };

    let credit = match crate::db::user::user_password_info(conn, user_id)? {
        None => return Err(ErrorKind::UserPasswordMissing),
        Some(cr) => cr,
    };

    Ok(Json(credit))
}

#[rocket::post(
    "/admin/user_password_create?<user_id>",
    format = "application/json",
    data = "<credit>"
)]
pub fn user_password_create(session: UserSession, user_id: u64, credit: Json<Credential>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_user_write {
        return Err(ErrorKind::RightUserMissing)?;
    };

    let (hash, salt) = match (&credit.password, &credit.salt) {
        (Some(p), Some(s)) => (p, s),
        _ => return Err(ErrorKind::UserPasswordInvalid)?,
    };

    crate::db::user::user_password_create(conn, user_id, hash, salt)?;

    Ok(())
}

#[rocket::post(
    "/admin/user_password_edit?<user_id>",
    format = "application/json",
    data = "<credit>"
)]
pub fn user_password_edit(session: UserSession, user_id: u64, credit: Json<Credential>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_user_write {
        return Err(ErrorKind::RightUserMissing);
    };

    let (hash, salt) = match (&credit.password, &credit.salt) {
        (Some(p), Some(s)) => (p, s),
        _ => return Err(ErrorKind::UserPasswordInvalid),
    };

    crate::db::user::user_password_edit(conn, user_id, hash, salt)?;
    Ok(())
}

#[rocket::head("/admin/user_password_delete?<user_id>")]
pub fn user_password_delete(session: UserSession, user_id: u64) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_user_write {
        return Err(ErrorKind::RightUserMissing);
    };

    crate::db::user::user_password_delete(conn, user_id)?;
    Ok(())
}

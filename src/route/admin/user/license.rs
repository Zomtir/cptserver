use rocket::serde::json::Json;

use crate::common::License;
use crate::error::{ErrorKind, Result};
use crate::session::UserSession;

/* ROUTES */

#[rocket::post(
    "/admin/user_license_main_create?<user_id>",
    format = "application/json",
    data = "<license>"
)]
pub fn user_license_main_create(session: UserSession, user_id: u64, license: Json<License>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_user_write {
        return Err(ErrorKind::RightUserMissing);
    };

    crate::db::user::user_license_main_create(conn, user_id, &license)?;

    Ok(())
}

#[rocket::post(
    "/admin/user_license_extra_create?<user_id>",
    format = "application/json",
    data = "<license>"
)]
pub fn user_license_extra_create(session: UserSession, user_id: u64, license: Json<License>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_user_write {
        return Err(ErrorKind::RightUserMissing);
    };

    crate::db::user::user_license_extra_create(conn, user_id, &license)?;

    Ok(())
}

#[rocket::post(
    "/admin/user_license_main_edit?<user_id>",
    format = "application/json",
    data = "<license>"
)]
pub fn user_license_main_edit(session: UserSession, user_id: u64, license: Json<License>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_user_write {
        return Err(ErrorKind::RightUserMissing);
    };

    crate::db::user::user_license_main_edit(conn, user_id, &license)?;

    Ok(())
}

#[rocket::post(
    "/admin/user_license_extra_edit?<user_id>",
    format = "application/json",
    data = "<license>"
)]
pub fn user_license_extra_edit(session: UserSession, user_id: u64, license: Json<License>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_user_write {
        return Err(ErrorKind::RightUserMissing);
    };

    crate::db::user::user_license_extra_edit(conn, user_id, &license)?;

    Ok(())
}

#[rocket::head("/admin/user_license_main_delete?<user_id>")]
pub fn user_license_main_delete(session: UserSession, user_id: u64) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_user_write {
        return Err(ErrorKind::RightUserMissing);
    };

    crate::db::user::user_license_main_delete(conn, user_id)?;
    Ok(())
}

#[rocket::head("/admin/user_license_extra_delete?<user_id>")]
pub fn user_license_extra_delete(session: UserSession, user_id: u64) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_user_write {
        return Err(ErrorKind::RightUserMissing);
    };

    crate::db::user::user_license_extra_delete(conn, user_id)?;
    Ok(())
}

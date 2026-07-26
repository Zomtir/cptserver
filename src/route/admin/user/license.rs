use axum::Json;

use crate::common::License;
use crate::error::Result;
use crate::session::UserSession;

pub fn user_license_main_create(session: UserSession, user_id: u64, license: Json<License>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_user_write)?;

    crate::db::user::user_license_main_create(conn, user_id, &license)?;

    Ok(())
}

pub fn user_license_extra_create(session: UserSession, user_id: u64, license: Json<License>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_user_write)?;

    crate::db::user::user_license_extra_create(conn, user_id, &license)?;

    Ok(())
}

pub fn user_license_main_edit(session: UserSession, user_id: u64, license: Json<License>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_user_write)?;

    crate::db::user::user_license_main_edit(conn, user_id, &license)?;

    Ok(())
}

pub fn user_license_extra_edit(session: UserSession, user_id: u64, license: Json<License>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_user_write)?;

    crate::db::user::user_license_extra_edit(conn, user_id, &license)?;

    Ok(())
}

pub fn user_license_main_delete(session: UserSession, user_id: u64) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_user_write)?;

    crate::db::user::user_license_main_delete(conn, user_id)?;
    Ok(())
}

pub fn user_license_extra_delete(session: UserSession, user_id: u64) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_user_write)?;

    crate::db::user::user_license_extra_delete(conn, user_id)?;
    Ok(())
}

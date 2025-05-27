use rocket::serde::json::Json;

use crate::common::BankAccount;
use crate::error::{ErrorKind, Result};
use crate::session::UserSession;

/* ROUTES */

#[rocket::post(
    "/admin/user_bank_account_create?<user_id>",
    format = "application/json",
    data = "<bank_account>"
)]
pub fn user_bank_account_create(session: UserSession, user_id: u64, bank_account: Json<BankAccount>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_user_write {
        return Err(ErrorKind::RightUserMissing);
    };

    crate::db::user::user_bank_account_create(conn, user_id, &bank_account)?;

    Ok(())
}

#[rocket::post(
    "/admin/user_bank_account_edit?<user_id>",
    format = "application/json",
    data = "<bank_account>"
)]
pub fn user_bank_account_edit(session: UserSession, user_id: u64, bank_account: Json<BankAccount>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_user_write {
        return Err(ErrorKind::RightUserMissing);
    };

    crate::db::user::user_bank_account_edit(conn, user_id, &bank_account)?;

    Ok(())
}

#[rocket::head("/admin/user_bank_account_delete?<user_id>")]
pub fn user_bank_account_delete(session: UserSession, user_id: u64) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_user_write {
        return Err(ErrorKind::RightUserMissing);
    };

    crate::db::user::user_bank_account_delete(conn, user_id)?;
    Ok(())
}

use axum::Json;

use crate::common::BankAccount;
use crate::error::Result;
use crate::session::UserSession;

pub fn user_bank_account_create(session: UserSession, user_id: u64, bank_account: Json<BankAccount>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_user_write)?;

    crate::db::user::user_bank_account_create(conn, user_id, &bank_account)?;

    Ok(())
}

pub fn user_bank_account_edit(session: UserSession, user_id: u64, bank_account: Json<BankAccount>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_user_write)?;

    crate::db::user::user_bank_account_edit(conn, user_id, &bank_account)?;

    Ok(())
}

pub fn user_bank_account_delete(session: UserSession, user_id: u64) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_user_write)?;

    crate::db::user::user_bank_account_delete(conn, user_id)?;
    Ok(())
}

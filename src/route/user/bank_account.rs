use axum::extract::{Path, Query, State};
use axum::Json;

use crate::common::BankAccount;
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

pub async fn user_bank_account_create(
    State(state): State<AppState>,
    session: UserSession,
    Path(user_id): Path<u64>,
    bank_account: Json<BankAccount>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_user_write)?;

    crate::db::user::user_bank_account_create(conn, user_id, &bank_account)?;

    Ok(())
}

pub async fn user_bank_account_edit(
    State(state): State<AppState>,
    session: UserSession,
    Path(user_id): Path<u64>,
    bank_account: Json<BankAccount>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_user_write)?;

    crate::db::user::user_bank_account_edit(conn, user_id, &bank_account)?;

    Ok(())
}

pub async fn user_bank_account_delete(
    State(state): State<AppState>,
    session: UserSession,
    Path(user_id): Path<u64>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_user_write)?;

    crate::db::user::user_bank_account_delete(conn, user_id)?;
    Ok(())
}

use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::BankAccount;
use crate::db::get_pool_conn;
use crate::error::Error;

pub fn user_bank_account_create(user_id: u64, bank_account: &BankAccount) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();

    let stmt = conn.prep(
        "INSERT INTO bank_accounts (iban, bic, institute)
        VALUES (:iban, :bic, :institute);",
    )?;

    let params = params! {
        "user_id" => &user_id,
        "iban" => &bank_account.iban,
        "bic" => &bank_account.bic,
        "institute" => &bank_account.institute,
    };

    conn.exec_drop(&stmt, &params)?;

    let stmt_user = conn.prep(
        "UPDATE users
        SET bank_account = :bank_account_id
        WHERE users.user_id = :user_id;",
    )?;

    let params_user = params! {
        "user_id" => &user_id,
        "bank_account_id" => &conn.last_insert_id(),
    };

    conn.exec_drop(&stmt_user, &params_user)?;

    Ok(())
}

pub fn user_bank_account_edit(user_id: u64, bank_account: &BankAccount) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE bank_accounts
        JOIN users ON users.bank_account = bank_accounts.id
        SET
            bank_accounts.iban = :iban,
            bank_accounts.bic = :bic,
            bank_accounts.institute = :institute
        WHERE user_id = :user_id;",
    )?;

    let params = params! {
        "user_id" => &user_id,
        "iban" => &bank_account.iban,
        "bic" => &bank_account.bic,
        "institute" => &bank_account.institute,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn user_bank_account_delete(user_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE bank_accounts FROM bank_accounts
        JOIN users ON users.bank_account = bank_accounts.id
        WHERE users.user_id = :user_id;",
    )?;

    let params = params! {
        "user_id" => user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

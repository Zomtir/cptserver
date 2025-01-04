use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::License;
use crate::db::get_pool_conn;
use crate::error::Error;

pub fn user_license_main_create(user_id: u64, license: &License) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();

    let stmt = conn.prep(
        "INSERT INTO licenses (number, name, expiration, file_url)
        VALUES (:number, :name, :expiration, :file_url);",
    )?;

    let params = params! {
        "user_id" => &user_id,
        "number" => &license.number,
        "name" => &license.name,
        "expiration" => &license.expiration,
        "file_url" => &license.file_url,
    };

    conn.exec_drop(&stmt, &params)?;

    let stmt_user = conn.prep(
        "UPDATE users
        SET license_main = :license_id
        WHERE users.user_id = :user_id;",
    )?;

    let params_user = params! {
        "user_id" => &user_id,
        "license_id" => &conn.last_insert_id(),
        "file_url" => &license.file_url,
    };

    conn.exec_drop(&stmt_user, &params_user)?;

    Ok(())
}

pub fn user_license_extra_create(user_id: u64, license: &License) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();

    let stmt = conn.prep(
        "INSERT INTO licenses (number, name, expiration, file_url)
        VALUES (:number, :name, :expiration, :file_url);",
    )?;

    let params = params! {
        "user_id" => &user_id,
        "number" => &license.number,
        "name" => &license.name,
        "expiration" => &license.expiration,
    };

    conn.exec_drop(&stmt, &params)?;

    let stmt_user = conn.prep(
        "UPDATE users
        SET license_extra = :license_id
        WHERE users.user_id = :user_id;",
    )?;

    let params_user = params! {
        "user_id" => &user_id,
        "license_id" => &conn.last_insert_id(),
        "file_url" => &license.file_url,
    };

    conn.exec_drop(&stmt_user, &params_user)?;

    Ok(())
}

pub fn user_license_main_edit(user_id: u64, license: &License) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE licenses
        JOIN users ON users.license_main = licenses.id
        SET
            licenses.number = :number,
            licenses.name = :name,
            licenses.expiration = :expiration,
            licenses.file_url = :file_url
        WHERE user_id = :user_id;",
    )?;

    let params = params! {
        "user_id" => &user_id,
        "number" => &license.number,
        "name" => &license.name,
        "expiration" => &license.expiration,
        "file_url" => &license.file_url,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn user_license_extra_edit(user_id: u64, license: &License) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE licenses
        JOIN users ON users.license_extra = licenses.id
        SET
            licenses.number = :number,
            licenses.name = :name,
            licenses.expiration = :expiration,
            licenses.file_url = :file_url
        WHERE users.user_id = :user_id;",
    )?;

    let params = params! {
        "user_id" => &user_id,
        "number" => &license.number,
        "name" => &license.name,
        "expiration" => &license.expiration,
        "file_url" => &license.file_url,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn user_license_main_delete(user_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE licenses FROM licenses
        JOIN users ON users.license_main = licenses.id
        WHERE users.user_id = :user_id;",
    )?;

    let params = params! {
        "user_id" => user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn user_license_extra_delete(user_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE licenses FROM licenses
        JOIN users ON users.license_extra = licenses.id
        WHERE users.user_id = :user_id;",
    )?;

    let params = params! {
        "user_id" => user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::User;
use crate::db::get_pool_conn;
use crate::error::Error;

/*
 * METHODS
 */

pub fn user_list(active: Option<bool>) -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT user_id, user_key, firstname, lastname, nickname
        FROM users
        WHERE :active IS NULL OR :active = active;",
    );

    let params = params! {
        "active" => &active,
    };

    let map = |(user_id, user_key, firstname, lastname, nickname)| {
        User::from_info(user_id, user_key, firstname, lastname, nickname)
    };

    Ok(conn.exec_map(&stmt.unwrap(), &params, &map)?)
}

pub fn user_info(user_id: u64) -> Result<User, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT
            users.user_id,
            user_key,
            enabled,
            active,
            firstname,
            lastname,
            nickname,
            address,
            email,
            phone,
            iban,
            birthday,
            birthlocation,
            nationality,
            gender,
            federationnumber,
            federationpermissionsolo,
            federationpermissionteam,
            federationresidency,
            datadeclaration,
            datadisclaimer,
            note
        FROM users
        WHERE users.user_id = :user_id;",
    )?;

    let params = params! {
        "user_id" => &user_id,
    };

    let mut row: mysql::Row = match conn.exec_first(&stmt, &params)? {
        None => return Err(Error::UserMissing),
        Some(row) => row,
    };

    let user = User {
        id: row.take("user_id").unwrap(),
        key: row.take("user_key").unwrap(),
        enabled: row.take("enabled").unwrap(),
        active: row.take("active").unwrap(),
        firstname: row.take("firstname").unwrap(),
        lastname: row.take("lastname").unwrap(),
        nickname: row.take("nickname").unwrap(),
        address: row.take("address").unwrap(),
        email: row.take("email").unwrap(),
        phone: row.take("phone").unwrap(),
        iban: row.take("iban").unwrap(),
        birthday: row.take("birthday").unwrap(),
        birthlocation: row.take("birthlocation").unwrap(),
        nationality: row.take("nationality").unwrap(),
        gender: row.take("gender").unwrap(),
        federationnumber: row.take("federationnumber").unwrap(),
        federationpermissionsolo: row.take("federationpermissionsolo").unwrap(),
        federationpermissionteam: row.take("federationpermissionteam").unwrap(),
        federationresidency: row.take("federationresidency").unwrap(),
        datadeclaration: row.take("datadeclaration").unwrap(),
        datadisclaimer: row.take("datadisclaimer").unwrap(),
        note: row.take("note").unwrap(),
    };

    Ok(user)
}

pub fn user_create(user: &mut User) -> Result<u64, Error> {
    user.key = match crate::common::check_user_key(&user.key) {
        Err(e) => Some(crate::common::random_string(6)),
        Ok(key) => Some(key),
    };

    user.email = crate::common::check_user_email(&user.email).ok();

    let mut conn: PooledConn = get_pool_conn();

    let stmt = conn.prep(
        "INSERT INTO users (user_key, pwd, pepper, salt, enabled, active, firstname, lastname, nickname,
        address, email, phone, iban, birthday, birthlocation, nationality, gender,
        federationnumber, federationpermissionsolo, federationpermissionteam, federationresidency,
        datadeclaration, datadisclaimer, note)
    VALUES (:user_key, :pwd, :pepper, :salt, :enabled, :active, :firstname, :lastname, :nickname,
        :address, :email, :phone, :iban, :birthday, :birthlocation, :nationality, :gender,
        :federationnumber, :federationpermissionsolo, :federationpermissionteam, :federationresidency,
        :datadeclaration, :datadisclaimer, :note);",
    )?;

    let params = params! {
        "user_key" => &user.key,
        "pwd" => crate::common::random_string(10),
        "pepper" => crate::common::random_bytes(16),
        "salt" => crate::common::random_bytes(16),
        "enabled" => user.enabled.unwrap_or(false),
        "active" => user.active.unwrap_or(true),
        "firstname" => &user.firstname,
        "lastname" => &user.lastname,
        "nickname" => &user.nickname,
        "address" => &user.address,
        "email" => &user.email,
        "phone" => &user.phone,
        "iban" => &user.iban,
        "birthday" => &user.birthday,
        "birthlocation" => &user.birthlocation,
        "nationality" => &user.nationality,
        "gender" => &user.gender,
        "federationnumber" => &user.federationnumber,
        "federationpermissionsolo" => &user.federationpermissionsolo,
        "federationpermissionteam" => &user.federationpermissionteam,
        "federationresidency" => &user.federationresidency,
        "datadeclaration" => &user.datadeclaration,
        "datadisclaimer" => &user.datadisclaimer,
        "note" => &user.note,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(conn.last_insert_id() as u64)
}

pub fn user_edit(user_id: u64, user: &mut User) -> Result<(), Error> {
    crate::common::check_user_key(&user.key)?;
    user.email = crate::common::check_user_email(&user.email).ok();

    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE users SET
        user_key = ?,
        enabled = ?,
        active = ?,
        firstname = ?,
        lastname = ?,
        nickname = ?,
        address = ?,
        email = ?,
        phone = ?,
        iban = ?,
        birthday = ?,
        birthlocation = ?,
        nationality = ?,
        gender = ?,
        federationnumber = ?,
        federationpermissionsolo = ?,
        federationpermissionteam = ?,
        federationresidency = ?,
        datadeclaration = ?,
        datadisclaimer = ?,
        note = ?
        WHERE user_id = ?;",
    )?;

    let params: Vec<mysql::Value> = vec![
        user.key.clone().into(),
        user.enabled.into(),
        user.active.into(),
        user.firstname.clone().into(),
        user.lastname.clone().into(),
        user.nickname.clone().into(),
        user.address.clone().into(),
        user.email.clone().into(),
        user.phone.clone().into(),
        user.iban.clone().into(),
        user.birthday.into(),
        user.birthlocation.clone().into(),
        user.nationality.clone().into(),
        user.gender.clone().clone().into(),
        user.federationnumber.into(),
        user.federationpermissionsolo.into(),
        user.federationpermissionteam.into(),
        user.federationresidency.into(),
        user.datadeclaration.into(),
        user.datadisclaimer.clone().into(),
        user.note.clone().into(),
        user_id.into(),
    ];

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn user_password_edit(user_id: u64, password: &str, salt: &str) -> Result<(), Error> {
    let bpassword: Vec<u8> = match crate::common::decode_hash256(password) {
        Some(bpassword) => bpassword,
        None => return Err(Error::UserPasswordInvalid),
    };

    let bsalt: Vec<u8> = match crate::common::decode_hash128(salt) {
        Some(bsalt) => bsalt,
        None => return Err(Error::UserPasswordInvalid),
    };

    let bpepper: Vec<u8> = crate::common::random_bytes(16);
    let shapassword: Vec<u8> = crate::common::hash_sha256(&bpassword, &bpepper);

    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("UPDATE users SET pwd = :pwd, pepper = :pepper, salt = :salt WHERE user_id = :user_id")?;
    let params = params! {
        "user_id" => &user_id,
        "pwd" => &shapassword,
        "pepper" => &bpepper,
        "salt" => &bsalt,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn user_delete(user_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE u FROM users u WHERE u.user_id = :user_id")?;
    let params = params! {
        "user_id" => user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn user_created_true(user_key: &str) -> Result<bool, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT COUNT(1) FROM users WHERE user_key = :user_key")?;
    let params = params! { "user_key" => user_key };
    let count: Option<i32> = conn.exec_first(&stmt, &params)?;

    Ok(count.unwrap() == 1)
}

pub fn user_salt_value(user_key: &str) -> Result<Vec<u8>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT salt FROM users WHERE user_key = :user_key")?;
    let params = params! {
        "user_key" => &user_key
    };

    // If the user does not exist, just return a random salt to prevent data scraping
    match conn.exec_first::<Vec<u8>, _, _>(&stmt, &params)? {
        None => Err(Error::UserMissing),
        Some(salt) => Ok(salt),
    }
}

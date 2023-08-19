use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::User;
use crate::db::get_pool_conn;
use crate::error::Error;

/*
 * METHODS
 */

pub fn list_user(active: Option<bool>) -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT user_id, user_key, firstname, lastname
        FROM users
        WHERE :active IS NULL OR :active = active;",
    );

    let params = params! {
        "active" => &active,
    };

    let map = |(user_id, user_key, firstname, lastname)| User::from_info(user_id, user_key, firstname, lastname);

    Ok(conn.exec_map(&stmt.unwrap(), &params, &map)?)
}

pub fn get_user_detailed(user_id: i64) -> Result<User, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT
            users.user_id,
            user_key,
            enabled,
            active,
            firstname,
            lastname,
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

pub fn create_user(user: &mut User) -> Result<i64, Error> {
    user.key = crate::common::validate_user_key(&user.key)?;
    user.email = crate::common::validate_email(&user.email)?;

    let mut conn: PooledConn = get_pool_conn();

    let stmt = conn.prep(
    "INSERT INTO users (user_key, pwd, pepper, salt, enabled, active, firstname, lastname,
        address, email, phone, iban, birthday, birthlocation, nationality, gender,
        federationnumber, federationpermissionsolo, federationpermissionteam, federationresidency,
        datadeclaration, datadisclaimer, note)
    VALUES (:user_key, :pwd, :pepper, :salt, :enabled, :active, :firstname, :lastname,
        :address, :email, :phone, :iban, :birthday, :birthlocation, :nationality, :gender,
        :federationnumber, :federationpermissionsolo, :federationpermissionteam, :federationresidency,
        :datadeclaration, :datadisclaimer, :note);")?;

    let params = params! {
        "user.key" => crate::common::random_string(6),
        "pwd" => crate::common::random_string(10),
        "pepper" => crate::common::random_bytes(16), 
        "salt" => crate::common::random_bytes(16),
        "enabled" => user.enabled.unwrap_or(false),
        "active" => user.active.unwrap_or(true),
        "firstname" => &user.firstname,
        "lastname" => &user.lastname,
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

    Ok(conn.last_insert_id() as i64)
}

pub fn is_user_created(user_key: &str) -> Result<bool, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT COUNT(1) FROM users WHERE user_key = :user_key")?;
    let params = params! { "user_key" => user_key };
    let count: Option<i32> = conn.exec_first(&stmt, &params)?;

    return Ok(count.unwrap() == 1);
}

pub fn edit_user(user_id: i64, user: &mut User) -> Result<(), Error> {
    user.key = crate::common::validate_user_key(&user.key)?;

    if user.key.is_none() {
        return Err(Error::UserMissing);
    };

    user.email = crate::common::validate_email(&user.email)?;

    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE users SET
        user_key = ?,
        enabled = ?,
        active = ?,
        firstname = ?,
        lastname = ?,
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

    let mut params = Vec::<mysql::Value>::with_capacity(20);

    params.push(user.key.clone().into());
    params.push(user.enabled.clone().into());
    params.push(user.active.clone().into());
    params.push(user.firstname.clone().into());
    params.push(user.lastname.clone().into());
    params.push(user.address.clone().into());
    params.push(user.email.clone().into());
    params.push(user.phone.clone().into());
    params.push(user.iban.clone().into());
    params.push(user.birthday.clone().into());
    params.push(user.birthlocation.clone().into());
    params.push(user.nationality.clone().into());
    params.push(user.gender.clone().clone().into());
    params.push(user.federationnumber.clone().into());
    params.push(user.federationpermissionsolo.clone().into());
    params.push(user.federationpermissionteam.clone().into());
    params.push(user.federationresidency.clone().into());
    params.push(user.datadeclaration.clone().into());
    params.push(user.datadisclaimer.clone().into());
    params.push(user.note.clone().into());
    params.push(user_id.clone().into());

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn edit_user_password(user_id: i64, password: &String, salt: &String) -> Result<(), Error> {
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

pub fn delete_user(user_id: i64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE u FROM users u WHERE u.user_id = :user_id")?;
    let params = params! {
        "user_id" => user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::User;
use crate::db::get_pool_conn;
use crate::error::CptError;

/*
 * METHODS
 */

pub fn list_user(enabled: Option<bool>) -> Result<Vec<User>, CptError> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT user_id, user_key, firstname, lastname
        FROM users
        WHERE :enabled IS NULL OR :enabled = enabled;");

    let params = params! {
        "enabled" => &enabled,
    };

    let map = |(user_id, user_key, firstname, lastname)| {
        User::from_info(user_id, user_key, firstname, lastname)
    };

    Ok(conn.exec_map(&stmt.unwrap(), &params, &map)?)
}

pub fn get_user_detailed(user_id: i64) -> Result<User,CptError> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT
            users.user_id,
            user_key,
            enabled,
            firstname,
            lastname,
            address,
            email,
            phone,
            iban,
            birthday,
            birthlocation,
            nationality
            gender,
            federationNumber,
            federationPermissionSolo,
            federationPermissionTeam,
            federationResidency,
            dataDeclaration,
            dataDisclaimer,
            note
        FROM users
        WHERE users.user_id = :user_id;")?;

    let params = params! {
        "user_id" => &user_id,
    };

    let mut row : mysql::Row = match conn.exec_first(&stmt,&params)? {
        None => return Err(CptError::UserMissing),
        Some(row) => row,
    };

    let user = User{
        id: row.take("user_id").unwrap(),
        key: row.take("user_key").unwrap(),
        enabled: row.take("user_key").unwrap(),
        firstname: row.take("user_key").unwrap(),
        lastname: row.take("user_key").unwrap(),
        address: row.take("address").unwrap(),
        email: row.take("email").unwrap(),
        phone: row.take("phone").unwrap(),
        iban: row.take("iban").unwrap(),
        birthday: row.take("birthday").unwrap(),
        birthlocation: row.take("birthlocation").unwrap(),
        nationality: row.take("nationality").unwrap(),
        gender: row.take("gender").unwrap(),
        federationNumber: row.take("federationNumber").unwrap(),
        federationPermissionSolo: row.take("federationPermissionSolo").unwrap(),
        federationPermissionTeam: row.take("federationPermissionTeam").unwrap(),
        federationResidency: row.take("federationResidency").unwrap(),
        dataDeclaration: row.take("dataDeclaration").unwrap(),
        dataDisclaimer: row.take("dataDisclaimer").unwrap(),
        note: row.take("note").unwrap(),
    };

    Ok(user)
}

pub fn create_user(user: &mut User) -> Result<i64, CptError> {
    user.key = crate::common::validate_user_key(&user.key)?;
    user.email = crate::common::validate_email(&user.email)?;

    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO users (user_key, pwd, pepper, salt, enabled, firstname, lastname,
            address, email, phone, iban, birthday, birthlocation, nationality, gender,
            federationNumber, federationPermissionSolo, federationPermissionTeam, federationResidency,
            dataDeclaration, dataDisclaimer, note)
        VALUES (:user_key, :pwd, :pepper, :salt, :enabled, :firstname, :lastname,
            :address, :email, :phone, :iban, :birthday, :birthlocation, :nationality, :gender,
            :federationNumber, :federationPermissionSolo, :federationPermissionTeam, :federationResidency,
            :dataDeclaration, :dataDisclaimer, :note);"
    );
    let params = params! {
        "user_key" => &user.key.as_ref().unwrap_or(&crate::common::random_string(6)),
        "pwd" => crate::common::random_string(10),
        "pepper" => crate::common::random_bytes(16),
        "salt" => crate::common::random_bytes(16),
        "enabled" => user.enabled.unwrap_or(false),
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
        "federationNumber" => &user.federationNumber,
        "federationPermissionSolo" => &user.federationPermissionSolo,
        "federationPermissionTeam" => &user.federationPermissionTeam,
        "federationResidency" => &user.federationResidency,
        "dataDeclaration" => &user.dataDeclaration,
        "dataDisclaimer" => &user.dataDisclaimer,
        "note" => &user.note,
    };

    conn.exec_drop(&stmt.unwrap(), &params)?;

    let user_id = crate::db::get_last_id(&mut conn)?;

    Ok(user_id)
}

pub fn is_user_created(user_key: &str) -> Option<bool> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT COUNT(1) FROM users WHERE user_key = :user_key");
    let params = params! { "user_key" => user_key };
    let count: Option<i32> = conn.exec_first(&stmt.unwrap(), &params).ok()?;

    return Some(count.unwrap() == 1);
}

pub fn edit_user(user_id: i64, user: &mut User) -> Result<(),CptError> {
    user.key = crate::common::validate_user_key(&user.key)?;

    if user.key.is_none() {
        return Err(CptError::UserKeyMissing);
    };

    user.email = crate::common::validate_email(&user.email)?;
    
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE users SET
        user_key = :user_key,
        enabled = :enabled,
        firstname = :firstname,
        lastname = :lastname,
        address = :address,
        email = :email,
        phone = :phone,
        iban = :iban,
        birthday = :birthday,
        birthlocation = :birthlocation,
        nationality = :nationality,
        gender = :gender,
        federationNumber = :federationNumber,
        federationPermissionSolo = :federationPermissionSolo,
        federationPermissionTeam = :federationPermissionTeam,
        federationResidency = :federationResidency,
        dataDeclaration = :dataDeclaration,
        dataDisclaimer = :dataDisclaimer,
        note = note:
        WHERE user_id = :user_id;",
    );
    let params = params! {
        "user_id" => &user_id,
        "user_key" => &user.key,
        "enabled" => &user.enabled,
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
        "federationNumber" => &user.federationNumber,
        "federationPermissionSolo" => &user.federationPermissionSolo,
        "federationPermissionTeam" => &user.federationPermissionTeam,
        "federationResidency" => &user.federationResidency,
        "dataDeclaration" => &user.dataDeclaration,
        "dataDisclaimer" => &user.dataDisclaimer,
        "note" => &user.note,
    };

    conn.exec_drop(&stmt?, &params)?;
    Ok(())
}

pub fn edit_user_password(user_id: i64, password: &String, salt: &String) -> Option<()> {
    let bpassword: Vec<u8> = match crate::common::decode_hash256(password) {
        Some(bpassword) => bpassword,
        None => return None,
    };

    let bsalt: Vec<u8> = match crate::common::decode_hash128(salt) {
        Some(bsalt) => bsalt,
        None => return None,
    };

    let bpepper: Vec<u8> = crate::common::random_bytes(16);
    let shapassword: Vec<u8> = crate::common::hash_sha256(&bpassword, &bpepper);

    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("UPDATE users SET pwd = :pwd, pepper = :pepper, salt = :salt WHERE user_id = :user_id");
    let params = params! {
        "user_id" => &user_id,
        "pwd" => &shapassword,
        "pepper" => &bpepper,
        "salt" => &bsalt,
    };

    match conn.exec_drop(&stmt.unwrap(), &params) {
        Err(..) => None,
        Ok(..) => Some(()),
    }
}

pub fn delete_user(user_id: i64) -> Option<()> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE u FROM users u WHERE u.user_id = :user_id");
    let params = params! {
        "user_id" => user_id,
    };

    match conn.exec_drop(&stmt.unwrap(), &params) {
        Err(..) => None,
        Ok(..) => Some(()),
    }
}

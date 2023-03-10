use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::User;
use crate::db::get_pool_conn;
use crate::error::CptError;

/*
 * METHODS
 */

pub fn list_user(enabled: Option<bool>) -> Option<Vec<User>> {
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

    match conn.exec_map(&stmt.unwrap(), &params, &map) {
        Err(..) => None,
        Ok(users) => Some(users),
    }
}

pub fn get_user_detailed(user_id: i64) -> Option<User> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT
            users.user_id,
            user_key,
            enabled,
            firstname,
            lastname,
            email,
            phone,
            iban,
            birthday,
            gender,
            organization_id,
            mediapermission
        FROM users
        LEFT JOIN user_detail ON user_detail.user_id = users.user_id
        WHERE users.user_id = :user_id;");

    let params = params! {
        "user_id" => &user_id,
    };

    let map = |(user_id, user_key, enabled, firstname, lastname,
                email, phone, iban, birthday, gender, organization_id, mediapermission)| {
        User{
            id: user_id, key: user_key, enabled, firstname, lastname,
            address: None, email, phone, iban, birthday, gender, organization_id, mediapermission
        }
    };

    match conn.exec_map(&stmt.unwrap(), &params, &map) {
        Err(..) => None,
        Ok(mut users) => Some(users.remove(0)),
    }
}

pub fn create_user(user: &mut User) -> Result<i64, CptError> {
    user.key = crate::common::validate_user_key(&user.key)?;
    user.email = crate::common::validate_email(&user.email)?;

    let mut conn: PooledConn = get_pool_conn();
    let stmt1 = conn.prep(
        "INSERT INTO users (user_key, pwd, pepper, salt, firstname, lastname, enabled)
        VALUES (:user_key, :pwd, :pepper, :salt, :firstname, :lastname, :enabled);"
    );
    let params1 = params! {
        "user_key" => &user.key.as_ref().unwrap_or(&crate::common::random_string(6)),
        "pwd" => crate::common::random_string(10),
        "pepper" => crate::common::random_bytes(16),
        "salt" => crate::common::random_bytes(16),
        "firstname" => &user.firstname,
        "lastname" => &user.lastname,
        "enabled" => user.enabled.unwrap_or(false),
    };

    conn.exec_drop(&stmt1.unwrap(), &params1)?;

    let user_id = crate::db::get_last_id(&mut conn)?;

    let stmt2 = conn.prep(  
        "INSERT INTO user_detail (user_id, email, phone, iban, birthday, gender, organization_id)
        VALUES (:user_id, :email, :phone, :iban, :birthday, :gender, :organization_id);",
    );

    let params2 = params! {
        "user_id" => &user_id,
        "email" => &user.email,
        "phone" => &user.phone,
        "iban" => &user.iban,
        "birthday" => &user.birthday,
        "gender" => &user.gender,
        "organization_id" => &user.organization_id,
    };

    conn.exec_drop(&stmt2.unwrap(), &params2)?;

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
    let stmt1 = conn.prep(
        "UPDATE users SET
        user_key = :user_key,
        firstname = :firstname,
        lastname = :lastname,
        enabled = :enabled
        WHERE user_id = :user_id;",
    );
    let stmt2 = conn.prep(
        "INSERT INTO user_detail (user_id, email, phone, iban, birthday, gender, organization_id)
        VALUES (:user_id, :email, :phone, :iban, :birthday, :gender, :organization_id)
        ON DUPLICATE KEY UPDATE
            user_id = :user_id,
            email = :email,
            phone = :phone,
            iban = :iban,
            birthday = :birthday,
            gender = :gender,
            organization_id = :organization_id;",
    );
    let params = params! {
        "user_id" => &user_id,
        "user_key" => &user.key,
        "firstname" => &user.firstname,
        "lastname" => &user.lastname,
        "enabled" => &user.enabled,
        "email" => &user.email,
        "phone" => &user.phone,
        "iban" => &user.iban,
        "birthday" => &user.birthday,
        "gender" => &user.gender,
        "organization_id" => &user.organization_id,
    };

    conn.exec_drop(&stmt1?, &params)?;
    conn.exec_drop(&stmt2?, &params)?;
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

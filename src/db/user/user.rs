use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{BankAccount, License, User};
use crate::error::Error;

pub fn user_list(conn: &mut PooledConn, active: Option<bool>) -> Result<Vec<User>, Error> {
    let stmt = conn.prep(
        "SELECT user_id, user_key, firstname, lastname, nickname
        FROM users
        WHERE :active IS NULL OR :active = active;",
    )?;

    let params = params! {
        "active" => &active,
    };

    let map = |(user_id, user_key, firstname, lastname, nickname)| {
        User::from_info(user_id, user_key, firstname, lastname, nickname)
    };

    let users = conn.exec_map(&stmt, &params, &map)?;
    Ok(users)
}

pub fn user_info(conn: &mut PooledConn, user_id: u64) -> Result<User, Error> {
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
            birth_date,
            birth_location,
            nationality,
            gender,
            height,
            weight,
            image_url,
            ba.id AS ba_id,
            ba.iban AS ba_iban,
            ba.bic AS ba_bic,
            ba.institute AS ba_institute,
            lm.id AS license_main_id,
            lm.number AS license_main_number,
            lm.name AS license_main_name,
            lm.expiration AS license_main_expiration,
            lm.file_url AS license_main_file_url,
            le.id AS license_extra_id,
            le.number AS license_extra_number,
            le.name AS license_extra_name,
            le.expiration AS license_extra_expiration,
            le.file_url AS license_extra_file_url,
            note
        FROM users
        LEFT JOIN bank_accounts ba ON users.bank_account = ba.id
        LEFT JOIN licenses lm ON users.license_main = lm.id
        LEFT JOIN licenses le ON users.license_extra = le.id
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
        birth_date: row.take("birth_date").unwrap(),
        birth_location: row.take("birth_location").unwrap(),
        nationality: row.take("nationality").unwrap(),
        gender: row.take("gender").unwrap(),
        height: row.take("height").unwrap(),
        weight: row.take("weight").unwrap(),
        image_url: row.take("image_url").unwrap(),
        bank_account: row.take::<Option<u32>, &str>("ba_id").unwrap().map(|id| BankAccount {
            id,
            iban: row.take("ba_iban").unwrap(),
            bic: row.take("ba_bic").unwrap(),
            institute: row.take("ba_institute").unwrap(),
        }),
        license_main: row
            .take::<Option<u32>, &str>("license_main_id")
            .unwrap()
            .map(|id| License {
                id,
                number: row.take("license_main_number").unwrap(),
                name: row.take("license_main_name").unwrap(),
                expiration: row.take("license_main_expiration").unwrap(),
                file_url: row.take("license_main_file_url").unwrap(),
            }),
        license_extra: row
            .take::<Option<u32>, &str>("license_extra_id")
            .unwrap()
            .map(|id| License {
                id,
                number: row.take("license_extra_number").unwrap(),
                name: row.take("license_extra_name").unwrap(),
                expiration: row.take("license_extra_expiration").unwrap(),
                file_url: row.take("license_extra_file_url").unwrap(),
            }),
        note: row.take("note").unwrap(),
    };

    Ok(user)
}

pub fn user_create(conn: &mut PooledConn, user: &mut User) -> Result<u64, Error> {
    user.key = match crate::common::check_user_key(&user.key) {
        Err(_) => Some(crate::common::random_string(6)),
        Ok(key) => Some(key),
    };

    user.email = crate::common::check_user_email(&user.email).ok();

    let stmt = conn.prep(
        "INSERT INTO users (user_key, pwd, pepper, salt, enabled, active, firstname, lastname, nickname,
        address, email, phone, birth_date, birth_location, nationality, gender, height, weight, image_url,
        note)
    VALUES (:user_key, :pwd, :pepper, :salt, :enabled, :active, :firstname, :lastname, :nickname,
        :address, :email, :phone, :birth_date, :birth_location, :nationality, :gender, :height, :weight, :image_url,
        :note);",
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
        "birth_date" => &user.birth_date,
        "birth_location" => &user.birth_location,
        "nationality" => &user.nationality,
        "gender" => &user.gender,
        "height" => &user.height,
        "weight" => &user.weight,
        "image_url" => &user.image_url,
        "note" => &user.note,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(conn.last_insert_id())
}

pub fn user_edit(conn: &mut PooledConn, user_id: u64, user: &mut User) -> Result<(), Error> {
    crate::common::check_user_key(&user.key)?;
    user.email = crate::common::check_user_email(&user.email).ok();

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
        birth_date = ?,
        birth_location = ?,
        nationality = ?,
        gender = ?,
        height = ?,
        weight = ?,
        image_url = ?,
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
        user.birth_date.into(),
        user.birth_location.clone().into(),
        user.nationality.clone().into(),
        user.gender.clone().into(),
        user.height.into(),
        user.weight.into(),
        user.image_url.clone().into(),
        user.note.clone().into(),
        user_id.into(),
    ];

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn user_password_edit(conn: &mut PooledConn, user_id: u64, password: &str, salt: &str) -> Result<(), Error> {
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

pub fn user_delete(conn: &mut PooledConn, user_id: u64) -> Result<(), Error> {
    let stmt = conn.prep("DELETE u FROM users u WHERE u.user_id = :user_id")?;
    let params = params! {
        "user_id" => user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn user_created_true(conn: &mut PooledConn, user_key: &str) -> Result<bool, Error> {
    let stmt = conn.prep("SELECT COUNT(1) FROM users WHERE user_key = :user_key")?;
    let params = params! { "user_key" => user_key };
    let count: Option<i32> = conn.exec_first(&stmt, &params)?;

    Ok(count.unwrap() == 1)
}

pub fn user_salt_value(conn: &mut PooledConn, user_key: &str) -> Result<Vec<u8>, Error> {
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

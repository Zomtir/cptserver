use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use crate::db::get_pool_conn;
use crate::common::{User};

/*
 * METHODS
 */

pub fn list_user() -> Option<Vec<User>> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT user_id, user_key, firstname, lastname FROM users").unwrap();
    let map = |(user_id, user_key, firstname, lastname)| {
        User::from_info( user_id, user_key, firstname, lastname )
    };

    match conn.exec_map(&stmt,params::Params::Empty,&map) {
        Err(..) => None,
        Ok(users) => Some(users),
    }
}

pub fn create_user(user : &User) -> Option<u32> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("INSERT INTO users (user_key, pwd, firstname, lastname, enabled)
                        VALUES (:user_key, :pwd, :firstname, :lastname, :enabled)").unwrap();
    let params = params! {
        "user_key" => crate::common::random_string(6),
        "pwd" => crate::common::random_string(10),
        "firstname" => &user.firstname,
        "lastname" => &user.lastname,
        "enabled" => user.enabled,
    };

    match conn.exec_drop(&stmt,&params) {
        Err(..) => return None,
        Ok(..) => (),
    };

    let stmt_id = conn.prep("SELECT LAST_INSERT_ID()").unwrap();

    match conn.exec_first::<u32,_,_>(&stmt_id,params::Params::Empty) {
        Err(..) | Ok(None) => None,
        Ok(Some(user_id)) => Some(user_id),
    }
}

pub fn is_user_created(user_key: & str) -> Option<bool> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT COUNT(1) FROM users WHERE user_key = :user_key").ok()?;
    let count : Option<i32> = conn.exec_first(&stmt, params! { "user_key" => user_key }).ok()?;

    return Some(count.unwrap() == 1);
}

pub fn edit_user(user_id : &u32, user : &User) -> Option<()> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("UPDATE users SET
        user_key = :user_key,
        firstname = :firstname,
        lastname = :lastname,
        enabled = :enabled
        WHERE user_id = :user_id").unwrap();
    let params = params! {
        "user_id" => &user_id,
        "user_key" => &user.key,
        "firstname" => &user.firstname,
        "lastname" => &user.lastname,
        "enabled" => &user.enabled,
    };

    match conn.exec_drop(&stmt,&params) {
        Err(..) => return None,
        Ok(..) => ()
    };

    let bpassword : Vec<u8> = match &user.pwd {
        Some(password) => match crate::common::verify_password(&password){
            Some(bpassword) => bpassword,
            None => return Some(()),
        },
        None => return Some(()),
    };

    let pepper : Vec<u8> = crate::common::random_bytes(16);
    let shapassword : Vec<u8> = crate::common::hash_sha256(&bpassword, &pepper);
    
    let stmt_pwd = conn.prep("UPDATE users SET pwd = :pwd, pepper = :pepper WHERE user_id = :user_id").unwrap();
    let params_pwd = params! {
        "user_id" => &user.id,
        "pwd" => &shapassword,
        "pepper" => &pepper,
    };

    match conn.exec_drop(&stmt_pwd,&params_pwd) {
        Err(..) => None,
        Ok(..) => Some(()),
    }
}

pub fn delete_user(user_id : &u32) -> Option<()> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE u FROM users u WHERE u.user_id = :user_id").unwrap();
    let params = params! {
        "user_id" => user_id,
    };
    
    match conn.exec_drop(&stmt,&params) {
        Err(..) => None,
        Ok(..) => Some(()),
    }
}



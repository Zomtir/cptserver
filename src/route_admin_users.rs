use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::session::{POOL, UserSession, User, random_string, random_bytes, verify_password, hash_sha256};

/* ROUTES */

#[get("/user_list")]
pub fn user_list(session: UserSession) -> Result<Json<Vec<User>>, Status> {
    if !session.user.admin_users {return Err(Status::Unauthorized)};

    let mut conn : PooledConn = POOL.clone().get_conn().unwrap();
    let stmt = conn.prep("SELECT user_id, user_key, firstname, lastname FROM users").unwrap();
    let map = |(user_id, user_key, firstname, lastname)| {
        User::info( user_id, user_key, firstname, lastname )
    };

    match conn.exec_map(&stmt,mysql::params::Params::Empty,&map) {
        Err(..) => Err(Status::InternalServerError),
        Ok(users) => Ok(Json(users)),
    }
}

#[post("/user_create", format = "application/json", data = "<user>")]
pub fn user_create(user: Json<User>, session: UserSession) -> Result<String, Status>{
    if !session.user.admin_users {return Err(Status::Unauthorized)};

    let mut conn : PooledConn = POOL.clone().get_conn().unwrap();
    let stmt = conn.prep("INSERT INTO users (user_key, pwd, firstname, lastname, enabled)
                          VALUES (:user_key, :pwd, :firstname, :lastname, :enabled)").unwrap();
    let params = mysql::params! {
        "user_key" => random_string(6),
        "pwd" => random_string(10),
        "firstname" => &user.firstname,
        "lastname" => &user.lastname,
        "enabled" => user.enabled,
    };

    match conn.exec::<String,_,_>(&stmt,&params) {
        Err(..) => return Err(Status::BadRequest),
        Ok(..) => (),
    };

    let stmt_id = conn.prep("SELECT LAST_INSERT_ID()").unwrap();

    match conn.exec_first::<u32,_,_>(&stmt_id,params::Params::Empty) {
        Err(..) | Ok(None) => return Err(Status::Conflict),
        Ok(Some(user_id)) => return Ok(user_id.to_string()),
    };
}

#[post("/user_edit", format = "application/json", data = "<user>")]
pub fn user_edit(user: Json<User>, session: UserSession) -> Status {
    if !session.user.admin_users {return Status::Unauthorized};

    let mut conn : PooledConn = POOL.clone().get_conn().unwrap();
    let stmt = conn.prep("UPDATE users SET
        user_key = :user_key,
        firstname = :firstname,
        lastname = :lastname,
        enabled = :enabled
        WHERE user_id = :user_id").unwrap();
    let params = mysql::params! {
        "user_id" => &user.id,
        "user_key" => &user.key,
        "firstname" => &user.firstname,
        "lastname" => &user.lastname,
        "enabled" => &user.enabled,
    };

    match conn.exec::<String,_,_>(&stmt,&params) {
        Err(..) => return Status::InternalServerError,
        Ok(..) => ()
    };

    let bpassword : Vec<u8> = match &user.pwd {
        Some(password) => match verify_password(&password){
            Some(bpassword) => bpassword,
            None => return Status::Ok,
        },
        None => return Status::Ok,
    };

    let pepper : Vec<u8> = random_bytes(16);
    let shapassword : Vec<u8> = hash_sha256(&bpassword, &pepper);
    
    let stmt_pwd = conn.prep("UPDATE users SET pwd = :pwd, pepper = :pepper WHERE user_id = :user_id").unwrap();
    let params_pwd = mysql::params! {
        "user_id" => &user.id,
        "pwd" => &shapassword,
        "pepper" => &pepper,
    };

    match conn.exec_first::<String,_,_>(&stmt_pwd,&params_pwd) {
        Err(..) => Status::InternalServerError,
        Ok(..) => Status::Ok,
    }
}

#[head("/user_delete?<user_id>")]
pub fn user_delete(user_id: u32, session: UserSession) -> Status {
    if !session.user.admin_users {return Status::Unauthorized};

    let mut conn : PooledConn = POOL.clone().get_conn().unwrap();
    let stmt = conn.prep("DELETE u FROM users u WHERE u.user_id = :user_id").unwrap();
    let params = mysql::params! {"user_id" => user_id};

    match conn.exec::<String,_,_>(&stmt,&params) {
        Err(..) => Status::InternalServerError,
        Ok(..) => Status::Ok,
    }
}

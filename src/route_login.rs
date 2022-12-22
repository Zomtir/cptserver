use rocket::serde::json::Json;
use rocket::http::uri::Origin;
use serde::{Serialize, Deserialize};

use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use crate::db::get_pool_conn;
use crate::api::ApiError;
use crate::session::{USERSESSIONS, SLOTSESSIONS, UserSession, SlotSession};
use crate::common::{User, Right};

/* 
 * STRUCTS
 */

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credential {
    login: String,
    password: String,
}

/* 
 * METHODS
 */

#[rocket::post("/user_login", format = "application/json", data = "<credit>")]
pub fn user_login(origin: &Origin, credit: Json<Credential>) -> Result<String,ApiError> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT u.user_id, u.pwd, u.pepper, u.enabled, u.firstname, u.lastname,
                          COALESCE(MAX(admin_courses),0) AS admin_courses,
                          COALESCE(MAX(admin_inventory),0) AS admin_inventory,
                          COALESCE(MAX(admin_rankings),0) AS admin_rankings,
                          COALESCE(MAX(admin_reservations),0) AS admin_reservations,
                          COALESCE(MAX(admin_teams),0) AS admin_teams,
                          COALESCE(MAX(admin_users),0) AS admin_users
                          FROM users u
                          LEFT JOIN team_members ON (u.user_id = team_members.user_id)
                          LEFT JOIN teams ON (team_members.team_id = teams.team_id)
                          WHERE u.user_key = :user_key
                          GROUP BY u.user_id").unwrap();
    let params = params! { "user_key" => credit.login.to_string() };

    let mut row : mysql::Row = match conn.exec_first(&stmt,&params) {
        Err(..) | Ok(None) => return Err(ApiError::USER_NO_ENTRY),
        Ok(Some(row)) => row,
    };
    // TODO the response should contain what call did result in the repsonse?
    //Err(ApiError::user_missing(origin.path()))
    // TODO should the client know the difference whether an account is exisiting or disabled? 
    let user_enabled : bool = row.take("enabled").unwrap();
    if user_enabled == false {
        return Err(ApiError::USER_DISABLED);
    }

    let bpassword : Vec<u8> = match crate::common::verify_password(&credit.password){
        Some(bpassword) => bpassword,
        None => return Err(ApiError::USER_BAD_PASSWORD),
    };

    let user_pepper : Vec<u8> = row.take("pepper").unwrap();
    let user_shapwd : Vec<u8> = crate::common::hash_sha256(&bpassword, &user_pepper);

    let user_pwd : Vec<u8> = row.take("pwd").unwrap();
    if user_pwd != user_shapwd {
        return Err(ApiError::USER_BAD_PASSWORD);
    };

    let user_token : String = crate::common::random_string(30);
    let user_expiry = chrono::Utc::now() + chrono::Duration::hours(3);

    let session : UserSession = UserSession {
        token: user_token.to_string(),
        expiry: user_expiry,
        user: User{
            id: row.take("user_id").unwrap(),
            key: credit.login.to_string(),
            pwd: None,
            enabled: user_enabled,
            firstname: row.take("firstname").unwrap(),
            lastname: row.take("lastname").unwrap(),
        },
        right: Right{
            admin_courses: row.take("admin_courses").unwrap(),
            admin_inventory: row.take("admin_inventory").unwrap(),
            admin_rankings: row.take("admin_rankings").unwrap(),
            admin_reservations: row.take("admin_reservations").unwrap(),
            admin_teams: row.take("admin_teams").unwrap(),
            admin_users: row.take("admin_users").unwrap(),
        },
    };

    USERSESSIONS.lock().unwrap().insert(user_token.to_string(),session);

    return Ok(user_token);
}

#[rocket::post("/slot_login", format = "application/json", data = "<credit>")]
pub fn slot_login(credit: Json<Credential>) -> Result<String,ApiError> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT slot_id, pwd FROM slots WHERE slot_id = :slot_id").unwrap();
    let params = params! { "slot_id" => credit.login.to_string() };

    let mut row : mysql::Row = match conn.exec_first(&stmt,&params) {
        Err(..) | Ok(None) => return Err(ApiError::SLOT_NO_ENTRY),
        Ok(Some(row)) => row,
    };
    
    let slot_pwd : String = row.take("pwd").unwrap();
    if slot_pwd != credit.password {
        return Err(ApiError::SLOT_BAD_PASSWORD);
    };

    let slot_token : String = crate::common::random_string(30);
    let slot_expiry = chrono::Utc::now() + chrono::Duration::hours(3);

    let slot_id : u32 = row.take("slot_id").unwrap();
    
    let session : SlotSession = SlotSession {
        token: slot_token.to_string(),
        expiry: slot_expiry,
        slot_id: slot_id,
        slot_key: credit.login.to_string(),
    };

    SLOTSESSIONS.lock().unwrap().insert(slot_token.to_string(),session);

    return Ok(slot_token);
}

#[rocket::get("/slot_autologin?<location_id>")]
pub fn slot_autologin(location_id: u16) -> Result<String,ApiError>  {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT slot_id, pwd
                          FROM slots
                          WHERE location_id = :location_id
                          AND begin <= UTC_TIMESTAMP() AND end >= UTC_TIMESTAMP()
                          AND autologin = 1").unwrap();
    let params = params! { "location_id" => location_id };
    let map = |(slot_id, slot_pwd): (u32, String)| {
        Credential { login: slot_id.to_string(), password: slot_pwd }
    };

    let credentials = conn.exec_map(&stmt,&params,&map).unwrap();

    if credentials.len() < 1 {
        return Err(ApiError::SLOT_BAD_PASSWORD);
    };
    
    return slot_login(Json(credentials[0].clone()));
}
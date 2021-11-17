use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};

use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use crate::session::{POOL, USERSESSIONS, SLOTSESSIONS, UserSession, SlotSession, User, random_string, verify_password, hash_sha256};

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

#[post("/user_login", format = "application/json", data = "<credit>")]
pub fn user_login(credit: Json<Credential>) -> Option<String> {
    let bpassword : Vec<u8> = match verify_password(&credit.password){
        Some(bpassword) => bpassword,
        None => return None,
    };

    let mut conn : PooledConn = POOL.clone().get_conn().unwrap();
    let stmt = conn.prep("SELECT u.user_id, u.pwd, u.pepper, u.term, u.status, u.firstname, u.lastname, u.email,
                          COALESCE(MAX(admin_users),0) AS admin_users,
                          COALESCE(MAX(admin_rankings),0) AS admin_rankings,
                          COALESCE(MAX(admin_reservations),0) AS admin_reservations,
                          COALESCE(MAX(admin_courses),0) AS admin_courses
                          FROM users u
                          LEFT JOIN team_members ON (u.user_id = team_members.user_id)
                          LEFT JOIN teams ON (team_members.team_id = teams.team_id)
                          WHERE u.user_key = :user_key
                          GROUP BY u.user_id").unwrap();
    let params = mysql::params! { "user_key" => credit.login.to_string() };

    let mut row : mysql::Row = match conn.exec_first(&stmt,&params) {
        Err(..) | Ok(None) => return None,
        Ok(Some(row)) => row,
    };

    let user_pepper : Vec<u8> = row.take("pepper").unwrap();
    let user_shapwd : Vec<u8> = hash_sha256(&bpassword, &user_pepper);
    
    let user_pwd : Vec<u8> = row.take("pwd").unwrap();
    if user_pwd != user_shapwd { return None; };

    let user_term : chrono::NaiveDate = row.take("term").unwrap();
    if chrono::Date::<chrono::Utc>::from_utc(user_term, chrono::Utc) < chrono::Utc::today() { return None; }

    let user_status : String = row.take("status").unwrap();
    if user_status != "ACTIVE".to_string() { return None; }

    let user_token : String = random_string(30);
    let user_expiry = chrono::Utc::now() + chrono::Duration::hours(3);

    let session : UserSession = UserSession {
        token: user_token.to_string(),
        expiry: user_expiry,
        user: User{
            id: row.take("user_id").unwrap(),
            key: credit.login.to_string(),
            pwd: None,
            firstname: row.take("firstname").unwrap(),
            lastname: row.take("lastname").unwrap(),
            email: row.take("email").unwrap(),
            term: user_term,
            admin_users: row.take("admin_users").unwrap(),
            admin_rankings: row.take("admin_rankings").unwrap(),
            admin_reservations: row.take("admin_reservations").unwrap(),
            admin_courses: row.take("admin_courses").unwrap(),
        },
    };

    USERSESSIONS.lock().unwrap().insert(user_token.to_string(),session);

    return Some(user_token);
}

#[post("/slot_login", format = "application/json", data = "<credit>")]
pub fn slot_login(credit: Json<Credential>) -> Option<String> {
    let mut conn : PooledConn = POOL.clone().get_conn().unwrap();
    let stmt = conn.prep("SELECT slot_id, pwd FROM slots WHERE slot_id = :slot_id").unwrap();
    let params = mysql::params! { "slot_id" => credit.login.to_string() };

    let mut row : mysql::Row = match conn.exec_first(&stmt,&params) {
        Err(..) | Ok(None) => return None,
        Ok(Some(row)) => row,
    };
    
    let slot_pwd : String = row.take("pwd").unwrap();
    if slot_pwd != credit.password { return None;}

    let slot_token : String = random_string(30);
    let slot_expiry = chrono::Utc::now() + chrono::Duration::hours(3);

    let slot_id : u32 = row.take("slot_id").unwrap();
    
    let session : SlotSession = SlotSession {
        token: slot_token.to_string(),
        expiry: slot_expiry,
        slot_id: slot_id,
        slot_key: credit.login.to_string(),
    };

    SLOTSESSIONS.lock().unwrap().insert(slot_token.to_string(),session);

    return Some(slot_token);
}

#[get("/slot_autologin?<location_id>")]
pub fn slot_autologin(location_id: u16) -> Option<String> {
    let mut conn : PooledConn = POOL.clone().get_conn().unwrap();
    let stmt = conn.prep("SELECT slot_id, pwd
                          FROM slots
                          WHERE location_id = :location_id
                          AND begin <= UTC_TIMESTAMP() AND end >= UTC_TIMESTAMP()
                          AND autologin = 1").unwrap();
    let params = mysql::params! { "location_id" => location_id };
    let map = |(slot_id, slot_pwd): (u32, String)| {
        Credential { login: slot_id.to_string(), password: slot_pwd }
    };

    let credentials = conn.exec_map(&stmt,&params,&map).unwrap();

    if credentials.len() < 1 {return None};
    
    return slot_login(Json(credentials[0].clone()));
}
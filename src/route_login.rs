use rocket::serde::json::Json;

use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Right, User};
use crate::db::get_pool_conn;
use crate::error::Error;
use crate::session::{Credential, EventSession, UserSession, EVENTSESSIONS, USERSESSIONS};

#[rocket::post("/user_login", format = "application/json", data = "<credit>")]
pub fn user_login(credit: Json<Credential>) -> Result<String, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT u.user_id, u.pwd, u.pepper, u.enabled, u.firstname, u.lastname, u.nickname,
            COALESCE(MAX(admin_competence),0) AS admin_competence,
            COALESCE(MAX(admin_courses),0) AS admin_courses,
            COALESCE(MAX(admin_event),0) AS admin_event,
            COALESCE(MAX(admin_inventory),0) AS admin_inventory,
            COALESCE(MAX(admin_teams),0) AS admin_teams,
            COALESCE(MAX(admin_term),0) AS admin_term,
            COALESCE(MAX(admin_users),0) AS admin_users
        FROM users u
        LEFT JOIN team_members ON (u.user_id = team_members.user_id)
        LEFT JOIN teams ON (team_members.team_id = teams.team_id)
        WHERE u.user_key = :user_key
        GROUP BY u.user_id",
    );
    let params = params! { "user_key" => credit.login.to_string() };

    let mut row: mysql::Row = match conn.exec_first(&stmt.unwrap(), &params) {
        Err(..) | Ok(None) => return Err(Error::UserMissing),
        Ok(Some(row)) => row,
    };

    // TODO should the client know the difference whether an account is exisiting or disabled?
    let user_enabled: bool = row.take("enabled").unwrap();
    if !user_enabled {
        return Err(Error::UserDisabled);
    }

    let bpassword: Vec<u8> = match crate::common::decode_hash256(&credit.password) {
        Some(bpassword) => bpassword,
        None => return Err(Error::UserPasswordInvalid),
    };

    let user_pepper: Vec<u8> = row.take("pepper").unwrap();
    let user_shapwd: Vec<u8> = crate::common::hash_sha256(&bpassword, &user_pepper);

    println!(
        "User {} login attempt with hash {}",
        credit.login,
        hex::encode(user_shapwd.clone())
    );

    let user_pwd: Vec<u8> = row.take("pwd").unwrap();
    if user_pwd != user_shapwd {
        return Err(Error::UserLoginFail);
    };

    let user_token: String = crate::common::random_string(30);
    let user_expiry = chrono::Utc::now() + chrono::Duration::hours(3);

    let session: UserSession = UserSession {
        token: user_token.to_string(),
        expiry: user_expiry,
        user: User::from_info(
            row.take("user_id").unwrap(),
            credit.login.to_string(),
            row.take("firstname").unwrap(),
            row.take("lastname").unwrap(),
            row.take("nickname").unwrap(),
        ),
        right: Right {
            admin_competence: row.take("admin_competence").unwrap(),
            admin_courses: row.take("admin_courses").unwrap(),
            admin_event: row.take("admin_event").unwrap(),
            admin_inventory: row.take("admin_inventory").unwrap(),
            admin_teams: row.take("admin_teams").unwrap(),
            admin_term: row.take("admin_term").unwrap(),
            admin_users: row.take("admin_users").unwrap(),
        },
    };

    USERSESSIONS.lock().unwrap().insert(user_token.to_string(), session);

    Ok(user_token)
}

#[rocket::post("/event_login", format = "application/json", data = "<credit>")]
pub fn event_login(credit: Json<Credential>) -> Result<String, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn
        .prep("SELECT event_id, pwd FROM events WHERE event_key = :event_key")
        .unwrap();
    let params = params! {
        "event_key" => credit.login.to_string(),
    };

    println!("Event {} login attempt with password {}", credit.login, credit.password);
    let mut row: mysql::Row = match conn.exec_first(&stmt, &params) {
        Err(..) | Ok(None) => return Err(Error::EventMissing),
        Ok(Some(row)) => row,
    };

    let event_pwd: String = row.take("pwd").unwrap();
    if event_pwd != credit.password {
        return Err(Error::EventLoginFail);
    };

    let event_token: String = crate::common::random_string(30);
    let event_expiry = chrono::Utc::now() + chrono::Duration::hours(3);

    let event_id: u64 = row.take("event_id").unwrap();

    let session: EventSession = EventSession {
        token: event_token.to_string(),
        expiry: event_expiry,
        event_id,
        event_key: credit.login.to_string(),
    };

    EVENTSESSIONS.lock().unwrap().insert(event_token.to_string(), session);

    Ok(event_token)
}

#[rocket::get("/course_login?<course_key>")]
pub fn course_login(course_key: String) -> Result<String, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT s.event_key, s.pwd
        FROM events s
        JOIN courses c ON c.course_id = s.course_id
        WHERE c.course_key = :course_key
        AND s.begin >= :date_min AND s.end <= :date_max
        AND c.active = TRUE",
    )?;
    let params = params! {
        "course_key" => course_key,
        "date_min" => (chrono::Utc::now() - crate::config::CONFIG_SLOT_PUBLIC_LOGIN_TIME()).naive_utc(),
        "date_max" => (chrono::Utc::now() + crate::config::CONFIG_SLOT_PUBLIC_LOGIN_TIME()).naive_utc(),
    };
    let map = |(event_key, event_pwd): (String, String)| Credential {
        login: event_key.to_string(),
        password: event_pwd,
        salt: "".into(),
    };

    let credentials = conn.exec_map(&stmt, &params, &map)?;

    if credentials.is_empty() {
        return Err(Error::EventPasswordInvalid);
    };

    event_login(Json(credentials[0].clone()))
}

#[rocket::get("/location_login?<location_key>")]
pub fn location_login(location_key: String) -> Result<String, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT s.event_key, s.pwd
        FROM events s
        JOIN locations l ON l.location_id = s.location_id
        WHERE l.location_key = :location_key
        AND s.begin <= UTC_TIMESTAMP() AND s.end >= UTC_TIMESTAMP()
        AND public = 1",
    )?;
    let params = params! { "location_key" => location_key, };
    let map = |(event_key, event_pwd): (String, String)| Credential {
        login: event_key.to_string(),
        password: event_pwd,
        salt: "".into(),
    };

    let credentials = conn.exec_map(&stmt, &params, &map)?;

    if credentials.is_empty() {
        return Err(Error::EventPasswordInvalid);
    };

    event_login(Json(credentials[0].clone()))
}

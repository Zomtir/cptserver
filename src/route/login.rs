use rocket::serde::json::Json;

use crate::common::{Credential, Right};
use crate::error::{ErrorKind, Result};
use crate::session::{EventSession, UserSession, ADMINSESSION, EVENTSESSIONS, USERSESSIONS};

#[rocket::post("/user_login", format = "application/json", data = "<credit>")]
pub fn user_login(credit: Json<Credential>) -> Result<String> {
    let conn = &mut crate::utils::db::get_db_conn()?;

    let user_key: &str = match &credit.login {
        Some(key) => key,
        None => return Err(ErrorKind::UserKeyMissing),
    };

    // If the user is a preconfigured admin, return him an admin session
    if ADMINSESSION.lock().unwrap().as_deref() == Some(user_key) {
        let user_id = match crate::db::user::user_created_true(conn, user_key)? {
            Some(id) => id,
            None => return Err(ErrorKind::UserMissing),
        };
        let user = crate::db::user::user_info(conn, user_id)?;
        let adminsession = UserSession::admin(&user);
        let token = crate::common::random_string(30);
        USERSESSIONS.lock().unwrap().insert(token.clone(), adminsession);
        return Ok(token);
    }

    let user_hash: Vec<u8> = match &credit.password {
        Some(hash_string) => crate::common::decode_hash256(hash_string)?,
        None => return Err(ErrorKind::UserPasswordMissing),
    };

    let user = crate::db::login::user_login(conn, user_key, &user_hash)?;
    let session_token: String = crate::common::random_string(30);
    let session_expiry = chrono::Utc::now() + crate::config::SESSION_DURATION();

    let user_right: Right = crate::db::login::user_right(conn, user.id)?;
    let session: UserSession = UserSession {
        expiry: session_expiry,
        user,
        right: user_right,
    };

    USERSESSIONS.lock().unwrap().insert(session_token.clone(), session);

    Ok(session_token)
}

#[rocket::post("/event_login", format = "application/json", data = "<credit>")]
pub fn event_login(credit: Json<Credential>) -> Result<String> {
    let conn = &mut crate::utils::db::get_db_conn()?;

    let event_key = match &credit.login {
        None => return Err(ErrorKind::EventKeyMissing),
        Some(key) => {
            if key.is_empty() {
                return Err(ErrorKind::EventKeyInvalid);
            }
            key
        }
    };

    let event_pwd = match &credit.password {
        None => return Err(ErrorKind::EventPasswordMissing),
        Some(pwd) => {
            if pwd.is_empty() {
                return Err(ErrorKind::EventPasswordInvalid);
            }
            pwd
        }
    };

    println!("Event {} login attempt with password {}", event_key, event_pwd);

    let (event_id, event_pwd_check) = crate::db::login::event_login(conn, event_key)?;

    if *event_pwd != event_pwd_check {
        return Err(ErrorKind::EventLoginFail);
    };

    let session_token: String = crate::common::random_string(30);
    let session_expiry = chrono::Utc::now() + crate::config::SESSION_DURATION();

    let session: EventSession = EventSession {
        token: session_token.to_string(),
        expiry: session_expiry,
        event_id,
    };

    EVENTSESSIONS.lock().unwrap().insert(session_token.to_string(), session);

    Ok(session_token)
}

#[rocket::get("/course_login?<course_key>")]
pub fn course_login(course_key: String) -> Result<String> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let begin = (chrono::Utc::now() - crate::config::EVENT_LOGIN_BUFFER()).naive_utc();
    let end = (chrono::Utc::now() + crate::config::EVENT_LOGIN_BUFFER()).naive_utc();
    let (event_key, event_pwd) = crate::db::login::course_current_event(conn, &course_key, &begin, &end)?;

    let credentials = Credential {
        id: None,
        login: Some(event_key),
        password: Some(event_pwd),
        salt: None,
        since: None,
    };

    event_login(Json(credentials))
}

#[rocket::get("/location_login?<location_key>")]
pub fn location_login(location_key: String) -> Result<String> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let begin = (chrono::Utc::now() - crate::config::EVENT_LOGIN_BUFFER()).naive_utc();
    let end = (chrono::Utc::now() + crate::config::EVENT_LOGIN_BUFFER()).naive_utc();
    let (event_key, event_pwd) = crate::db::login::location_current_event(conn, &location_key, &begin, &end)?;

    let credentials = Credential {
        id: None,
        login: Some(event_key),
        password: Some(event_pwd),
        salt: None,
        since: None,
    };

    event_login(Json(credentials))
}

use rocket::serde::json::Json;

use crate::common::Right;
use crate::error::Error;
use crate::session::{Credential, EventSession, UserSession, ADMINSESSION, EVENTSESSIONS, USERSESSIONS};

#[rocket::post("/user_login", format = "application/json", data = "<credit>")]
pub fn user_login(credit: Json<Credential>) -> Result<String, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;

    let (user, user_pwd, user_pepper) = crate::db::login::user_login(conn, &credit.login.to_string())?;

    if user.key == *ADMINSESSION.lock().unwrap() {
        let adminsession = UserSession::admin(&user);
        let token = crate::common::random_string(30);
        USERSESSIONS.lock().unwrap().insert(token.clone(), adminsession);
        return Ok(token);
    }

    // For now let the client know if the user is disabled.
    // Otherwise return `Error::UserMissing` in the future.
    if !user.enabled.unwrap() {
        return Err(Error::UserDisabled);
    }

    let bpassword: Vec<u8> = match crate::common::decode_hash256(&credit.password) {
        Some(bpassword) => bpassword,
        None => return Err(Error::UserPasswordInvalid),
    };

    let user_shapwd: Vec<u8> = crate::common::hash_sha256(&bpassword, &user_pepper);

    println!(
        "User {} login attempt with hash {}",
        credit.login,
        hex::encode(user_shapwd.clone())
    );

    if user_pwd != user_shapwd {
        return Err(Error::UserLoginFail);
    };

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
pub fn event_login(credit: Json<Credential>) -> Result<String, Error> {
    println!("Event {} login attempt with password {}", credit.login, credit.password);

    let conn = &mut crate::utils::db::get_db_conn()?;
    let (event_id, event_pwd) = crate::db::login::event_login(conn, &credit.login.to_string())?;
    if event_pwd != credit.password {
        return Err(Error::EventLoginFail);
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
pub fn course_login(course_key: String) -> Result<String, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let begin = (chrono::Utc::now() - crate::config::EVENT_LOGIN_BUFFER()).naive_utc();
    let end = (chrono::Utc::now() + crate::config::EVENT_LOGIN_BUFFER()).naive_utc();
    let (event_key, event_pwd) = crate::db::login::course_current_event(conn, &course_key, &begin, &end)?;

    let credentials = Credential {
        login: event_key,
        password: event_pwd,
        salt: "".into(),
    };

    event_login(Json(credentials))
}

#[rocket::get("/location_login?<location_key>")]
pub fn location_login(location_key: String) -> Result<String, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let begin = (chrono::Utc::now() - crate::config::EVENT_LOGIN_BUFFER()).naive_utc();
    let end = (chrono::Utc::now() + crate::config::EVENT_LOGIN_BUFFER()).naive_utc();
    let (event_key, event_pwd) = crate::db::login::location_current_event(conn, &location_key, &begin, &end)?;

    let credentials = Credential {
        login: event_key,
        password: event_pwd,
        salt: "".into(),
    };

    event_login(Json(credentials))
}

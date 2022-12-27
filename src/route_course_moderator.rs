use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use rocket::http::Status;
use rocket::serde::json::Json;

use crate::api::ApiError;
use crate::db::get_pool_conn;
use crate::session::{UserSession};
use crate::common::{Course, Slot, User};

#[rocket::get("/mod/course_responsibility")]
pub fn course_responsibility(session: UserSession) -> Result<Json<Vec<Course>>,ApiError> {
    match crate::db_course::get_course_responsibility(session.user.id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(courses) => Ok(Json(courses)),
    }
}

#[rocket::get("/mod/course_moderator_list?<course_id>")]
pub fn course_moderator_list(session: UserSession, course_id: u32) -> Result<Json<Vec<User>>, ApiError> {
    let moderators = match crate::db_course::get_course_moderator_list(&course_id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(moderators) => moderators,
    };

    if !moderators.iter().any(|member| member.id == session.user.id){
        return Err(ApiError::COURSE_NO_MODERATOR);
    };

    return Ok(Json(moderators));
}

#[rocket::head("/mod/course_moderator_add?<course_id>&<user_id>")]
pub fn course_moderator_add(session: UserSession, course_id: u32, user_id: u32) -> Result<(),ApiError> {
    match crate::config::CONFIG_COURSE_MODERATOR_PROMOTION {
        false => return Err(ApiError::RIGHT_CONFLICT),
        true => (),
    }

    match crate::db_course::is_course_moderator(&course_id, &session.user.id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => return Err(ApiError::RIGHT_CONFLICT),
        Some(true) => (),
    };

    match crate::db_course::add_course_moderator(course_id, user_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}

#[rocket::head("/mod/course_moderator_remove?<course_id>&<user_id>")]
pub fn course_moderator_remove(session: UserSession, course_id: u32, user_id: u32) -> Result<(),ApiError> {
    match crate::config::CONFIG_COURSE_MODERATOR_PROMOTION {
        false => return Err(ApiError::RIGHT_CONFLICT),
        true => (),
    }

    match crate::db_course::is_course_moderator(&course_id, &session.user.id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => return Err(ApiError::RIGHT_CONFLICT),
        Some(true) => (),
    };

    match crate::db_course::remove_course_moderator(course_id, user_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}

#[rocket::get("/mod/course_class_list?<course_id>")]
pub fn course_class_list(session: UserSession, course_id: u32) -> Result<Json<Vec<Slot>>,ApiError> {
    match crate::db_course::is_course_moderator(&course_id, &session.user.id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => return Err(ApiError::RIGHT_CONFLICT),
        Some(true) => (),
    };

    match crate::db_course::get_course_class_list(course_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(slots) => Ok(Json(slots)),
    }
}

#[rocket::post("/mod/course_class_create?<course_id>", format = "application/json", data = "<slot>")]
pub fn course_class_create(session: UserSession, course_id: u32, mut slot: Json<Slot>) -> Result<String,ApiError> {
    match crate::db_course::is_course_moderator(&course_id, &session.user.id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => return Err(ApiError::RIGHT_CONFLICT),
        Some(true) => (),
    };

    crate::common::validate_slot_dates(&mut slot);

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("INSERT INTO slots (slot_key, pwd, title, status, autologin, location_id, begin, end, course_id)
                          SELECT :slot_key, :pwd, :title, :status, :autologin, :location_id, :begin, :end, m.course_id
                          FROM course_moderators m
                          WHERE m.course_id = :course_id AND m.user_id = :user_id").unwrap();

    let params = params! {
        "slot_key" => crate::common::random_string(8),
        "pwd" => crate::common::random_string(8),
        "title" => &slot.title,
        "status" => "OCCURRING",
        "autologin" => false,
        "location_id" => &slot.location.id,
        "begin" => &slot.begin,
        "end" => &slot.end,
        "course_id" => &slot.course_id,
        "user_id" => &session.user.id,
    };

    match conn.exec_drop(&stmt,&params) {
        Err(..) => return Err(ApiError::DB_CONFLICT),
        Ok(..) => (),
    };

    let stmt_id = conn.prep("SELECT LAST_INSERT_ID()").unwrap();
    let params_id = params::Params::Empty;

    match conn.exec_first::<u32,_,_>(&stmt_id, &params_id) {
        Err(..) | Ok(None) => Err(ApiError::DB_CONFLICT),
        Ok(Some(slot_id)) => Ok(slot_id.to_string()),
    }
}

#[rocket::post("/mod/course_class_edit?<course_id>", format = "application/json", data = "<slot>")]
pub fn course_class_edit(session: UserSession, course_id: u32, mut slot: Json<Slot>) -> Result<(),ApiError> {
    match crate::db_course::is_course_moderator(&course_id, &session.user.id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => return Err(ApiError::RIGHT_CONFLICT),
        Some(true) => (),
    };

    crate::common::validate_slot_dates(&mut slot);

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("UPDATE slots s, course_moderators m SET
        s.title = :title,
        s.location_id = :location_id,
        s.begin = :begin,
        s.end = :end,
        s.status = 'OCCURRING'
        WHERE (s.course_id = m.course_id) AND s.slot_id = :slot_id AND s.course_id = :course_id AND m.user_id = :user_id").unwrap();

    let params = params! {
        "slot_id" => &slot.id,
        "title" => &slot.title,
        "location_id" => &slot.location.id,
        "begin" => &slot.begin,
        "end" => &slot.end,
        "course_id" => &course_id,
        "user_id" => &session.user.id,
    };

    match conn.exec_drop(&stmt,&params) {
        Err(..) => return Err(ApiError::DB_CONFLICT),
        Ok(..) => (),
    };

    if slot.pwd.is_none() || slot.pwd.as_ref().unwrap().len() < 8 {
        return Err(ApiError::SLOT_BAD_PASSWORD);
    };

    let stmt_pwd = conn.prep("UPDATE slots s, course_moderators m SET s.pwd = :pwd
                              WHERE (s.user_id = m.user_id)
                              AND s.slot_id = :slot_id
                              AND s.course_id = :course_id
                              AND m.user_id = :user_id").unwrap();

    let params_pwd = params! {
        "slot_id" => &slot.id,
        "pwd" => &slot.pwd.as_ref().unwrap(),
        "course_id" => &slot.course_id,
        "user_id" => &session.user.id,
    };

    match conn.exec_drop(&stmt_pwd,&params_pwd) {
        Err(..) => Err(ApiError::DB_CONFLICT),
        Ok(..) => Ok(()),
    }
}

#[rocket::head("/mod/course_class_delete?<slot_id>")]
pub fn course_class_delete(session: UserSession, slot_id: u32) -> Status {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE s FROM slots s
                          JOIN course_moderators m ON s.course_id = m.course_id
                          WHERE s.slot_id = :slot_id AND m.user_id = :user_id").unwrap();
    let params = params! {
        "slot_id" => &slot_id,
        "user_id" => &session.user.id,
    };

    match conn.exec_drop(&stmt,&params) {
        Err(..) => Status::Conflict,
        Ok(..) => Status::Ok,
    }
}

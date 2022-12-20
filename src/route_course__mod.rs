use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use rocket::http::Status;
use rocket::serde::json::Json;

use crate::api::ApiError;
use crate::db::get_pool_conn;
use crate::session::{UserSession};
use crate::common::{Course, Branch, Access, Member, Slot, Location};

#[rocket::get("/mod/course_list")]
pub fn course_list(session: UserSession) -> Json<Vec<Course>> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT c.course_id, c.course_key, c.title, c.active,
                            b.branch_id, b.branch_key, b.title, c.threshold,
                            a.access_id, a.access_key, a.title
                          FROM courses c
                          JOIN branches b ON c.branch_id = b.branch_id
                          JOIN access a ON c.access_id = a.access_id
                          JOIN course_moderators m ON c.course_id = m.course_id
                          WHERE m.user_id = :user_id").unwrap();
    
    let params = params! { "user_id" => session.user.id};

    let map = |(course_id, course_key, course_title, active,
            branch_id, branch_key, branch_title, threshold,
            access_id, access_key, access_title): (u32, String, String, bool, u16, String, String, u8, u8, String, String)|
        Course {
            id: course_id, key: course_key, title: course_title, active,
            branch: Branch{id: branch_id, key: branch_key, title: branch_title}, threshold,
            access: Access{id: access_id, key: access_key, title: access_title}};
    
    let courses = conn.exec_map(&stmt,&params,&map).unwrap();
    return Json(courses);
}

#[rocket::get("/mod/course_moderator_list?<course_id>")]
pub fn course_moderator_list(session: UserSession, course_id: u32) -> Result<Json<Vec<Member>>, ApiError> {
    let moderators = match crate::db_course::get_course_moderator_list(&course_id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(moderators) => moderators,
    };

    if !moderators.iter().any(|member| member.id == session.user.id){
        return Err(ApiError::COURSE_NO_MODERATOR);
    };

    return Ok(Json(moderators));
}

#[rocket::get("/mod/course_slot_list?<course_id>")]
pub fn course_slot_list(session: UserSession, course_id: u32) -> Json<Vec<Slot>> {
    // TODO check if session user is moderator

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT slot_id, slot_key, s.title, l.location_id, l.location_key, l.title, s.begin, s.end, s.status
                          FROM slots s
                          JOIN locations l ON l.location_id = s.location_id
                          JOIN course_moderators m ON s.course_id = m.course_id
                          JOIN users u ON m.user_id = u.user_id
                          WHERE s.course_id = :course_id AND m.user_id = :user_id").unwrap();

    let params = params! {
        "course_id" => course_id,
        "user_id" => session.user.id,
    };

    let map = |(slot_id, slot_key, slot_title, location_id, location_key, location_title, begin, end, status): (u32, _, _, u32, _, _, _, _, String)|
        Slot {
            id: slot_id, key: slot_key, pwd: None, title: slot_title, begin, end, status: Some(status),
            location: Location {id: location_id, key: location_key, title: location_title},
            course_id: Some(course_id), owners: None};
    
    let slots = conn.exec_map(&stmt,&params,&map).unwrap();
    return Json(slots);
}

#[rocket::post("/mod/course_slot_create?<course_id>", format = "application/json", data = "<slot>")]
pub fn course_slot_create(session: UserSession, course_id: u32, mut slot: Json<Slot>) -> Result<String,ApiError> {
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

#[rocket::post("/mod/course_slot_edit?<course_id>", format = "application/json", data = "<slot>")]
pub fn course_slot_edit(session: UserSession, course_id: u32, mut slot: Json<Slot>) -> Result<(),ApiError> {
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

#[rocket::head("/mod/course_slot_delete?<slot_id>")]
pub fn course_slot_delete(session: UserSession, slot_id: u32) -> Status {
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

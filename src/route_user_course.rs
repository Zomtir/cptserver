use rocket::serde::json::Json;
use rocket::http::Status;

use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use crate::db::get_pool_conn;
use crate::session::UserSession;
use crate::common::{Course, Slot, Location, Branch, Access};
use crate::common::{random_string};

/*
 * ROUTES
 */

#[rocket::get("/user_course_list")]
pub fn user_course_list(session: UserSession) -> Json<Vec<Course>> {
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

#[rocket::get("/course_slot_list?<course_id>")]
pub fn course_slot_list(session: UserSession, course_id: u32) -> Json<Vec<Slot>> {
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

#[rocket::post("/course_slot_create", format = "application/json", data = "<slot>")]
pub fn course_slot_create(session: UserSession, mut slot: Json<Slot>) -> Option<String> {
    crate::common::round_slot_window(&mut slot);

    if !crate::common::is_slot_valid(&mut slot) {return None;}

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("INSERT INTO slots (slot_key, pwd, title, status, autologin, location_id, begin, end, course_id)
                          SELECT :slot_key, :pwd, :title, :status, :autologin, :location_id, :begin, :end, m.course_id
                          FROM course_moderators m
                          WHERE m.course_id = :course_id AND m.user_id = :user_id").unwrap();

    let params = params! {
        "slot_key" => random_string(8),
        "pwd" => random_string(8),
        "title" => &slot.title,
        "status" => "OCCURRING",
        "autologin" => false,
        "location_id" => &slot.location.id,
        "begin" => &slot.begin,
        "end" => &slot.end,
        "course_id" => &slot.course_id,
        "user_id" => &session.user.id,
    };

    match conn.exec::<String,_,_>(&stmt,&params) {
        Err(..) => return None,
        Ok(..) => (),
    };

    let stmt_id = conn.prep("SELECT LAST_INSERT_ID()").unwrap();

    let result_id = conn.exec_first::<u32,_,_>(
        &stmt_id,
        params::Params::Empty,
    );

    match result_id {
        Err(..) | Ok(None) => None,
        Ok(Some(slot_id)) => Some(slot_id.to_string()),
    }
}

// TODO round slot times
#[rocket::post("/course_slot_edit", format = "application/json", data = "<slot>")]
pub fn course_slot_edit(session: UserSession, slot: Json<Slot>) -> Status {
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
        "course_id" => &slot.course_id,
        "user_id" => &session.user.id,
    };

    match conn.exec::<String,_,_>(&stmt,&params) {
        Err(..) => return Status::Conflict,
        Ok(..) => (),
    };

    if slot.pwd.is_none() || slot.pwd.as_ref().unwrap().len() < 8 {return Status::Conflict};

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

    match conn.exec::<String,_,_>(&stmt_pwd,&params_pwd) {
        Err(..) => Status::Conflict,
        Ok(..) => Status::Ok,
    }
}

#[rocket::head("/course_slot_delete?<slot_id>")]
pub fn course_slot_delete(session: UserSession, slot_id: u32) -> Status {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE s FROM slots s
                          JOIN course_moderators m ON s.course_id = m.course_id
                          WHERE s.slot_id = :slot_id AND m.user_id = :user_id").unwrap();
    let params = params! {
        "slot_id" => &slot_id,
        "user_id" => &session.user.id,
    };

    match conn.exec::<String,_,_>(&stmt,&params) {
        Err(..) => Status::Conflict,
        Ok(..) => Status::Ok,
    }
}

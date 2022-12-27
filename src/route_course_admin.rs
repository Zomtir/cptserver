use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use rocket::serde::json::Json;

use crate::api::ApiError;
use crate::db::get_pool_conn;
use crate::session::{UserSession};
use crate::common::{Course, User};

#[rocket::get("/admin/course_list?<mod_id>")]
pub fn course_list(session: UserSession, mod_id: Option<u32>) -> Result<Json<Vec<Course>>, ApiError> {
    if !session.right.admin_courses {return Err(ApiError::RIGHT_NO_COURSES)};
    
    match crate::db_course::get_course_list(mod_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(courses) => Ok(Json(courses)),
    }
}

#[rocket::post("/admin/course_create", format = "application/json", data = "<course>")]
pub fn course_create(course: Json<Course>, session: UserSession) -> Result<String, ApiError> {
    if !session.right.admin_courses {return Err(ApiError::RIGHT_NO_COURSES)};

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("INSERT INTO courses (course_key, title, active, access_id, branch_id, threshold)
        VALUES (:course_key, :title, :active, :access_id, :branch_id, :threshold)").unwrap();
    let params = params! {
        "course_key" => crate::common::random_string(10),
        "title" => &course.title,
        "active" => &course.active,
        "access_id" => &course.access.id,
        "branch_id" => &course.branch.id,
        "threshold" => &course.threshold,
    };

    match conn.exec_drop(&stmt,&params) {
        Err(..) => return Err(ApiError::DB_CONFLICT),
        Ok(..) => (),
    };

    let stmt_id = conn.prep("SELECT LAST_INSERT_ID()").unwrap();

    match conn.exec_first::<u32,_,_>(&stmt_id,params::Params::Empty) {
        Err(..) | Ok(None) => Err(ApiError::DB_CONFLICT),
        Ok(Some(course_id)) => Ok(course_id.to_string()),
    }
}

#[rocket::post("/admin/course_edit", format = "application/json", data = "<course>")]
pub fn course_edit(course: Json<Course>, session: UserSession) -> Result<(),ApiError> {
    if !session.right.admin_courses {return Err(ApiError::RIGHT_NO_COURSES)};

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("UPDATE courses SET
        course_key = :course_key,
        title = :title,
        active = :active,
        access_id = :access_id,
        branch_id = :branch_id,
        threshold = :threshold
        WHERE course_id = :course_id").unwrap();

    let params = params! {
        "course_id" => &course.id,
        "course_key" => &course.key,
        "title" => &course.title,
        "active" => &course.active,
        "access_id" => &course.access.id,
        "branch_id" => &course.branch.id,
        "threshold" => &course.threshold,
    };

    match conn.exec_drop(&stmt,&params) {
        Err(..) => Err(ApiError::DB_CONFLICT),
        Ok(..) => Ok(()),
    }
}

#[rocket::head("/admin/course_delete?<course_id>")]
pub fn course_delete(course_id: u32, session: UserSession) -> Result<(),ApiError> {
    if !session.right.admin_courses {return Err(ApiError::RIGHT_NO_COURSES)};

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE c FROM courses c
                          WHERE c.course_id = :course_id").unwrap();
    let params = params! {"course_id" => &course_id};

    match conn.exec_drop(&stmt,&params) {
        Err(..) => Err(ApiError::DB_CONFLICT),
        Ok(..) => Ok(()),
    }
}

#[rocket::get("/admin/course_moderator_list?<course_id>")]
pub fn course_moderator_list(session: UserSession, course_id: u32) -> Result<Json<Vec<User>>, ApiError> {
    if !session.right.admin_courses {return Err(ApiError::RIGHT_NO_COURSES)};

    match crate::db_course::get_course_moderator_list(&course_id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(moderators) => Ok(Json(moderators)),
    }
}

#[rocket::head("/admin/course_moderator_add?<course_id>&<user_id>")]
pub fn course_moderator_add(session: UserSession, course_id: u32, user_id: u32) -> Result<(),ApiError> {
    if !session.right.admin_courses {return Err(ApiError::RIGHT_NO_COURSES)};

    match crate::db_course::add_course_moderator(course_id, user_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}

#[rocket::head("/admin/course_moderator_remove?<course_id>&<user_id>")]
pub fn course_moderator_remove(session: UserSession, course_id: u32, user_id: u32) -> Result<(),ApiError> {
    if !session.right.admin_courses {return Err(ApiError::RIGHT_NO_COURSES)};

    match crate::db_course::remove_course_moderator(course_id, user_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}

use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use rocket::serde::json::Json;

use crate::api::ApiError;
use crate::db::get_pool_conn;
use crate::session::{UserSession};
use crate::common::{Course, User};

#[rocket::get("/admin/course_list?<mod_id>")]
pub fn course_list(session: UserSession, mod_id: Option<i64>) -> Result<Json<Vec<Course>>, ApiError> {
    if !session.right.admin_courses {return Err(ApiError::RIGHT_NO_COURSES)};
    
    match crate::db_course::list_courses(mod_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(courses) => Ok(Json(courses)),
    }
}

#[rocket::post("/admin/course_create", format = "application/json", data = "<course>")]
pub fn course_create(session: UserSession, course: Json<Course>) -> Result<String, ApiError> {
    if !session.right.admin_courses {return Err(ApiError::RIGHT_NO_COURSES)};

    let id = crate::db_course::create_course(&course)?;
    Ok(id.to_string())
}

#[rocket::post("/admin/course_edit", format = "application/json", data = "<course>")]
pub fn course_edit(session: UserSession, course: Json<Course>) -> Result<(),ApiError> {
    if !session.right.admin_courses {return Err(ApiError::RIGHT_NO_COURSES)};

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("UPDATE courses SET
        course_key = :course_key,
        title = :title,
        active = :active,
        public = :public,
        branch_id = :branch_id,
        threshold = :threshold
        WHERE course_id = :course_id").unwrap();

    let params = params! {
        "course_id" => &course.id,
        "course_key" => &course.key,
        "title" => &course.title,
        "active" => &course.active,
        "public" => &course.public,
        "branch_id" => &course.branch.id,
        "threshold" => &course.threshold,
    };

    match conn.exec_drop(&stmt,&params) {
        Err(..) => Err(ApiError::DB_CONFLICT),
        Ok(..) => Ok(()),
    }
}

#[rocket::head("/admin/course_delete?<course_id>")]
pub fn course_delete(session: UserSession, course_id: i64) -> Result<(),ApiError> {
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
pub fn course_moderator_list(session: UserSession, course_id: i64) -> Result<Json<Vec<User>>, ApiError> {
    if !session.right.admin_courses {return Err(ApiError::RIGHT_NO_COURSES)};

    match crate::db_course::list_course_moderators(course_id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(moderators) => Ok(Json(moderators)),
    }
}

#[rocket::head("/admin/course_moderator_add?<course_id>&<user_id>")]
pub fn course_moderator_add(session: UserSession, course_id: i64, user_id: i64) -> Result<(),ApiError> {
    if !session.right.admin_courses {return Err(ApiError::RIGHT_NO_COURSES)};

    match crate::db_course::add_course_moderator(course_id, user_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}

#[rocket::head("/admin/course_moderator_remove?<course_id>&<user_id>")]
pub fn course_moderator_remove(session: UserSession, course_id: i64, user_id: i64) -> Result<(),ApiError> {
    if !session.right.admin_courses {return Err(ApiError::RIGHT_NO_COURSES)};

    match crate::db_course::remove_course_moderator(course_id, user_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}

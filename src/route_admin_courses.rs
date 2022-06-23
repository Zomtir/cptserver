use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use rocket::http::Status;
use rocket::serde::json::Json;

use crate::db::get_pool_conn;
use crate::session::{UserSession, Course, Branch, Access, Member, random_string};

#[rocket::get("/course_list?<user_id>")]
pub fn course_list(user_id: u32, session: UserSession) -> Result<Json<Vec<Course>>, Status> {
    if !session.user.admin_courses {return Err(Status::Unauthorized)};

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT c.course_id, c.course_key, c.title, c.active,
                          b.branch_id, b.branch_key, b.title, c.threshold,
                          a.access_id, a.access_key, a.title
                        FROM courses c
                        JOIN branches b ON c.branch_id = b.branch_id
                        JOIN access a ON c.access_id = a.access_id
                        LEFT JOIN course_moderators m ON c.course_id = m.course_id
                        WHERE ((:user_id = '0') OR (m.user_id = :user_id))
                        GROUP BY c.course_id").unwrap();
    // TODO the WHERE and GROUP BY clause can be removed, if the user filter is deemed to be useless
        
    let params = params! {"user_id" => user_id};
    let map = |(course_id, course_key, course_title, active,
            branch_id, branch_key, branch_title, threshold,
            access_id, access_key, access_title): (u32, String, String, bool, u16, String, String, u8, u8, String, String)|
        Course {
            id: course_id, key: course_key, title: course_title, active,
            branch: Branch{id: branch_id, key: branch_key, title: branch_title}, threshold,
            access: Access{id: access_id, key: access_key, title: access_title}};
    
    match conn.exec_map(&stmt,&params,&map) {
        Err(..) => Err(Status::Conflict),
        Ok(courses) => Ok(Json(courses)),
    }
}

#[rocket::post("/course_create", format = "application/json", data = "<course>")]
pub fn course_create(course: Json<Course>, session: UserSession) -> Result<String, Status> {
    if !session.user.admin_courses {return Err(Status::Unauthorized)};

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("INSERT INTO courses (course_key, title, active, access_id, branch_id, threshold)
        VALUES (:course_key, :title, :active, :access_id, :branch_id, :threshold)").unwrap();
    let params = params! {
        "course_key" => random_string(10),
        "title" => &course.title,
        "active" => &course.active,
        "access_id" => &course.access.id,
        "branch_id" => &course.branch.id,
        "threshold" => &course.threshold,
    };

    match conn.exec::<String,_,_>(&stmt,&params) {
        Err(..) => return Err(Status::BadRequest),
        Ok(..) => (),
    };

    let stmt_id = conn.prep("SELECT LAST_INSERT_ID()").unwrap();

    match conn.exec_first::<u32,_,_>(&stmt_id,params::Params::Empty) {
        Err(..) | Ok(None) => Err(Status::Conflict),
        Ok(Some(course_id)) => Ok(course_id.to_string()),
    }
}

#[rocket::post("/course_edit", format = "application/json", data = "<course>")]
pub fn course_edit(course: Json<Course>, session: UserSession) -> Status {
    if !session.user.admin_courses {return Status::Unauthorized;};

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

    match conn.exec::<String,_,_>(&stmt,&params) {
        Err(..) => Status::BadRequest,
        Ok(..) => Status::Ok,
    }
}

#[rocket::head("/course_delete?<course_id>")]
pub fn course_delete(course_id: u32, session: UserSession) -> Status {
    if !session.user.admin_courses {return Status::Unauthorized};

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE c FROM courses c
                          WHERE c.course_id = :course_id").unwrap();
    let params = params! {"course_id" => &course_id};

    match conn.exec::<String,_,_>(&stmt,&params) {
        Err(..) => Status::BadRequest,
        Ok(..) => Status::Ok,
    }
}

// TODO check SQL call if permissions are correct, also this does not require admin to call??? 
#[rocket::get("/course_moderator_list?<course_id>")]
pub fn course_moderator_list(course_id: u32, session: UserSession) -> Result<Json<Vec<Member>>, Status> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT u.user_id, u.user_key, u.firstname, u.lastname
                          FROM users u
                          JOIN course_moderators m ON m.user_id = u.user_id
                          WHERE m.course_id = :course_id").unwrap();

    let params = params! { "course_id" => course_id};
    let map = |(user_id, user_key, firstname, lastname)| {
        Member{id: user_id, key: user_key, firstname, lastname}
    };

    let members = conn.exec_map(&stmt, &params, &map).unwrap();

    // Bail if you are neither admin nor course moderator
    // This means that participants will not be able to see the teachers
    if !session.user.admin_courses && !members.iter().any(|member| member.id == session.user.id){
        return Err(Status::Unauthorized);
    };

    return Ok(Json(members));
}

#[rocket::head("/course_mod?<course_id>&<user_id>")]
pub fn course_mod(course_id: u32, user_id: u32, session: UserSession) -> Status {
    if !session.user.admin_courses {return Status::Unauthorized};

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("INSERT INTO course_moderators (course_id, user_id)
                          SELECT :course_id, :user_id").unwrap();
    let params = params! {
        "course_id" => &course_id,
        "user_id" => &user_id,
    };

    match conn.exec::<String,_,_>(&stmt,&params) {
        Err(..) => Status::Conflict,
        Ok(..) => Status::Ok,
    }
}

#[rocket::head("/course_unmod?<course_id>&<user_id>")]
pub fn course_unmod(course_id: u32, user_id: u32, session: UserSession) -> Status {
    if !session.user.admin_courses {return Status::Unauthorized};

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE e FROM course_moderators e
                          WHERE course_id = :course_id AND user_id = :user_id").unwrap();
    let params = params! {
        "course_id" => &course_id,
        "user_id" => &user_id,
    };

    match conn.exec::<String,_,_>(&stmt,&params) {
        Err(..) => Status::Conflict,
        Ok(..) => Status::Ok,
    }
}
use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use rocket::serde::json::Json;

use crate::common::{Course, Team, User};
use crate::db::get_pool_conn;
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/admin/course_list?<mod_id>")]
pub fn course_list(session: UserSession, mod_id: Option<i64>) -> Result<Json<Vec<Course>>, Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    match crate::db_course::list_courses(mod_id, None, None)? {
        courses => Ok(Json(courses)),
    }
}

#[rocket::post("/admin/course_create", format = "application/json", data = "<course>")]
pub fn course_create(session: UserSession, course: Json<Course>) -> Result<String, Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    let id = crate::db_course::course_create(&course)?;
    Ok(id.to_string())
}

#[rocket::post("/admin/course_edit", format = "application/json", data = "<course>")]
pub fn course_edit(session: UserSession, course: Json<Course>) -> Result<(), Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn
        .prep(
            "UPDATE courses SET
        course_key = :course_key,
        title = :title,
        active = :active,
        public = :public,
        branch_id = :branch_id,
        threshold = :threshold
        WHERE course_id = :course_id",
        )
        .unwrap();

    let params = params! {
        "course_id" => &course.id,
        "course_key" => &course.key,
        "title" => &course.title,
        "active" => &course.active,
        "public" => &course.public,
        "branch_id" => &course.branch.id,
        "threshold" => &course.threshold,
    };

    match conn.exec_drop(&stmt, &params) {
        Err(..) => Err(Error::DatabaseError),
        Ok(..) => Ok(()),
    }
}

#[rocket::head("/admin/course_delete?<course_id>")]
pub fn course_delete(session: UserSession, course_id: i64) -> Result<(), Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn
        .prep(
            "DELETE c FROM courses c
                          WHERE c.course_id = :course_id",
        )
        .unwrap();
    let params = params! {"course_id" => &course_id};

    match conn.exec_drop(&stmt, &params) {
        Err(..) => Err(Error::DatabaseError),
        Ok(..) => Ok(()),
    }
}

#[rocket::get("/admin/course_moderator_list?<course_id>")]
pub fn course_moderator_list(session: UserSession, course_id: i64) -> Result<Json<Vec<User>>, Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    match crate::db_course::course_moderator_list(course_id)? {
        moderators => Ok(Json(moderators)),
    }
}

#[rocket::head("/admin/course_moderator_add?<course_id>&<user_id>")]
pub fn course_moderator_add(session: UserSession, course_id: i64, user_id: i64) -> Result<(), Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    match crate::db_course::course_moderator_add(course_id, user_id) {
        None => Err(Error::DatabaseError),
        Some(..) => Ok(()),
    }
}

#[rocket::head("/admin/course_moderator_remove?<course_id>&<user_id>")]
pub fn course_moderator_remove(session: UserSession, course_id: i64, user_id: i64) -> Result<(), Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    match crate::db_course::course_moderator_remove(course_id, user_id) {
        None => Err(Error::DatabaseError),
        Some(..) => Ok(()),
    }
}

#[rocket::get("/admin/course_teaminvite_list?<course_id>")]
pub fn course_teaminvite_list(session: UserSession, course_id: i64) -> Result<Json<Vec<Team>>, Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    match crate::db_course::course_teaminvite_list(course_id)? {
        teams => Ok(Json(teams)),
    }
}

#[rocket::head("/admin/course_teaminvite_add?<course_id>&<team_id>")]
pub fn course_teaminvite_add(session: UserSession, course_id: i64, team_id: i64) -> Result<(), Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    crate::db_course::course_teaminvite_add(course_id, team_id)?;
    Ok(())
}

#[rocket::head("/admin/course_teaminvite_remove?<course_id>&<team_id>")]
pub fn course_teaminvite_remove(session: UserSession, course_id: i64, team_id: i64) -> Result<(), Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    crate::db_course::course_teaminvite_remove(course_id, team_id)?;
    Ok(())
}

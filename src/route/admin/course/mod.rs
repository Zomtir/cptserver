pub mod attendance;
pub mod moderator;

use rocket::serde::json::Json;

use crate::common::{Acceptance, Course, Event, Requirement, User, WebBool};
use crate::error::{ErrorKind, Result};
use crate::session::UserSession;

#[rocket::get("/admin/course_list?<mod_id>&<active>&<public>")]
pub fn course_list(
    session: UserSession,
    mod_id: Option<u64>,
    active: Option<WebBool>,
    public: Option<WebBool>,
) -> Result<Json<Vec<Course>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_course_read {
        return Err(ErrorKind::RightCourseMissing);
    };

    let courses =
        crate::db::course::course_list(conn, mod_id, active.map(|b| b.to_bool()), public.map(|b| b.to_bool()))?;
    Ok(Json(courses))
}

#[rocket::post("/admin/course_create", format = "application/json", data = "<course>")]
pub fn course_create(session: UserSession, course: Json<Course>) -> Result<String> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_course_write {
        return Err(ErrorKind::RightCourseMissing);
    };

    let id = crate::db::course::course_create(conn, &course)?;
    Ok(id.to_string())
}

#[rocket::post("/admin/course_edit?<course_id>", format = "application/json", data = "<course>")]
pub fn course_edit(session: UserSession, course_id: u32, course: Json<Course>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_course_write {
        return Err(ErrorKind::RightCourseMissing);
    };

    crate::db::course::course_edit(conn, course_id, &course)?;
    Ok(())
}

#[rocket::head("/admin/course_delete?<course_id>")]
pub fn course_delete(session: UserSession, course_id: u32) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_course_write {
        return Err(ErrorKind::RightCourseMissing);
    };

    crate::db::course::course_delete(conn, course_id)?;
    Ok(())
}

#[rocket::get("/admin/course_event_list?<course_id>")]
pub fn course_event_list(session: UserSession, course_id: u32) -> Result<Json<Vec<Event>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_course_read {
        return Err(ErrorKind::RightCourseMissing);
    };

    let events = crate::db::event::event_list(
        conn,
        None,
        None,
        None,
        None,
        Some(Acceptance::Accepted),
        Some(true),
        Some(course_id),
        None,
    )?;
    Ok(Json(events))
}

#[rocket::get("/admin/course_requirement_list?<course_id>")]
pub fn course_requirement_list(session: UserSession, course_id: u32) -> Result<Json<Vec<Requirement>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_course_read {
        return Err(ErrorKind::RightCourseMissing);
    };

    let reqs = crate::db::course::course_requirement_list(conn, course_id)?;
    Ok(Json(reqs))
}

#[rocket::head("/admin/course_requirement_add?<course_id>&<skill_id>&<rank>")]
pub fn course_requirement_add(session: UserSession, course_id: u32, skill_id: u32, rank: u32) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_course_write {
        return Err(ErrorKind::RightCourseMissing);
    };

    crate::db::course::course_requirement_add(conn, course_id, skill_id, rank)?;
    Ok(())
}

#[rocket::head("/admin/course_requirement_remove?<requirement_id>")]
pub fn course_requirement_remove(session: UserSession, requirement_id: u64) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_course_write {
        return Err(ErrorKind::RightCourseMissing);
    };

    crate::db::course::course_requirement_remove(conn, requirement_id)?;
    Ok(())
}

#[rocket::get("/admin/course_club_info?<course_id>")]
pub fn course_club_info(session: UserSession, course_id: u64) -> Result<Json<Option<u32>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_course_read {
        return Err(ErrorKind::RightCourseMissing);
    };

    let club_id = crate::db::course::course_club_info(conn, course_id)?;
    Ok(Json(club_id))
}

#[rocket::head("/admin/course_club_edit?<course_id>&<club_id>")]
pub fn course_club_edit(session: UserSession, course_id: u64, club_id: Option<u32>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_course_write {
        return Err(ErrorKind::RightCourseMissing);
    };
    if !session.right.right_club_write {
        return Err(ErrorKind::RightClubMissing);
    };

    crate::db::course::course_club_edit(conn, course_id, club_id)?;
    Ok(())
}

#[rocket::get("/admin/course_statistic_class?<course_id>")]
pub fn course_statistic_class(session: UserSession, course_id: u32) -> Result<Json<Vec<(Event, u64, u64, u64, u64)>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_course_read {
        return Err(ErrorKind::RightCourseMissing);
    };

    let stats = crate::db::course::course_statistic_class(conn, course_id)?;
    Ok(Json(stats))
}

#[rocket::get("/admin/course_statistic_attendance?<course_id>&<role>")]
pub fn course_statistic_attendance(
    session: UserSession,
    course_id: u32,
    role: String,
) -> Result<Json<Vec<(User, u64)>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_course_read {
        return Err(ErrorKind::RightCourseMissing);
    };

    let stats = crate::db::course::course_statistic_attendance(conn, course_id, role)?;
    Ok(Json(stats))
}

#[rocket::get("/admin/course_statistic_attendance1?<course_id>&<user_id>&<role>")]
pub fn course_statistic_attendance1(
    session: UserSession,
    course_id: u32,
    user_id: u64,
    role: String,
) -> Result<Json<Vec<Event>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_course_read {
        return Err(ErrorKind::RightCourseMissing);
    };

    let stats = crate::db::course::course_statistic_attendance1(conn, course_id, user_id, role)?;
    Ok(Json(stats))
}

use rocket::serde::json::Json;

use crate::common::{Course, Team, User};
use crate::error::Error;
use crate::session::UserSession;
use chrono::NaiveDateTime;

#[rocket::get("/admin/course_list?<mod_id>&<active>&<public>")]
pub fn course_list(
    session: UserSession,
    mod_id: Option<i64>,
    active: Option<bool>,
    public: Option<bool>,
) -> Result<Json<Vec<Course>>, Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    let courses = crate::db_course::course_list(mod_id, active, public)?;
    Ok(Json(courses))
}

#[rocket::post("/admin/course_create", format = "application/json", data = "<course>")]
pub fn course_create(session: UserSession, course: Json<Course>) -> Result<String, Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    let id = crate::db_course::course_create(&course)?;
    Ok(id.to_string())
}

#[rocket::post("/admin/course_edit?<course_id>", format = "application/json", data = "<course>")]
pub fn course_edit(session: UserSession, course_id: i64, course: Json<Course>) -> Result<(), Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    crate::db_course::course_edit(course_id, &course)?;
    Ok(())
}

#[rocket::head("/admin/course_delete?<course_id>")]
pub fn course_delete(session: UserSession, course_id: i64) -> Result<(), Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    crate::db_course::course_delete(course_id)?;
    Ok(())
}

#[rocket::get("/admin/course_moderator_list?<course_id>")]
pub fn course_moderator_list(session: UserSession, course_id: i64) -> Result<Json<Vec<User>>, Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    let moderators = crate::db_course::course_moderator_list(course_id)?;
    Ok(Json(moderators))
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

    crate::db_course::course_moderator_remove(course_id, user_id)?;
    Ok(())
}

#[rocket::get("/admin/course_participant_team_list?<course_id>")]
pub fn course_participant_team_list(session: UserSession, course_id: i64) -> Result<Json<Vec<Team>>, Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    let teams = crate::db_course::course_participant_team_list(course_id)?;
    Ok(Json(teams))
}

#[rocket::head("/admin/course_participant_team_add?<course_id>&<team_id>")]
pub fn course_participant_team_add(session: UserSession, course_id: i64, team_id: i64) -> Result<(), Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    crate::db_course::course_participant_team_add(course_id, team_id)?;
    Ok(())
}

#[rocket::head("/admin/course_participant_team_remove?<course_id>&<team_id>")]
pub fn course_participant_team_remove(session: UserSession, course_id: i64, team_id: i64) -> Result<(), Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    crate::db_course::course_participant_team_remove(course_id, team_id)?;
    Ok(())
}

#[rocket::get("/admin/course_owner_team_list?<course_id>")]
pub fn course_owner_team_list(session: UserSession, course_id: i64) -> Result<Json<Vec<Team>>, Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    let teams = crate::db_course::course_owner_team_list(course_id)?;
    Ok(Json(teams))
}

#[rocket::head("/admin/course_owner_team_add?<course_id>&<team_id>")]
pub fn course_owner_team_add(session: UserSession, course_id: i64, team_id: i64) -> Result<(), Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    crate::db_course::course_owner_team_add(course_id, team_id)?;
    Ok(())
}

#[rocket::head("/admin/course_owner_team_remove?<course_id>&<team_id>")]
pub fn course_owner_team_remove(session: UserSession, course_id: i64, team_id: i64) -> Result<(), Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    crate::db_course::course_owner_team_remove(course_id, team_id)?;
    Ok(())
}

#[rocket::get("/admin/course_statistic_class?<course_id>")]
pub fn course_statistic_class(
    session: UserSession,
    course_id: i64,
) -> Result<Json<Vec<(i64, String, NaiveDateTime, NaiveDateTime, i64, i64)>>, Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    let stats = crate::db_course::course_statistic_class(course_id)?;
    Ok(Json(stats))
}

#[rocket::get("/admin/course_statistic_participant?<course_id>")]
pub fn course_statistic_participant(
    session: UserSession,
    course_id: i64,
) -> Result<Json<Vec<(i64, String, String, i64)>>, Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    let stats = crate::db_course::course_statistic_participant(course_id)?;
    Ok(Json(stats))
}

#[rocket::get("/admin/course_statistic_participant1?<course_id>&<participant_id>")]
pub fn course_statistic_participant1(
    session: UserSession,
    course_id: i64,
    participant_id: i64,
) -> Result<Json<Vec<(i64, String, NaiveDateTime, NaiveDateTime)>>, Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    let stats = crate::db_course::course_statistic_participant1(course_id, participant_id)?;
    Ok(Json(stats))
}

#[rocket::get("/admin/course_statistic_owner?<course_id>")]
pub fn course_statistic_owner(
    session: UserSession,
    course_id: i64,
) -> Result<Json<Vec<(i64, String, String, i64)>>, Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    let stats = crate::db_course::course_statistic_owner(course_id)?;
    Ok(Json(stats))
}

#[rocket::get("/admin/course_statistic_owner1?<course_id>&<owner_id>")]
pub fn course_statistic_owner1(
    session: UserSession,
    course_id: i64,
    owner_id: i64,
) -> Result<Json<Vec<(i64, String, NaiveDateTime, NaiveDateTime)>>, Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    let stats = crate::db_course::course_statistic_owner1(course_id, owner_id)?;
    Ok(Json(stats))
}

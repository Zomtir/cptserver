use rocket::serde::json::Json;

use crate::common::{Course, Event, Team, User, WebBool};
use crate::error::Error;
use crate::session::UserSession;
use chrono::NaiveDateTime;

#[rocket::get("/admin/course_list?<mod_id>&<active>&<public>")]
pub fn course_list(
    session: UserSession,
    mod_id: Option<u64>,
    active: Option<WebBool>,
    public: Option<WebBool>,
) -> Result<Json<Vec<Course>>, Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    let courses = crate::db_course::course_list(mod_id, active.map(|b| b.to_bool()), public.map(|b| b.to_bool()))?;
    Ok(Json(courses))
}

#[rocket::post("/admin/course_create", format = "application/json", data = "<course>")]
pub fn course_create(session: UserSession, course: Json<Course>) -> Result<String, Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    let id = crate::db_course::course_create(&course)?;
    Ok(id.to_string())
}

#[rocket::post("/admin/course_edit?<course_id>", format = "application/json", data = "<course>")]
pub fn course_edit(session: UserSession, course_id: u64, course: Json<Course>) -> Result<(), Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    crate::db_course::course_edit(course_id, &course)?;
    Ok(())
}

#[rocket::head("/admin/course_delete?<course_id>")]
pub fn course_delete(session: UserSession, course_id: u64) -> Result<(), Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    crate::db_course::course_delete(course_id)?;
    Ok(())
}

#[rocket::get("/admin/course_event_list?<course_id>")]
pub fn course_event_list(session: UserSession, course_id: u64) -> Result<Json<Vec<Event>>, Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    let events = crate::db_event::event_list(None, None, None, None, Some(true), Some(course_id), None)?;
    Ok(Json(events))
}

#[rocket::get("/admin/course_moderator_list?<course_id>")]
pub fn course_moderator_list(session: UserSession, course_id: u64) -> Result<Json<Vec<User>>, Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    let moderators = crate::db_course::course_moderator_list(course_id)?;
    Ok(Json(moderators))
}

#[rocket::head("/admin/course_moderator_add?<course_id>&<user_id>")]
pub fn course_moderator_add(session: UserSession, course_id: u64, user_id: u64) -> Result<(), Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    match crate::db_course::course_moderator_add(course_id, user_id) {
        None => Err(Error::DatabaseError),
        Some(..) => Ok(()),
    }
}

#[rocket::head("/admin/course_moderator_remove?<course_id>&<user_id>")]
pub fn course_moderator_remove(session: UserSession, course_id: u64, user_id: u64) -> Result<(), Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    crate::db_course::course_moderator_remove(course_id, user_id)?;
    Ok(())
}

#[rocket::get("/admin/course_owner_summon_list?<course_id>")]
pub fn course_owner_summon_list(session: UserSession, course_id: u64) -> Result<Json<Vec<Team>>, Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    let teams = crate::db_course::course_owner_summon_list(course_id)?;
    Ok(Json(teams))
}

#[rocket::head("/admin/course_owner_summon_add?<course_id>&<team_id>")]
pub fn course_owner_summon_add(session: UserSession, course_id: u64, team_id: u64) -> Result<(), Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    crate::db_course::course_owner_summon_add(course_id, team_id)?;
    Ok(())
}

#[rocket::head("/admin/course_owner_summon_remove?<course_id>&<team_id>")]
pub fn course_owner_summon_remove(session: UserSession, course_id: u64, team_id: u64) -> Result<(), Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    crate::db_course::course_owner_summon_remove(course_id, team_id)?;
    Ok(())
}

#[rocket::get("/admin/course_owner_unsummon_list?<course_id>")]
pub fn course_owner_unsummon_list(session: UserSession, course_id: u64) -> Result<Json<Vec<Team>>, Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    let teams = crate::db_course::course_owner_unsummon_list(course_id)?;
    Ok(Json(teams))
}

#[rocket::head("/admin/course_owner_unsummon_add?<course_id>&<team_id>")]
pub fn course_owner_unsummon_add(session: UserSession, course_id: u64, team_id: u64) -> Result<(), Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    crate::db_course::course_owner_unsummon_add(course_id, team_id)?;
    Ok(())
}

#[rocket::head("/admin/course_owner_unsummon_remove?<course_id>&<team_id>")]
pub fn course_owner_unsummon_remove(session: UserSession, course_id: u64, team_id: u64) -> Result<(), Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    crate::db_course::course_owner_unsummon_remove(course_id, team_id)?;
    Ok(())
}

#[rocket::get("/admin/course_participant_summon_list?<course_id>")]
pub fn course_participant_summon_list(session: UserSession, course_id: u64) -> Result<Json<Vec<Team>>, Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    let teams = crate::db_course::course_participant_summon_list(course_id)?;
    Ok(Json(teams))
}

#[rocket::head("/admin/course_participant_summon_add?<course_id>&<team_id>")]
pub fn course_participant_summon_add(session: UserSession, course_id: u64, team_id: u64) -> Result<(), Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    crate::db_course::course_participant_summon_add(course_id, team_id)?;
    Ok(())
}

#[rocket::head("/admin/course_participant_summon_remove?<course_id>&<team_id>")]
pub fn course_participant_summon_remove(session: UserSession, course_id: u64, team_id: u64) -> Result<(), Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    crate::db_course::course_participant_summon_remove(course_id, team_id)?;
    Ok(())
}

#[rocket::get("/admin/course_participant_unsummon_list?<course_id>")]
pub fn course_participant_unsummon_list(session: UserSession, course_id: u64) -> Result<Json<Vec<Team>>, Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    let teams = crate::db_course::course_participant_unsummon_list(course_id)?;
    Ok(Json(teams))
}

#[rocket::head("/admin/course_participant_unsummon_add?<course_id>&<team_id>")]
pub fn course_participant_unsummon_add(session: UserSession, course_id: u64, team_id: u64) -> Result<(), Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    crate::db_course::course_participant_unsummon_add(course_id, team_id)?;
    Ok(())
}

#[rocket::head("/admin/course_participant_unsummon_remove?<course_id>&<team_id>")]
pub fn course_participant_unsummon_remove(session: UserSession, course_id: u64, team_id: u64) -> Result<(), Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    crate::db_course::course_participant_unsummon_remove(course_id, team_id)?;
    Ok(())
}

#[rocket::get("/admin/course_statistic_class?<course_id>")]
pub fn course_statistic_class(
    session: UserSession,
    course_id: u64,
) -> Result<Json<Vec<(u64, String, NaiveDateTime, NaiveDateTime, u64, u64)>>, Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    let stats = crate::db_course::course_statistic_class(course_id)?;
    Ok(Json(stats))
}

#[rocket::get("/admin/course_statistic_participant?<course_id>")]
pub fn course_statistic_participant(
    session: UserSession,
    course_id: u64,
) -> Result<Json<Vec<(u64, String, String, u64)>>, Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    let stats = crate::db_course::course_statistic_participant(course_id)?;
    Ok(Json(stats))
}

#[rocket::get("/admin/course_statistic_participant1?<course_id>&<participant_id>")]
pub fn course_statistic_participant1(
    session: UserSession,
    course_id: u64,
    participant_id: u64,
) -> Result<Json<Vec<(u64, String, NaiveDateTime, NaiveDateTime)>>, Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    let stats = crate::db_course::course_statistic_participant1(course_id, participant_id)?;
    Ok(Json(stats))
}

#[rocket::get("/admin/course_statistic_owner?<course_id>")]
pub fn course_statistic_owner(
    session: UserSession,
    course_id: u64,
) -> Result<Json<Vec<(u64, String, String, u64)>>, Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    let stats = crate::db_course::course_statistic_owner(course_id)?;
    Ok(Json(stats))
}

#[rocket::get("/admin/course_statistic_owner1?<course_id>&<owner_id>")]
pub fn course_statistic_owner1(
    session: UserSession,
    course_id: u64,
    owner_id: u64,
) -> Result<Json<Vec<(u64, String, NaiveDateTime, NaiveDateTime)>>, Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    let stats = crate::db_course::course_statistic_owner1(course_id, owner_id)?;
    Ok(Json(stats))
}

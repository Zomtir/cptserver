pub mod leader;
pub mod moderator;
pub mod participant;
pub mod supporter;

use rocket::serde::json::Json;

use crate::common::{Acceptance, Course, Event, Requirement, User, WebBool};
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/admin/course_list?<mod_id>&<active>&<public>")]
pub fn course_list(
    session: UserSession,
    mod_id: Option<u64>,
    active: Option<WebBool>,
    public: Option<WebBool>,
) -> Result<Json<Vec<Course>>, Error> {
    if !session.right.right_course_read {
        return Err(Error::RightCourseMissing);
    };

    let courses = crate::db::course::course_list(mod_id, active.map(|b| b.to_bool()), public.map(|b| b.to_bool()))?;
    Ok(Json(courses))
}

#[rocket::post("/admin/course_create", format = "application/json", data = "<course>")]
pub fn course_create(session: UserSession, course: Json<Course>) -> Result<String, Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    let id = crate::db::course::course_create(&course)?;
    Ok(id.to_string())
}

#[rocket::post("/admin/course_edit?<course_id>", format = "application/json", data = "<course>")]
pub fn course_edit(session: UserSession, course_id: u32, course: Json<Course>) -> Result<(), Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    crate::db::course::course_edit(course_id, &course)?;
    Ok(())
}

#[rocket::head("/admin/course_delete?<course_id>")]
pub fn course_delete(session: UserSession, course_id: u32) -> Result<(), Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    crate::db::course::course_delete(course_id)?;
    Ok(())
}

#[rocket::get("/admin/course_event_list?<course_id>")]
pub fn course_event_list(session: UserSession, course_id: u32) -> Result<Json<Vec<Event>>, Error> {
    if !session.right.right_course_read {
        return Err(Error::RightCourseMissing);
    };

    let events = crate::db::event::event_list(
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
pub fn course_requirement_list(session: UserSession, course_id: u32) -> Result<Json<Vec<Requirement>>, Error> {
    if !session.right.right_course_read {
        return Err(Error::RightCourseMissing);
    };

    let reqs = crate::db::course::course_requirement_list(course_id)?;
    Ok(Json(reqs))
}

#[rocket::head("/admin/course_requirement_add?<course_id>&<skill_id>&<rank>")]
pub fn course_requirement_add(session: UserSession, course_id: u32, skill_id: u32, rank: u32) -> Result<(), Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    crate::db::course::course_requirement_add(course_id, skill_id, rank)?;
    Ok(())
}

#[rocket::head("/admin/course_requirement_remove?<requirement_id>")]
pub fn course_requirement_remove(session: UserSession, requirement_id: u64) -> Result<(), Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    crate::db::course::course_requirement_remove(requirement_id)?;
    Ok(())
}

#[rocket::get("/admin/course_club_info?<course_id>")]
pub fn course_club_info(session: UserSession, course_id: u64) -> Result<Json<Option<u32>>, Error> {
    if !session.right.right_course_read {
        return Err(Error::RightCourseMissing);
    };

    let club_id = crate::db::course::course_club_info(course_id)?;
    Ok(Json(club_id))
}

#[rocket::head("/admin/course_club_edit?<course_id>&<club_id>")]
pub fn course_club_edit(session: UserSession, course_id: u64, club_id: Option<u32>) -> Result<(), Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };
    if !session.right.right_club_write {
        return Err(Error::RightClubMissing);
    };

    crate::db::course::course_club_edit(course_id, club_id)?;
    Ok(())
}

#[rocket::get("/admin/course_statistic_class?<course_id>")]
pub fn course_statistic_class(
    session: UserSession,
    course_id: u32,
) -> Result<Json<Vec<(Event, u64, u64, u64)>>, Error> {
    if !session.right.right_course_read {
        return Err(Error::RightCourseMissing);
    };

    let stats = crate::db::course::course_statistic_class(course_id)?;
    Ok(Json(stats))
}

#[rocket::get("/admin/course_statistic_leader?<course_id>")]
pub fn course_statistic_leader(session: UserSession, course_id: u32) -> Result<Json<Vec<(User, u64)>>, Error> {
    if !session.right.right_course_read {
        return Err(Error::RightCourseMissing);
    };

    let stats = crate::db::course::course_statistic_leader(course_id)?;
    Ok(Json(stats))
}

#[rocket::get("/admin/course_statistic_leader1?<course_id>&<leader_id>")]
pub fn course_statistic_leader1(
    session: UserSession,
    course_id: u32,
    leader_id: u64,
) -> Result<Json<Vec<Event>>, Error> {
    if !session.right.right_course_read {
        return Err(Error::RightCourseMissing);
    };

    let stats = crate::db::course::course_statistic_leader1(course_id, leader_id)?;
    Ok(Json(stats))
}

#[rocket::get("/admin/course_statistic_supporter?<course_id>")]
pub fn course_statistic_supporter(session: UserSession, course_id: u32) -> Result<Json<Vec<(User, u64)>>, Error> {
    if !session.right.right_course_read {
        return Err(Error::RightCourseMissing);
    };

    let stats = crate::db::course::course_statistic_supporter(course_id)?;
    Ok(Json(stats))
}

#[rocket::get("/admin/course_statistic_supporter1?<course_id>&<supporter_id>")]
pub fn course_statistic_supporter1(
    session: UserSession,
    course_id: u32,
    supporter_id: u64,
) -> Result<Json<Vec<Event>>, Error> {
    if !session.right.right_course_read {
        return Err(Error::RightCourseMissing);
    };

    let stats = crate::db::course::course_statistic_supporter1(course_id, supporter_id)?;
    Ok(Json(stats))
}

#[rocket::get("/admin/course_statistic_participant?<course_id>")]
pub fn course_statistic_participant(session: UserSession, course_id: u32) -> Result<Json<Vec<(User, u64)>>, Error> {
    if !session.right.right_course_read {
        return Err(Error::RightCourseMissing);
    };

    let stats = crate::db::course::course_statistic_participant(course_id)?;
    Ok(Json(stats))
}

#[rocket::get("/admin/course_statistic_participant1?<course_id>&<participant_id>")]
pub fn course_statistic_participant1(
    session: UserSession,
    course_id: u32,
    participant_id: u64,
) -> Result<Json<Vec<Event>>, Error> {
    if !session.right.right_course_read {
        return Err(Error::RightCourseMissing);
    };

    let stats = crate::db::course::course_statistic_participant1(course_id, participant_id)?;
    Ok(Json(stats))
}

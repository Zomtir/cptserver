pub mod leader;
pub mod owner;
pub mod participant;
pub mod supporter;

use rocket::serde::json::Json;

use crate::common::{Acceptance, Affiliation, Event, Occurrence, User, WebBool, WebDateTime};
use crate::error::Error;
use crate::session::{Credential, UserSession};

#[rocket::get(
    "/admin/event_list?<begin>&<end>&<location_id>&<occurrence>&<acceptance>&<course_true>&<course_id>&<owner_id>"
)]
pub fn event_list(
    session: UserSession,
    begin: Option<WebDateTime>,
    end: Option<WebDateTime>,
    location_id: Option<u64>,
    occurrence: Option<Occurrence>,
    acceptance: Option<Acceptance>,
    course_true: Option<WebBool>,
    course_id: Option<u32>,
    owner_id: Option<u64>,
) -> Result<Json<Vec<Event>>, Error> {
    if !session.right.right_event_read {
        return Err(Error::RightEventMissing);
    };

    let events = crate::db::event::event_list(
        begin.map(|dt| dt.to_naive()),
        end.map(|dt| dt.to_naive()),
        location_id,
        occurrence,
        acceptance,
        course_true.map(|b| b.to_bool()),
        course_id,
        owner_id,
    )?;
    Ok(Json(events))
}

#[rocket::get("/admin/event_info?<event_id>")]
pub fn event_info(session: UserSession, event_id: u64) -> Result<Json<Event>, Error> {
    if !session.right.right_event_read {
        return Err(Error::RightEventMissing);
    };

    Ok(Json(crate::db::event::event_info(event_id)?))
}

#[rocket::get("/admin/event_credential?<event_id>")]
pub fn event_credential(session: UserSession, event_id: u64) -> Result<Json<Credential>, Error> {
    if !session.right.right_event_read {
        return Err(Error::RightEventMissing);
    };

    Ok(Json(crate::db::event::event_credential(event_id)?))
}

#[rocket::post("/admin/event_create?<course_id>", format = "application/json", data = "<event>")]
pub fn event_create(session: UserSession, course_id: Option<u32>, mut event: Json<Event>) -> Result<String, Error> {
    if !session.right.right_event_write {
        return Err(Error::RightEventMissing);
    };

    if course_id.is_some() && !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    crate::common::validate_event_dates(&mut event)?;

    let id = crate::db::event::event_create(&event, &Acceptance::Draft, course_id)?;
    Ok(id.to_string())
}

#[rocket::post("/admin/event_edit?<event_id>", format = "application/json", data = "<event>")]
pub fn event_edit(session: UserSession, event_id: u64, mut event: Json<Event>) -> Result<(), Error> {
    if !session.right.right_event_write {
        return Err(Error::RightEventMissing);
    };

    crate::common::validate_event_dates(&mut event)?;

    crate::db::event::event_edit(event_id, &event)?;
    Ok(())
}

#[rocket::post("/admin/event_password_edit?<event_id>", format = "text/plain", data = "<password>")]
pub fn event_password_edit(session: UserSession, event_id: u64, password: String) -> Result<(), Error> {
    if !session.right.right_event_write {
        return Err(Error::RightEventMissing);
    };

    crate::db::event::event_password_edit(event_id, password)?;
    Ok(())
}

#[rocket::get("/admin/event_course_info?<event_id>")]
pub fn event_course_info(session: UserSession, event_id: u64) -> Result<Json<Option<u32>>, Error> {
    if !session.right.right_event_read {
        return Err(Error::RightEventMissing);
    };

    let course_id = crate::db::event::event_course_info(event_id)?;
    Ok(Json(course_id))
}

#[rocket::head("/admin/event_course_edit?<event_id>&<course_id>")]
pub fn event_course_edit(session: UserSession, event_id: u64, course_id: Option<u32>) -> Result<(), Error> {
    if !session.right.right_event_write {
        return Err(Error::RightEventMissing);
    };
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    crate::db::event::event_course_edit(event_id, course_id)?;
    Ok(())
}

#[rocket::head("/admin/event_delete?<event_id>")]
pub fn event_delete(session: UserSession, event_id: u64) -> Result<(), Error> {
    if !session.right.right_event_write {
        return Err(Error::RightEventMissing);
    };

    crate::db::event::event_delete(event_id)?;
    Ok(())
}

#[rocket::head("/admin/event_accept?<event_id>")]
pub fn event_accept(session: UserSession, event_id: u64) -> Result<(), Error> {
    if !session.right.right_event_write {
        return Err(Error::RightEventMissing);
    };

    // Perhaps lock the DB during checking and potentially accepting the request
    let event: Event = crate::db::event::event_info(event_id)?;

    // Check if the event is somewhat reasonable
    if !crate::common::is_event_valid(&event) {
        return Err(Error::EventWindowInvalid);
    }

    crate::db::event::event_acceptance_edit(event.id, &Acceptance::Accepted)?;
    Ok(())
}

#[rocket::head("/admin/event_reject?<event_id>")]
pub fn event_reject(session: UserSession, event_id: u64) -> Result<(), Error> {
    if !session.right.right_event_write {
        return Err(Error::RightEventMissing);
    };

    crate::db::event::event_acceptance_edit(event_id, &Acceptance::Rejected)?;
    Ok(())
}

#[rocket::head("/admin/event_suspend?<event_id>")]
pub fn event_suspend(session: UserSession, event_id: u64) -> Result<(), Error> {
    if !session.right.right_event_write {
        return Err(Error::RightEventMissing);
    };

    crate::db::event::event_acceptance_edit(event_id, &Acceptance::Pending)?;
    Ok(())
}

#[rocket::head("/admin/event_withdraw?<event_id>")]
pub fn event_withdraw(session: UserSession, event_id: u64) -> Result<(), Error> {
    if !session.right.right_event_write {
        return Err(Error::RightEventMissing);
    };

    crate::db::event::event_acceptance_edit(event_id, &Acceptance::Draft)?;
    Ok(())
}

#[rocket::get("/admin/event_statistic_packlist?<event_id>&<category1>&<category2>&<category3>")]
pub fn statistic_packlist(
    session: UserSession,
    event_id: u64,
    category1: Option<u32>,
    category2: Option<u32>,
    category3: Option<u32>,
) -> Result<Json<Vec<(User, u32, u32, u32)>>, Error> {
    if !session.right.right_event_read {
        return Err(Error::RightEventMissing);
    };

    let stats = crate::db::event::event_statistic_packlist(event_id, category1, category2, category3)?;
    Ok(Json(stats))
}

#[rocket::get("/admin/event_statistic_division?<event_id>")]
pub fn statistic_division(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>, Error> {
    if !session.right.right_event_read {
        return Err(Error::RightEventMissing);
    };

    let stats = crate::db::event::event_statistic_division(event_id)?;
    Ok(Json(stats))
}

#[rocket::get("/admin/event_statistic_organisation?<event_id>&<organisation_id>")]
pub fn statistic_organisation(
    session: UserSession,
    event_id: u64,
    organisation_id: u64,
) -> Result<Json<Vec<Affiliation>>, Error> {
    if !session.right.right_event_read {
        return Err(Error::RightEventMissing);
    };

    let stats = crate::db::event::event_statistic_organisation(event_id, organisation_id)?;
    Ok(Json(stats))
}

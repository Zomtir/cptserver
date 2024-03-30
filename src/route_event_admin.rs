use rocket::serde::json::Json;

use crate::common::{Event, EventStatus, User, WebBool, WebDateTime};
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/admin/event_list?<begin>&<end>&<status>&<location_id>&<course_true>&<course_id>&<owner_id>")]
pub fn event_list(
    session: UserSession,
    begin: Option<WebDateTime>,
    end: Option<WebDateTime>,
    status: Option<EventStatus>,
    location_id: Option<u64>,
    course_true: Option<WebBool>,
    course_id: Option<u64>,
    owner_id: Option<u64>,
) -> Result<Json<Vec<Event>>, Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    let events = crate::db_event::event_list(
        begin.map(|dt| dt.to_naive()),
        end.map(|dt| dt.to_naive()),
        status,
        location_id,
        course_true.map(|b| b.to_bool()),
        course_id,
        owner_id,
    )?;
    Ok(Json(events))
}

#[rocket::get("/admin/event_info?<event_id>")]
pub fn event_info(session: UserSession, event_id: u64) -> Result<Json<Event>, Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    Ok(Json(crate::db_event::event_info(event_id)?))
}

#[rocket::post("/admin/event_create?<course_id>", format = "application/json", data = "<event>")]
pub fn event_create(session: UserSession, course_id: Option<u64>, mut event: Json<Event>) -> Result<String, Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    if course_id == None || !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    crate::common::validate_event_dates(&mut event)?;

    let id = crate::db_event::event_create(&event, "OCCURRING", course_id)?;
    Ok(id.to_string())
}

#[rocket::post("/admin/event_edit?<event_id>", format = "application/json", data = "<event>")]
pub fn event_edit(session: UserSession, event_id: u64, mut event: Json<Event>) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::common::validate_event_dates(&mut event)?;

    crate::db_event::event_edit(event_id, &event)?;
    Ok(())
}

#[rocket::post("/admin/event_password_edit?<event_id>", format = "text/plain", data = "<password>")]
pub fn event_password_edit(session: UserSession, event_id: u64, password: String) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::db_event::event_password_edit(event_id, password)?;
    Ok(())
}

#[rocket::head("/admin/event_delete?<event_id>")]
pub fn event_delete(session: UserSession, event_id: u64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::db_event::event_delete(event_id)?;
    Ok(())
}

#[rocket::head("/admin/event_accept?<event_id>")]
pub fn event_accept(session: UserSession, event_id: u64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    // Perhaps lock the DB during checking and potentially accepting the request
    let event: Event = crate::db_event::event_info(event_id)?;

    // The check is here intentional to be able to return early although it is also checked during is_event_free
    if !crate::common::is_event_valid(&event) {
        return Err(Error::EventWindowInvalid);
    }

    let status_update = match crate::db_event::event_free_true(&event)? {
        false => "REJECTED",
        true => "OCCURRING",
    };

    crate::db_event::event_status_edit(event.id, Some("PENDING"), status_update)?;
    Ok(())
}

#[rocket::head("/admin/event_deny?<event_id>")]
pub fn event_deny(session: UserSession, event_id: u64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::db_event::event_status_edit(event_id, Some("PENDING"), "REJECTED")?;
    Ok(())
}

#[rocket::head("/admin/event_cancel?<event_id>")]
pub fn event_cancel(session: UserSession, event_id: u64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::db_event::event_status_edit(event_id, Some("OCCURRING"), "REJECTED")?;
    Ok(())
}

#[rocket::head("/admin/event_suspend?<event_id>")]
pub fn event_suspend(session: UserSession, event_id: u64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    let event: Event = crate::db_event::event_info(event_id)?;

    if event.status != "OCCURRING" && event.status != "REJECTED" {
        return Err(Error::EventStatusConflict);
    }

    crate::db_event::event_status_edit(event_id, None, "PENDING")?;
    Ok(())
}

#[rocket::get("/admin/event_owner_pool?<event_id>")]
pub fn event_owner_pool(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>, Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    let users = crate::db_event::event_owner_pool(event_id)?;
    Ok(Json(users))
}

#[rocket::get("/admin/event_owner_list?<event_id>")]
pub fn event_owner_list(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>, Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    let users = crate::db_event::event_owner_list(event_id)?;
    Ok(Json(users))
}

#[rocket::head("/admin/event_owner_add?<event_id>&<user_id>")]
pub fn event_owner_add(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::db_event::event_owner_add(event_id, user_id)?;
    Ok(())
}

#[rocket::head("/admin/event_owner_remove?<event_id>&<user_id>")]
pub fn event_owner_remove(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::db_event::event_owner_remove(event_id, user_id)?;
    Ok(())
}

#[rocket::get("/admin/event_participant_pool?<event_id>")]
pub fn event_participant_pool(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>, Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    let users = crate::db_event::event_participant_pool(event_id)?;
    Ok(Json(users))
}

#[rocket::get("/admin/event_participant_list?<event_id>")]
pub fn event_participant_list(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>, Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    let users = crate::db_event::event_participant_list(event_id)?;
    Ok(Json(users))
}

#[rocket::head("/admin/event_participant_add?<event_id>&<user_id>")]
pub fn event_participant_add(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::db_event::event_participant_add(event_id, user_id)?;
    Ok(())
}

#[rocket::head("/admin/event_participant_remove?<event_id>&<user_id>")]
pub fn event_participant_remove(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::db_event::event_participant_remove(event_id, user_id)?;
    Ok(())
}

use rocket::serde::json::Json;

use crate::common::{Slot, SlotStatus, WebDateTime, User};
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/admin/event_list?<begin>&<end>&<status>&<location_id>&<course_true>&<owner_id>")]
pub fn event_list(
    session: UserSession,
    begin: WebDateTime,
    end: WebDateTime,
    status: Option<SlotStatus>,
    location_id: Option<u64>,
    course_true: Option<bool>,
    owner_id: Option<u64>,
) -> Result<Json<Vec<Slot>>, Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    let slots = crate::db_slot::list_slots(
        Some(begin.to_naive()),
        Some(end.to_naive()),
        status,
        location_id,
        course_true,
        None,
        owner_id,
    )?;
    Ok(Json(slots))
}

#[rocket::get("/admin/event_info?<slot_id>")]
pub fn event_info(session: UserSession, slot_id: u64) -> Result<Json<Slot>, Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    Ok(Json(crate::db_slot::slot_info(slot_id)?))
}

#[rocket::post("/admin/event_create?<course_id>", format = "application/json", data = "<slot>")]
pub fn event_create(session: UserSession, course_id: Option<u64>, mut slot: Json<Slot>) -> Result<String, Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    if course_id == None || !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    crate::common::validate_slot_dates(&mut slot)?;

    let id = crate::db_slot::slot_create(&slot, "OCCURRING", course_id)?;
    Ok(id.to_string())
}

#[rocket::post("/admin/event_edit?<slot_id>", format = "application/json", data = "<slot>")]
pub fn event_edit(session: UserSession, slot_id: u64, mut slot: Json<Slot>) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::common::validate_slot_dates(&mut slot)?;

    crate::db_slot::edit_slot(slot_id, &slot)?;
    Ok(())
}

#[rocket::post("/admin/event_edit_password?<slot_id>", format = "text/plain", data = "<password>")]
pub fn event_edit_password(session: UserSession, slot_id: u64, password: String) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::db_slot::edit_slot_password(slot_id, password)?;
    Ok(())
}

#[rocket::head("/admin/event_delete?<slot_id>")]
pub fn event_delete(session: UserSession, slot_id: u64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::db_slot::slot_delete(slot_id)?;
    Ok(())
}

#[rocket::head("/admin/event_accept?<slot_id>")]
pub fn event_accept(session: UserSession, slot_id: u64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    // Perhaps lock the DB during checking and potentially accepting the request
    let slot: Slot = crate::db_slot::slot_info(slot_id)?;

    // The check is here intentional to be able to return early although it is also checked during is_slot_free
    if !crate::common::is_slot_valid(&slot) {
        return Err(Error::SlotWindowInvalid);
    }

    let status_update = match crate::db_slot::is_slot_free(&slot)? {
        false => "REJECTED",
        true => "OCCURRING",
    };

    crate::db_slot::edit_slot_status(slot.id, Some("PENDING"), status_update)?;
    Ok(())
}

#[rocket::head("/admin/event_deny?<slot_id>")]
pub fn event_deny(session: UserSession, slot_id: u64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::db_slot::edit_slot_status(slot_id, Some("PENDING"), "REJECTED")?;
    Ok(())
}

#[rocket::head("/admin/event_cancel?<slot_id>")]
pub fn event_cancel(session: UserSession, slot_id: u64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::db_slot::edit_slot_status(slot_id, Some("OCCURRING"), "REJECTED")?;
    Ok(())
}

#[rocket::head("/admin/event_suspend?<slot_id>")]
pub fn event_suspend(session: UserSession, slot_id: u64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    let slot: Slot = crate::db_slot::slot_info(slot_id)?;

    if slot.status != "OCCURRING" && slot.status != "REJECTED" {
        return Err(Error::SlotStatusConflict);
    }

    crate::db_slot::edit_slot_status(slot_id, None, "PENDING")?;
    Ok(())
}

#[rocket::get("/admin/event_owner_pool?<slot_id>")]
pub fn event_owner_pool(session: UserSession, slot_id: u64) -> Result<Json<Vec<User>>, Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    let users = crate::db_slot::slot_owner_pool(slot_id)?;
    Ok(Json(users))
}

#[rocket::get("/admin/event_owner_list?<slot_id>")]
pub fn event_owner_list(session: UserSession, slot_id: u64) -> Result<Json<Vec<User>>, Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    let users = crate::db_slot::slot_owner_list(slot_id)?;
    Ok(Json(users))
}

#[rocket::head("/admin/event_owner_add?<slot_id>&<user_id>")]
pub fn event_owner_add(session: UserSession, slot_id: u64, user_id: u64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::db_slot::slot_owner_add(slot_id, user_id)?;
    Ok(())
}

#[rocket::head("/admin/event_owner_remove?<slot_id>&<user_id>")]
pub fn event_owner_remove(session: UserSession, slot_id: u64, user_id: u64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::db_slot::slot_owner_remove(slot_id, user_id)?;
    Ok(())
}

#[rocket::get("/admin/event_participant_pool?<slot_id>")]
pub fn event_participant_pool(session: UserSession, slot_id: u64) -> Result<Json<Vec<User>>, Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    let users = crate::db_slot::slot_participant_pool(slot_id)?;
    Ok(Json(users))
}

#[rocket::get("/admin/event_participant_list?<slot_id>")]
pub fn event_participant_list(session: UserSession, slot_id: u64) -> Result<Json<Vec<User>>, Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    let users = crate::db_slot::slot_participant_list(slot_id)?;
    Ok(Json(users))
}

#[rocket::head("/admin/event_participant_add?<slot_id>&<user_id>")]
pub fn event_participant_add(session: UserSession, slot_id: u64, user_id: u64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::db_slot::slot_participant_add(slot_id, user_id)?;
    Ok(())
}

#[rocket::head("/admin/event_participant_remove?<slot_id>&<user_id>")]
pub fn event_participant_remove(session: UserSession, slot_id: u64, user_id: u64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::db_slot::slot_participant_remove(slot_id, user_id)?;
    Ok(())
}

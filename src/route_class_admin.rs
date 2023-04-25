use rocket::serde::json::Json;

use crate::common::{Slot, User};
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/admin/class_list?<course_id>")]
pub fn class_list(session: UserSession, course_id: i64) -> Result<Json<Vec<Slot>>, Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    match crate::db_slot::list_slots(None, None, None, Some(course_id), None)? {
        slots => Ok(Json(slots)),
    }
}

#[rocket::post("/admin/class_create?<course_id>", format = "application/json", data = "<slot>")]
pub fn class_create(session: UserSession, course_id: i64, mut slot: Json<Slot>) -> Result<String, Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    crate::common::validate_slot_dates(&mut slot)?;

    let id = crate::db_slot::create_slot(&slot, "OCCURRING", Some(course_id))?;
    Ok(id.to_string())
}

#[rocket::post("/admin/class_edit?<slot_id>", format = "application/json", data = "<slot>")]
pub fn class_edit(session: UserSession, slot_id: i64, mut slot: Json<Slot>) -> Result<(), Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    match crate::db_slot::is_slot_in_any_course(slot_id)? {
        false => return Err(Error::SlotCourseMissing),
        true => (),
    };

    crate::common::validate_slot_dates(&mut slot)?;

    crate::db_slot::edit_slot(slot_id, &slot)?;
    Ok(())
}

#[rocket::post("/admin/class_edit_password?<slot_id>", format = "text/plain", data = "<password>")]
pub fn class_edit_password(session: UserSession, slot_id: i64, password: String) -> Result<(), Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    match crate::db_slot::is_slot_in_any_course(slot_id)? {
        false => return Err(Error::SlotCourseMissing),
        true => (),
    };

    crate::db_slot::edit_slot_password(slot_id, password)?;
    Ok(())
}

#[rocket::head("/admin/class_delete?<slot_id>")]
pub fn class_delete(session: UserSession, slot_id: i64) -> Result<(), Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    crate::db_slot::delete_slot(slot_id)?;
    Ok(())
}

#[rocket::get("/admin/class_owner_list?<slot_id>")]
pub fn class_owner_list(session: UserSession, slot_id: i64) -> Result<Json<Vec<User>>, Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    match crate::db_slot::is_slot_in_any_course(slot_id)? {
        false => return Err(Error::SlotCourseMissing),
        true => (),
    };

    match crate::db_slot::get_slot_owners(slot_id)? {
        users => Ok(Json(users)),
    }
}

#[rocket::head("/admin/class_owner_add?<slot_id>&<user_id>")]
pub fn class_owner_add(session: UserSession, slot_id: i64, user_id: i64) -> Result<(), Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    match crate::db_slot::is_slot_in_any_course(slot_id)? {
        false => return Err(Error::SlotCourseMissing),
        true => (),
    };

    crate::db_slot::add_slot_owner(slot_id, user_id)?;
    Ok(())
}

#[rocket::head("/admin/class_owner_remove?<slot_id>&<user_id>")]
pub fn class_owner_remove(session: UserSession, slot_id: i64, user_id: i64) -> Result<(), Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    match crate::db_slot::is_slot_in_any_course(slot_id)? {
        false => return Err(Error::SlotCourseMissing),
        true => (),
    };

    crate::db_slot::remove_slot_owner(slot_id, user_id)?;
    Ok(())
}

#[rocket::get("/admin/class_participant_list?<slot_id>")]
pub fn class_participant_list(session: UserSession, slot_id: i64) -> Result<Json<Vec<User>>, Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    match crate::db_slot::is_slot_in_any_course(slot_id)? {
        false => return Err(Error::SlotCourseMissing),
        true => (),
    };

    match crate::db_slot::list_slot_participants(slot_id)? {
        users => Ok(Json(users)),
    }
}

#[rocket::head("/admin/class_participant_add?<slot_id>&<user_id>")]
pub fn class_participant_add(session: UserSession, slot_id: i64, user_id: i64) -> Result<(), Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    match crate::db_slot::is_slot_in_any_course(slot_id)? {
        false => return Err(Error::SlotCourseMissing),
        true => (),
    };

    crate::db_slot::add_slot_participant(slot_id, user_id)?;
    Ok(())
}

#[rocket::head("/admin/class_participant_remove?<slot_id>&<user_id>")]
pub fn class_participant_remove(session: UserSession, slot_id: i64, user_id: i64) -> Result<(), Error> {
    if !session.right.admin_courses {
        return Err(Error::RightCourseMissing);
    };

    match crate::db_slot::is_slot_in_any_course(slot_id)? {
        false => return Err(Error::SlotCourseMissing),
        true => (),
    };

    crate::db_slot::remove_slot_participant(slot_id, user_id)?;
    Ok(())
}

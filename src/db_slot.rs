use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Location, Slot, User};
use crate::db::get_pool_conn;
use crate::error::Error;

/*
 * METHODS
 */

pub fn get_slot_info(slot_id: i64) -> Result<Slot, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT slot_id, slot_key, s.title, l.location_id, l.location_key, l.title, s.begin, s.end, s.status, s.course_id
        FROM slots s
        JOIN locations l ON l.location_id = s.location_id
        WHERE slot_id = :slot_id",
    )?;
    let params = params! {
        "slot_id" => slot_id,
    };
    let map = |(
        slot_id,
        slot_key,
        slot_title,
        location_id,
        location_key,
        location_title,
        begin,
        end,
        status,
        course_id,
    )| Slot {
        id: slot_id,
        key: slot_key,
        pwd: None,
        title: slot_title,
        begin,
        end,
        status: status,
        location: Location {
            id: location_id,
            key: location_key,
            title: location_title,
        },
        course_id: course_id,
    };

    let mut slots = conn.exec_map(&stmt, &params, &map)?;

    match slots.pop() {
        None => Err(Error::SlotMissing),
        Some(slot) => Ok(slot),
    }
}

// TODO make a check that status is not an invalid string by implementing a proper trait
// TODO should status even be a searchable criteria? if so, please make it enum with FromFormValue::default()
pub fn list_slots(
    mut begin: Option<chrono::NaiveDate>,
    mut end: Option<chrono::NaiveDate>,
    status: Option<String>,
    course_id: Option<i64>,
    owner_id: Option<i64>,
) -> Result<Vec<Slot>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT s.slot_id, s.slot_key, s.title, l.location_id, l.location_key, l.title, s.begin, s.end, s.status
        FROM slots s
        JOIN locations l ON l.location_id = s.location_id
        LEFT JOIN slot_owners o ON s.slot_id = o.slot_id
        WHERE (:frame_start IS NULL OR :frame_start < s.begin)
        AND (:frame_stop IS NULL OR :frame_stop > s.begin)
        AND (:status IS NULL OR :status = s.status)
        AND (:course_id IS NULL OR :course_id = s.course_id)
        AND (:owner_id IS NULL OR :owner_id = o.user_id)
        GROUP BY s.slot_id")?;

    if begin.is_none() || begin < crate::config::CONFIG_SLOT_DATE_MIN() {
        begin = crate::config::CONFIG_SLOT_DATE_MIN();
    }

    if end.is_none() || end < crate::config::CONFIG_SLOT_DATE_MAX() {
        end = crate::config::CONFIG_SLOT_DATE_MAX();
    }

    let params = params! {
        "frame_start" => &begin,
        "frame_stop" => &end,
        "status" => &status,
        "course_id" => &course_id,
        "owner_id" => &owner_id,
    };

    let map = |(
        slot_id,
        slot_key,
        slot_title,
        location_id,
        location_key,
        location_title,
        begin,
        end,
        status,
    ): (i64, String, String, u32, _, _, _, _, String)| Slot {
        id: slot_id,
        key: slot_key,
        pwd: None,
        title: slot_title,
        begin,
        end,
        status,
        location: Location {
            id: location_id,
            key: location_key,
            title: location_title,
        },
        course_id: None,
    };

    let slots = conn.exec_map(&stmt, &params, &map)?;
    Ok(slots)
}

pub fn create_slot(slot: &Slot, status: &str, course_id: Option<i64>) -> Result<i64, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO slots (slot_key, pwd, title, status, autologin, location_id, begin, end, course_id)
        SELECT :slot_key, :pwd, :title, :status, :autologin, :location_id, :begin, :end, :course_id");

    let params = params! {
        "slot_key" => crate::common::random_string(8),
        "pwd" => crate::common::random_string(8),
        "title" => &slot.title,
        "status" => status,
        "autologin" => false,
        "location_id" => &slot.location.id,
        "begin" => &slot.begin,
        "end" => &slot.end,
        "course_id" => &course_id,
    };

    conn.exec_drop(&stmt.unwrap(), &params)?;

    crate::db::get_last_id(&mut conn)
}

pub fn edit_slot(slot_id: i64, slot: &Slot) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE slots
        SET
            title = :title,
            location_id = :location_id,
            begin = :begin,
            end = :end
        WHERE slot_id = :slot_id",
    )?;

    let params = params! {
        "slot_id" => &slot_id,
        "title" => &slot.title,
        "location_id" => &slot.location.id,
        "begin" => &slot.begin,
        "end" => &slot.end,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn edit_slot_status(slot_id: i64, status_required: &str, status_update: &str) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE slots SET
        status = :status_update
        WHERE slot_id = :slot_id AND status = :status_required",
    )?;
    let params = params! {
        "slot_id" => slot_id,
        "status_required" => status_required,
        "status_update" => status_update,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn edit_slot_password(slot_id: i64, password: String) -> Result<(), Error> {
    let password = crate::common::validate_clear_password(password)?;

    let mut conn: PooledConn = get_pool_conn();

    let stmt = conn.prep(
        "UPDATE slots SET pwd = :pwd
        WHERE slot_id = :slot_id",
    )?;

    let params = params! {
        "slot_id" => &slot_id,
        "pwd" => &password,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn delete_slot(slot_id: i64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE s
        FROM slots s
        WHERE slot_id = :slot_id",
    )?;

    let params = params! {
        "slot_id" => &slot_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn is_slot_free(slot: &Slot) -> Result<bool, Error> {
    if !crate::common::is_slot_valid(slot) {
        return Ok(false);
    };

    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT COUNT(1)
        FROM slots
        WHERE location_id = :location_id
        AND NOT (end <= :begin OR begin >= :end)
        AND status = 'OCCURRING'",
    )?;
    let params = params! {
        "location_id" => &slot.location.id,
        "begin" => &slot.begin,
        "end" => &slot.end,
    };

    let count = conn.exec_first::<i64, _, _>(&stmt, &params)?;
    match count {
        None => Err(Error::DatabaseError),
        Some(count) => Ok(count == 0),
    }
}

/* PARTICIPANT RELATED */

pub fn get_slot_candidates(slot_id: i64) -> Result<Vec<User>, Error> {
    let mut conn : PooledConn = get_pool_conn();
    // TODO this is way more complicated than just then enabled users...
    // course invites, slot invites, team invites
    let stmt = conn.prep("SELECT user_id, user_key, firstname, lastname FROM users
                          WHERE enabled = TRUE")?;

    let map = |(user_id, user_key, firstname, lastname)|
        User::from_info(user_id, user_key, firstname, lastname);

    // TODO level check threshold if existent

    let users = conn.exec_map(&stmt,params::Params::Empty,&map)?;
    Ok(users)
}

/* EVENT RELATED */

pub fn get_slot_owners(slot_id: i64) -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname
        FROM slot_owners
        JOIN users u ON u.user_id = slot_owners.user_id
        WHERE slot_owners.slot_id = :slot_id",
    )?;
    let params = params! {
        "slot_id" => slot_id,
    };
    let map = |(user_id, user_key, firstname, lastname): (i64, String, String, String)| {
        User::from_info(user_id, user_key, firstname, lastname)
    };

    let users = conn.exec_map(&stmt, &params, &map)?;
    Ok(users)
}

pub fn is_slot_owner(slot_id: i64, user_id: i64) -> Result<bool, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT COUNT(1)
        FROM slot_owners
        WHERE slot_id = :slot_id AND user_id = :user_id",
    )?;
    let params = params! {
        "slot_id" => slot_id,
        "user_id" => user_id,
    };

    match conn.exec_first::<i64, _, _>(&stmt, &params)? {
        None => Ok(false),
        Some(count) => Ok(count == 1),
    }
}

pub fn add_slot_owner(slot_id: i64, user_id: i64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO slot_owners (slot_id, user_id)
        VALUES (:slot_id, :user_id)",
    )?;
    let params = params! {
        "slot_id" => &slot_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn remove_slot_owner(slot_id: i64, user_id: i64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE FROM slot_owners
        WHERE slot_id = :slot_id AND user_id = :user_id",
    )?;

    let params = params! {
        "slot_id" => &slot_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

/* COURSE RELATED */

pub fn is_slot_moderator(slot_id: i64, user_id: i64) -> Result<bool, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT COUNT(1)
        FROM slots s
        LEFT JOIN course_moderators m ON m.course_id = s.course_id
        WHERE s.slot_id = :slot_id AND m.user_id = :user_id;",
    )?;

    let params = params! {
        "slot_id" => slot_id,
        "user_id" => user_id,
    };

    match conn.exec_first::<u32, _, _>(&stmt, &params)? {
        None => Ok(false),
        Some(count) => Ok(count == 1),
    }
}

pub fn edit_slot_in_course(slot_id: i64, course_id: Option<i64>) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();

    let stmt = conn.prep(
        "UPDATE slots
        SET course_id = :course_id
        WHERE slot_id = :slot_id",
    )?;

    let params = params! {
        "slot_id" => &slot_id,
        "course_id" => &course_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn is_slot_in_course(slot_id: i64, course_id: i64) -> Result<bool, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT COUNT(1)
        FROM slots
        WHERE slot_id = :slot_id AND course_id = :course_id",
    )?;
    let params = params! {
        "slot_id" => slot_id,
        "course_id" => course_id,
    };

    match conn.exec_first::<i64, _, _>(&stmt, &params)? {
        None => Ok(false),
        Some(count) => Ok(count == 1),
    }
}

pub fn is_slot_in_any_course(slot_id: i64) -> Result<bool, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT COUNT(1)
        FROM slots
        WHERE slot_id = :slot_id AND course_id IS NOT NULL;",
    )?;
    let params = params! {
        "slot_id" => slot_id,
    };

    match conn.exec_first::<i64, _, _>(&stmt, &params)? {
        None => Ok(false),
        Some(count) => Ok(count == 1),
    }
}

/* PARTICIPANT RELATED */

pub fn list_slot_participants(slot_id: i64) -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname
        FROM slot_enrollments e
        JOIN users u ON u.user_id = e.user_id
        WHERE slot_id = :slot_id;",
    )?;
    let params = params! {
        "slot_id" => slot_id,
    };
    let map = |(user_id, user_key, firstname, lastname)| {
        User::from_info(user_id, user_key, firstname, lastname)
    };

    let users = conn.exec_map(&stmt, &params, &map)?;
    Ok(users)
}

pub fn add_slot_participant(slot_id: i64, user_id: i64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO slot_enrollments (slot_id, user_id)
        VALUES (:slot_id, :user_id);",
    )?;
    let params = params! {
        "slot_id" => &slot_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn remove_slot_participant(slot_id: i64, user_id: i64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE FROM slot_enrollments
        WHERE slot_id = :slot_id AND user_id = :user_id;",
    )?;

    let params = params! {
        "slot_id" => &slot_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

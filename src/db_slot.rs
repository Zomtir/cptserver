use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Location, Slot, SlotStatus, User};
use crate::db::get_pool_conn;
use crate::error::Error;

/*
 * METHODS
 */

pub fn slot_info(slot_id: i64) -> Result<Slot, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT slot_id, slot_key, s.title, l.location_id, l.location_key, l.title AS location_title, s.begin, s.end, s.status, s.public, s.scrutable, s.note, s.course_id
        FROM slots s
        JOIN locations l ON l.location_id = s.location_id
        WHERE slot_id = :slot_id",
    )?;
    let params = params! {
        "slot_id" => slot_id,
    };

    let mut row: mysql::Row = conn.exec_first(&stmt, &params)?.ok_or(Error::SlotMissing)?;

    let slot = Slot {
        id: row.take("slot_id").unwrap(),
        key: row.take("slot_key").unwrap(),
        pwd: None,
        title: row.take("title").unwrap(),
        begin: row.take("begin").unwrap(),
        end: row.take("end").unwrap(),
        location: Location {
            id: row.take("location_id").unwrap(),
            key: row.take("location_key").unwrap(),
            title: row.take("location_title").unwrap(),
        },
        status: row.take("status").unwrap(),
        public: row.take("public").unwrap(),
        scrutable: row.take("scrutable").unwrap(),
        note: row.take("note").unwrap(),
        course_id: row.take("course_id").unwrap(),
    };

    Ok(slot)
}

pub fn list_slots(
    mut begin: Option<chrono::NaiveDate>,
    mut end: Option<chrono::NaiveDate>,
    status: Option<SlotStatus>,
    location_id: Option<i64>,
    course_true: Option<bool>,
    course_id: Option<i64>,
    owner_id: Option<i64>,
) -> Result<Vec<Slot>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT s.slot_id, s.slot_key, s.title, l.location_id, l.location_key, l.title AS location_title, s.begin, s.end, s.status, s.public, s.scrutable, s.note
        FROM slots s
        JOIN locations l ON l.location_id = s.location_id
        LEFT JOIN slot_owners o ON s.slot_id = o.slot_id
        WHERE (:frame_start IS NULL OR :frame_start < s.begin)
        AND (:frame_stop IS NULL OR :frame_stop > s.begin)
        AND (:status IS NULL OR :status = s.status)
        AND (:location_id IS NULL OR :location_id = l.location_id)
        AND (:course_true IS NULL OR (:course_true = TRUE AND :course_id = s.course_id) OR (:course_true = FALSE AND s.course_id IS NULL))
        AND (:owner_id IS NULL OR :owner_id = o.user_id)
        GROUP BY s.slot_id",
    )?;

    if begin.is_none() || begin < crate::config::CONFIG_SLOT_DATE_MIN() {
        begin = crate::config::CONFIG_SLOT_DATE_MIN();
    }

    if end.is_none() || end > crate::config::CONFIG_SLOT_DATE_MAX() {
        end = crate::config::CONFIG_SLOT_DATE_MAX();
    }

    let params = params! {
        "frame_start" => &begin,
        "frame_stop" => &end,
        "status" => &status,
        "location_id" => &location_id,
        "course_true" => &course_true,
        "course_id" => &course_id,
        "owner_id" => &owner_id,
    };

    let rows: Vec<mysql::Row> = conn.exec(&stmt, &params)?;
    let mut slots: Vec<Slot> = Vec::new();

    for mut row in rows {
        let item = Slot {
            id: row.take("slot_id").unwrap(),
            key: row.take("slot_key").unwrap(),
            pwd: None,
            title: row.take("title").unwrap(),
            begin: row.take("begin").unwrap(),
            end: row.take("end").unwrap(),
            location: Location {
                id: row.take("location_id").unwrap(),
                key: row.take("location_key").unwrap(),
                title: row.take("location_title").unwrap(),
            },
            status: row.take("status").unwrap(),
            public: row.take("public").unwrap(),
            scrutable: row.take("scrutable").unwrap(),
            note: row.take("note").unwrap(),
            course_id: None,
        };
        slots.push(item);
    }

    Ok(slots)
}

pub fn slot_create(slot: &Slot, status: &str, course_id: Option<i64>) -> Result<i64, Error> {
    if slot.key.len() < 3 || slot.key.len() > 12 {
        return Err(Error::SlotKeyInvalid);
    }

    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO slots (slot_key, pwd, title, location_id, begin, end, status, public, scrutable, note, course_id)
        SELECT :slot_key, :pwd, :title, :location_id, :begin, :end, :status, :public, :scrutable, :note, :course_id",
    )?;

    let params = params! {
        "slot_key" => &slot.key,
        "pwd" => crate::common::random_string(8),
        "title" => &slot.title,
        "location_id" => &slot.location.id,
        "begin" => &slot.begin,
        "end" => &slot.end,
        "status" => status,
        "public" => slot.public,
        "scrutable" => &slot.scrutable,
        "note" => &slot.note,
        "course_id" => &course_id,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(conn.last_insert_id() as i64)
}

pub fn edit_slot(slot_id: i64, slot: &Slot) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE slots
        SET
            slot_key = :slot_key,
            title = :title,
            location_id = :location_id,
            begin = :begin,
            end = :end,
            public = :public,
            scrutable = :scrutable,
            note = :note
        WHERE slot_id = :slot_id",
    )?;

    let params = params! {
        "slot_id" => &slot_id,
        "slot_key" => &slot.key,
        "title" => &slot.title,
        "location_id" => &slot.location.id,
        "begin" => &slot.begin,
        "end" => &slot.end,
        "public" => &slot.public,
        "scrutable" => &slot.scrutable,
        "note" => &slot.note,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn edit_slot_status(slot_id: i64, status_required: Option<&str>, status_update: &str) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE slots SET
        status = :status_update
        WHERE slot_id = :slot_id
        AND (:status_required IS NULL OR status = :status_required)",
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

pub fn slot_note_edit(slot_id: i64, note: &String) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE slots
        SET note = :note
        WHERE slot_id = :slot_id",
    )?;

    let params = params! {
        "slot_id" => &slot_id,
        "note" => &note,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn slot_delete(slot_id: i64) -> Result<(), Error> {
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

/* EVENT RELATED */

pub fn slot_owner_pool(slot_id: i64) -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();

    let stmt = conn.prep(
        "SELECT users.user_id, users.user_key, users.firstname, users.lastname, users.nickname
        FROM course_owner_teams AS cot
        JOIN teams ON teams.team_id = cot.team_id
        JOIN team_members tm ON teams.team_id = tm.team_id
        JOIN users ON tm.user_id = users.user_id
        JOIN slots ON slots.course_id = cot.course_id
        WHERE slots.slot_id = :slot_id AND users.active = TRUE
        GROUP BY users.user_id",
    )?;

    let params = params! {
        "slot_id" => slot_id,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname)| {
        User::from_info(user_id, user_key, firstname, lastname, nickname)
    };

    let users = conn.exec_map(&stmt, &params, &map)?;
    Ok(users)
}

pub fn slot_owner_list(slot_id: i64) -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname, u.nickname
        FROM slot_owners
        JOIN users u ON u.user_id = slot_owners.user_id
        WHERE slot_owners.slot_id = :slot_id",
    )?;
    let params = params! {
        "slot_id" => slot_id,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname)| {
        User::from_info(user_id, user_key, firstname, lastname, nickname)
    };

    let users = conn.exec_map(&stmt, &params, &map)?;
    Ok(users)
}

pub fn slot_owner_add(slot_id: i64, user_id: i64) -> Result<(), Error> {
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

pub fn slot_owner_remove(slot_id: i64, user_id: i64) -> Result<(), Error> {
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

pub fn slot_owner_true(slot_id: i64, user_id: i64) -> Result<bool, Error> {
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

pub fn slot_course_any(slot_id: i64) -> Result<bool, Error> {
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

pub fn slot_participant_pool(slot_id: i64) -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    // TODO UNION slot invites, team invites
    // TODO level check threshold if existent

    let stmt = conn.prep(
        "SELECT users.user_id, users.user_key, users.firstname, users.lastname, users.nickname
        FROM course_participant_teams AS cpt
        JOIN teams ON teams.team_id = cpt.team_id
        JOIN team_members tm ON teams.team_id = tm.team_id
        JOIN users ON tm.user_id = users.user_id
        JOIN slots ON slots.course_id = cpt.course_id
        WHERE slots.slot_id = :slot_id AND users.active = TRUE
        GROUP BY users.user_id",
    )?;

    let params = params! {
        "slot_id" => slot_id,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname)| {
        User::from_info(user_id, user_key, firstname, lastname, nickname)
    };

    let users = conn.exec_map(&stmt, &params, &map)?;
    Ok(users)
}

pub fn slot_participant_list(slot_id: i64) -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname, u.nickname
        FROM slot_participants p
        JOIN users u ON u.user_id = p.user_id
        WHERE slot_id = :slot_id;",
    )?;
    let params = params! {
        "slot_id" => slot_id,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname)| {
        User::from_info(user_id, user_key, firstname, lastname, nickname)
    };

    let users = conn.exec_map(&stmt, &params, &map)?;
    Ok(users)
}

pub fn slot_participant_add(slot_id: i64, user_id: i64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO slot_participants (slot_id, user_id)
        VALUES (:slot_id, :user_id);",
    )?;
    let params = params! {
        "slot_id" => &slot_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn slot_participant_remove(slot_id: i64, user_id: i64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE FROM slot_participants
        WHERE slot_id = :slot_id AND user_id = :user_id;",
    )?;

    let params = params! {
        "slot_id" => &slot_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

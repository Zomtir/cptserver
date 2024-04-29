use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Event, EventStatus, Location, User};
use crate::db::get_pool_conn;
use crate::error::Error;

/*
 * METHODS
 */

pub fn event_info(event_id: u64) -> Result<Event, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT event_id, event_key, e.title,
            l.location_id, l.location_key, l.name AS location_name, l.description AS location_description,
            e.begin, e.end, e.status, e.public, e.scrutable, e.note, e.course_id
        FROM events e
        JOIN locations l ON l.location_id = e.location_id
        WHERE event_id = :event_id",
    )?;
    let params = params! {
        "event_id" => event_id,
    };

    let mut row: mysql::Row = conn.exec_first(&stmt, &params)?.ok_or(Error::EventMissing)?;

    let event = Event {
        id: row.take("event_id").unwrap(),
        key: row.take("event_key").unwrap(),
        pwd: None,
        title: row.take("title").unwrap(),
        begin: row.take("begin").unwrap(),
        end: row.take("end").unwrap(),
        location: Location {
            id: row.take("location_id").unwrap(),
            key: row.take("location_key").unwrap(),
            name: row.take("location_name").unwrap(),
            description: row.take("location_description").unwrap(),
        },
        status: row.take("status").unwrap(),
        public: row.take("public").unwrap(),
        scrutable: row.take("scrutable").unwrap(),
        note: row.take("note").unwrap(),
        course_id: row.take("course_id").unwrap(),
    };

    Ok(event)
}

pub fn event_list(
    begin: Option<chrono::NaiveDateTime>,
    end: Option<chrono::NaiveDateTime>,
    status: Option<EventStatus>,
    location_id: Option<u64>,
    course_true: Option<bool>,
    course_id: Option<u64>,
    owner_id: Option<u64>,
) -> Result<Vec<Event>, Error> {
    // If there is a search window, make sure it is somewhat correct
    if let (Some(begin), Some(end)) = (begin, end) {
        let delta = end.signed_duration_since(begin);

        if delta < crate::config::CONFIG_SLOT_LIST_TIME_MIN() || delta > crate::config::CONFIG_SLOT_LIST_TIME_MAX() {
            return Err(Error::EventSearchLimit);
        }

        if begin < crate::config::CONFIG_SLOT_LIST_DATE_MIN() || end > crate::config::CONFIG_SLOT_LIST_DATE_MAX() {
            return Err(Error::EventSearchLimit);
        }
    }

    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT e.event_id, e.event_key, e.title,
            l.location_id, l.location_key, l.name AS location_name, l.description AS location_description,
            e.begin, e.end, e.status, e.public, e.scrutable, e.note
        FROM events e
        JOIN locations l ON l.location_id = e.location_id
        LEFT JOIN event_owners o ON e.event_id = o.event_id
        WHERE (:begin IS NULL OR end > :begin)
        AND (:end IS NULL OR begin < :end)
        AND (:status IS NULL OR :status = e.status)
        AND (:location_id IS NULL OR :location_id = l.location_id)
        AND (:course_true IS NULL OR (:course_true = TRUE AND :course_id = e.course_id) OR (:course_true = FALSE AND e.course_id IS NULL))
        AND (:owner_id IS NULL OR :owner_id = o.user_id)
        GROUP BY e.event_id;")?;

    let params = params! {
        "begin" => &begin,
        "end" => &end,
        "status" => &status,
        "location_id" => &location_id,
        "course_true" => &course_true,
        "course_id" => &course_id,
        "owner_id" => &owner_id,
    };

    let rows: Vec<mysql::Row> = conn.exec(&stmt, &params)?;
    let mut events: Vec<Event> = Vec::new();

    for mut row in rows {
        let item = Event {
            id: row.take("event_id").unwrap(),
            key: row.take("event_key").unwrap(),
            pwd: None,
            title: row.take("title").unwrap(),
            begin: row.take("begin").unwrap(),
            end: row.take("end").unwrap(),
            location: Location {
                id: row.take("location_id").unwrap(),
                key: row.take("location_key").unwrap(),
                name: row.take("location_name").unwrap(),
                description: row.take("location_description").unwrap(),
            },
            status: row.take("status").unwrap(),
            public: row.take("public").unwrap(),
            scrutable: row.take("scrutable").unwrap(),
            note: row.take("note").unwrap(),
            course_id: None,
        };
        events.push(item);
    }

    Ok(events)
}

pub fn event_create(event: &Event, status: &str, course_id: Option<u64>) -> Result<u64, Error> {
    if event.key.len() < 3 || event.key.len() > 12 {
        return Err(Error::EventKeyInvalid);
    }

    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO events (event_key, pwd, title, location_id, begin, end, status, public, scrutable, note, course_id)
        SELECT :event_key, :pwd, :title, :location_id, :begin, :end, :status, :public, :scrutable, :note, :course_id",
    )?;

    let params = params! {
        "event_key" => &event.key,
        "pwd" => crate::common::random_string(8),
        "title" => &event.title,
        "location_id" => &event.location.id,
        "begin" => &event.begin,
        "end" => &event.end,
        "status" => status,
        "public" => event.public,
        "scrutable" => &event.scrutable,
        "note" => &event.note,
        "course_id" => &course_id,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(conn.last_insert_id() as u64)
}

pub fn event_edit(event_id: u64, event: &Event) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE events
        SET
            event_key = :event_key,
            title = :title,
            location_id = :location_id,
            begin = :begin,
            end = :end,
            public = :public,
            scrutable = :scrutable,
            note = :note
        WHERE event_id = :event_id",
    )?;

    let params = params! {
        "event_id" => &event_id,
        "event_key" => &event.key,
        "title" => &event.title,
        "location_id" => &event.location.id,
        "begin" => &event.begin,
        "end" => &event.end,
        "public" => &event.public,
        "scrutable" => &event.scrutable,
        "note" => &event.note,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn event_status_edit(event_id: u64, status_required: Option<&str>, status_update: &str) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE events SET
        status = :status_update
        WHERE event_id = :event_id
        AND (:status_required IS NULL OR status = :status_required)",
    )?;
    let params = params! {
        "event_id" => event_id,
        "status_required" => status_required,
        "status_update" => status_update,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn event_password_edit(event_id: u64, password: String) -> Result<(), Error> {
    let password = crate::common::validate_clear_password(password)?;

    let mut conn: PooledConn = get_pool_conn();

    let stmt = conn.prep(
        "UPDATE events SET pwd = :pwd
        WHERE event_id = :event_id",
    )?;

    let params = params! {
        "event_id" => &event_id,
        "pwd" => &password,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn event_note_edit(event_id: u64, note: &String) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE events
        SET note = :note
        WHERE event_id = :event_id",
    )?;

    let params = params! {
        "event_id" => &event_id,
        "note" => &note,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn event_delete(event_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE s
        FROM events s
        WHERE event_id = :event_id",
    )?;

    let params = params! {
        "event_id" => &event_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn event_free_true(event: &Event) -> Result<bool, Error> {
    if !crate::common::is_event_valid(event) {
        return Ok(false);
    };

    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT COUNT(1)
        FROM events
        WHERE location_id = :location_id
        AND NOT (end <= :begin OR begin >= :end)
        AND status = 'OCCURRING'",
    )?;
    let params = params! {
        "location_id" => &event.location.id,
        "begin" => &event.begin,
        "end" => &event.end,
    };

    let count = conn.exec_first::<u64, _, _>(&stmt, &params)?;
    match count {
        None => Err(Error::DatabaseError),
        Some(count) => Ok(count == 0),
    }
}

/* OWNER RELATED */

pub fn event_owner_pool(event_id: u64) -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();

    let stmt = conn.prep(
        "SELECT users.user_id, users.user_key, users.firstname, users.lastname, users.nickname
        FROM course_owner_teams AS cot
        JOIN teams ON teams.team_id = cot.team_id
        JOIN team_members tm ON teams.team_id = tm.team_id
        JOIN users ON tm.user_id = users.user_id
        JOIN events ON events.course_id = cot.course_id
        WHERE events.event_id = :event_id AND users.active = TRUE
        GROUP BY users.user_id",
    )?;

    let params = params! {
        "event_id" => event_id,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname)| {
        User::from_info(user_id, user_key, firstname, lastname, nickname)
    };

    let users = conn.exec_map(&stmt, &params, &map)?;
    Ok(users)
}

pub fn event_owner_list(event_id: u64) -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname, u.nickname
        FROM event_owners
        JOIN users u ON u.user_id = event_owners.user_id
        WHERE event_owners.event_id = :event_id",
    )?;
    let params = params! {
        "event_id" => event_id,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname)| {
        User::from_info(user_id, user_key, firstname, lastname, nickname)
    };

    let users = conn.exec_map(&stmt, &params, &map)?;
    Ok(users)
}

pub fn event_owner_add(event_id: u64, user_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();

    let stmt = conn.prep(
        "INSERT INTO event_owners (event_id, user_id)
        VALUES (:event_id, :user_id)",
    )?;
    let params = params! {
        "event_id" => &event_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn event_owner_remove(event_id: u64, user_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE FROM event_owners
        WHERE event_id = :event_id AND user_id = :user_id",
    )?;

    let params = params! {
        "event_id" => &event_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn event_owner_true(event_id: u64, user_id: u64) -> Result<bool, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT COUNT(1)
        FROM event_owners
        WHERE event_id = :event_id AND user_id = :user_id",
    )?;
    let params = params! {
        "event_id" => event_id,
        "user_id" => user_id,
    };

    match conn.exec_first::<u64, _, _>(&stmt, &params)? {
        None => Ok(false),
        Some(count) => Ok(count == 1),
    }
}

/* OWNER INVITES */

pub fn event_owner_invite_list(event_id: u64) -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname, u.nickname
        FROM event_owner_invites
        JOIN users u ON u.user_id = event_owner_invites.user_id
        WHERE event_id = :event_id;",
    )?;
    let params = params! {
        "event_id" => event_id,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname)| {
        User::from_info(user_id, user_key, firstname, lastname, nickname)
    };

    let users = conn.exec_map(&stmt, &params, &map)?;
    Ok(users)
}

pub fn event_owner_invite_add(event_id: u64, user_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO event_owner_invites (event_id, user_id)
        VALUES (:event_id, :user_id);",
    )?;
    let params = params! {
        "event_id" => &event_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn event_owner_invite_remove(event_id: u64, user_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE FROM event_owner_invites
        WHERE event_id = :event_id AND user_id = :user_id;",
    )?;

    let params = params! {
        "event_id" => &event_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

/* OWNER UNINVITES */

pub fn event_owner_uninvite_list(event_id: u64) -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname, u.nickname
        FROM event_owner_uninvites
        JOIN users u ON u.user_id = event_owner_uninvites.user_id
        WHERE event_id = :event_id;",
    )?;
    let params = params! {
        "event_id" => event_id,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname)| {
        User::from_info(user_id, user_key, firstname, lastname, nickname)
    };

    let users = conn.exec_map(&stmt, &params, &map)?;
    Ok(users)
}

pub fn event_owner_uninvite_add(event_id: u64, user_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO event_owner_uninvites (event_id, user_id)
        VALUES (:event_id, :user_id);",
    )?;
    let params = params! {
        "event_id" => &event_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn event_owner_uninvite_remove(event_id: u64, user_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE FROM event_owner_uninvites
        WHERE event_id = :event_id AND user_id = :user_id;",
    )?;

    let params = params! {
        "event_id" => &event_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

/* PARTICIPANT RELATED */

pub fn event_participant_pool(event_id: u64) -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    // TODO UNION event invites, team invites
    // TODO level check threshold if existent

    let stmt = conn.prep(
        "SELECT users.user_id, users.user_key, users.firstname, users.lastname, users.nickname
        FROM course_participant_teams AS cpt
        JOIN teams ON teams.team_id = cpt.team_id
        JOIN team_members tm ON teams.team_id = tm.team_id
        JOIN users ON tm.user_id = users.user_id
        JOIN events ON events.course_id = cpt.course_id
        WHERE events.event_id = :event_id AND users.active = TRUE
        GROUP BY users.user_id",
    )?;

    let params = params! {
        "event_id" => event_id,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname)| {
        User::from_info(user_id, user_key, firstname, lastname, nickname)
    };

    let users = conn.exec_map(&stmt, &params, &map)?;
    Ok(users)
}

pub fn event_participant_list(event_id: u64) -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname, u.nickname
        FROM event_participants p
        JOIN users u ON u.user_id = p.user_id
        WHERE event_id = :event_id;",
    )?;
    let params = params! {
        "event_id" => event_id,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname)| {
        User::from_info(user_id, user_key, firstname, lastname, nickname)
    };

    let users = conn.exec_map(&stmt, &params, &map)?;
    Ok(users)
}

pub fn event_participant_add(event_id: u64, user_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO event_participants (event_id, user_id)
        VALUES (:event_id, :user_id);",
    )?;
    let params = params! {
        "event_id" => &event_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn event_participant_remove(event_id: u64, user_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE FROM event_participants
        WHERE event_id = :event_id AND user_id = :user_id;",
    )?;

    let params = params! {
        "event_id" => &event_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

/* PARTICIPANT INVITE */

pub fn event_participant_invite_list(event_id: u64) -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname, u.nickname
        FROM event_participant_invites
        JOIN users u ON u.user_id = event_participant_invites.user_id
        WHERE event_id = :event_id;",
    )?;
    let params = params! {
        "event_id" => event_id,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname)| {
        User::from_info(user_id, user_key, firstname, lastname, nickname)
    };

    let users = conn.exec_map(&stmt, &params, &map)?;
    Ok(users)
}

pub fn event_participant_invite_add(event_id: u64, user_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO event_participant_invites (event_id, user_id)
        VALUES (:event_id, :user_id);",
    )?;
    let params = params! {
        "event_id" => &event_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn event_participant_invite_remove(event_id: u64, user_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE FROM event_participant_invites
        WHERE event_id = :event_id AND user_id = :user_id;",
    )?;

    let params = params! {
        "event_id" => &event_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

/* PARTICIPANT UNINVITE */

pub fn event_participant_uninvite_list(event_id: u64) -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname, u.nickname
        FROM event_participant_uninvites
        JOIN users u ON u.user_id = event_participant_uninvites.user_id
        WHERE event_id = :event_id;",
    )?;
    let params = params! {
        "event_id" => event_id,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname)| {
        User::from_info(user_id, user_key, firstname, lastname, nickname)
    };

    let users = conn.exec_map(&stmt, &params, &map)?;
    Ok(users)
}

pub fn event_participant_uninvite_add(event_id: u64, user_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO event_participant_uninvites (event_id, user_id)
        VALUES (:event_id, :user_id);",
    )?;
    let params = params! {
        "event_id" => &event_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn event_participant_uninvite_remove(event_id: u64, user_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE FROM event_participant_uninvites
        WHERE event_id = :event_id AND user_id = :user_id;",
    )?;

    let params = params! {
        "event_id" => &event_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

/* COURSE RELATED */

pub fn event_course_edit(event_id: u64, course_id: Option<u64>) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();

    let stmt = conn.prep(
        "UPDATE events
        SET course_id = :course_id
        WHERE event_id = :event_id",
    )?;

    let params = params! {
        "event_id" => &event_id,
        "course_id" => &course_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn event_course_true(event_id: u64, course_id: u64) -> Result<bool, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT COUNT(1)
        FROM events
        WHERE event_id = :event_id AND course_id = :course_id",
    )?;
    let params = params! {
        "event_id" => event_id,
        "course_id" => course_id,
    };

    match conn.exec_first::<u64, _, _>(&stmt, &params)? {
        None => Ok(false),
        Some(count) => Ok(count == 1),
    }
}

pub fn event_course_any(event_id: u64) -> Result<bool, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT COUNT(1)
        FROM events
        WHERE event_id = :event_id AND course_id IS NOT NULL;",
    )?;
    let params = params! {
        "event_id" => event_id,
    };

    match conn.exec_first::<u64, _, _>(&stmt, &params)? {
        None => Ok(false),
        Some(count) => Ok(count == 1),
    }
}

/* MODERATOR RELATED */

pub fn event_moderator_true(event_id: u64, user_id: u64) -> Result<bool, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT COUNT(1)
        FROM events e
        LEFT JOIN course_moderators m ON m.course_id = e.course_id
        WHERE e.event_id = :event_id AND m.user_id = :user_id;",
    )?;

    let params = params! {
        "event_id" => event_id,
        "user_id" => user_id,
    };

    match conn.exec_first::<u32, _, _>(&stmt, &params)? {
        None => Ok(false),
        Some(count) => Ok(count == 1),
    }
}

use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Acceptance, Event, Location, Occurrence, User};
use crate::db::get_pool_conn;
use crate::error::Error;

pub mod leader;
pub mod owner;
pub mod participant;
pub mod supporter;

/*
 * METHODS
 */

pub fn event_info(event_id: u64) -> Result<Event, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT event_id, event_key, e.title,
            l.location_id, l.location_key, l.name AS location_name, l.description AS location_description,
            e.begin, e.end, e.occurrence, e.acceptance, e.public, e.scrutable, e.note, e.course_id
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
        location: Some(Location {
            id: row.take("location_id").unwrap(),
            key: row.take("location_key").unwrap(),
            name: row.take("location_name").unwrap(),
            description: row.take("location_description").unwrap(),
        }),
        occurrence: row.take("occurrence").unwrap(),
        acceptance: row.take("acceptance").unwrap(),
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
    location_id: Option<u64>,
    occurrence: Option<Occurrence>,
    acceptance: Option<Acceptance>,
    course_true: Option<bool>,
    course_id: Option<u64>,
    owner_id: Option<u64>,
) -> Result<Vec<Event>, Error> {
    // If there is a search window, make sure it is somewhat correct
    if let (Some(begin), Some(end)) = (begin, end) {
        let delta = end.signed_duration_since(begin);

        if delta < crate::config::EVENT_SEARCH_WINDOW_MIN() || delta > crate::config::EVENT_SEARCH_WINDOW_MAX() {
            return Err(Error::EventSearchLimit);
        }

        if begin < crate::config::EVENT_SEARCH_DATE_MIN() || end > crate::config::EVENT_SEARCH_DATE_MAX() {
            return Err(Error::EventSearchLimit);
        }
    }

    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT e.event_id, e.event_key, e.title,
            l.location_id, l.location_key, l.name AS location_name, l.description AS location_description,
            e.begin, e.end, e.occurrence, e.acceptance, e.public, e.scrutable, e.note
        FROM events e
        JOIN locations l ON l.location_id = e.location_id
        LEFT JOIN event_owners o ON e.event_id = o.event_id
        WHERE (:begin IS NULL OR end > :begin)
        AND (:end IS NULL OR begin < :end)
        AND (:location_id IS NULL OR :location_id = l.location_id)
        AND (:occurrence IS NULL OR :occurrence = e.occurrence)
        AND (:acceptance IS NULL OR :acceptance = e.acceptance)
        AND (:course_true IS NULL OR (:course_true = TRUE AND :course_id = e.course_id) OR (:course_true = FALSE AND e.course_id IS NULL))
        AND (:owner_id IS NULL OR :owner_id = o.user_id)
        GROUP BY e.event_id;")?;

    let params = params! {
        "begin" => &begin,
        "end" => &end,
        "location_id" => &location_id,
        "occurrence" => &occurrence,
        "acceptance" => &acceptance,
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
            location: Some(Location {
                id: row.take("location_id").unwrap(),
                key: row.take("location_key").unwrap(),
                name: row.take("location_name").unwrap(),
                description: row.take("location_description").unwrap(),
            }),
            occurrence: row.take("occurrence").unwrap(),
            acceptance: row.take("acceptance").unwrap(),
            public: row.take("public").unwrap(),
            scrutable: row.take("scrutable").unwrap(),
            note: row.take("note").unwrap(),
            course_id: None,
        };
        events.push(item);
    }

    Ok(events)
}

pub fn event_create(event: &Event, acceptance: &Acceptance, course_id: Option<u64>) -> Result<u64, Error> {
    if event.key.len() < 3 || event.key.len() > 12 {
        return Err(Error::EventKeyInvalid);
    }

    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO events (event_key, pwd, title, begin, end, location_id, occurrence, acceptance, public, scrutable, note, course_id)
        SELECT :event_key, :pwd, :title, :begin, :end, :location_id, :occurrence, :acceptance, :public, :scrutable, :note, :course_id",
    )?;

    let params = params! {
        "event_key" => &event.key,
        "pwd" => crate::common::random_string(8),
        "title" => &event.title,
        "begin" => &event.begin,
        "end" => &event.end,
        "location_id" => &event.location.as_ref().map(|location| location.id),
        "occurrence" => &event.occurrence,
        "acceptance" => &acceptance,
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
        "location_id" => &event.location.as_ref().map(|location| location.id),
        "begin" => &event.begin,
        "end" => &event.end,
        "public" => &event.public,
        "scrutable" => &event.scrutable,
        "note" => &event.note,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn event_acceptance_edit(event_id: u64, acceptance: &Acceptance) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE events SET
        acceptance = :acceptance
        WHERE event_id = :event_id;",
    )?;
    let params = params! {
        "event_id" => event_id,
        "acceptance" => acceptance,
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
        WHERE NOT (end <= :begin OR begin >= :end)
        AND location_id = :location_id
        AND occurrence = 'OCCURRING'
        AND acceptance = 'ACCEPTED'",
    )?;
    let params = params! {
        "begin" => &event.begin,
        "end" => &event.end,
        "location_id" => &event.location.as_ref().map(|location| location.id),
    };

    let count = conn.exec_first::<u64, _, _>(&stmt, &params)?;
    match count {
        None => Err(Error::DatabaseError),
        Some(count) => Ok(count == 0),
    }
}

/* COURSE RELATED */

pub fn event_course_info(event_id: u64) -> Result<Option<u32>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT course_id
        FROM events
        WHERE event_id = :event_id",
    )?;
    let params = params! {
        "event_id" => event_id,
    };

    match conn.exec_first::<Option<u32>, _, _>(&stmt, &params)? {
        None => Err(Error::EventMissing),
        Some(course_id) => Ok(course_id),
    }
}

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
        Some(0) => Ok(false),
        Some(1) => Ok(true),
        _ => Err(Error::DatabaseError),
    }
}

/* BOOKMARKS */

pub fn event_bookmark_true(event_id: u64, user_id: u64) -> Result<bool, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT COUNT(1)
        FROM event_bookmarks b
        WHERE b.event_id = :event_id AND b.user_id = :user_id;",
    )?;

    let params = params! {
        "event_id" => event_id,
        "user_id" => user_id,
    };

    match conn.exec_first::<u32, _, _>(&stmt, &params)? {
        Some(0) => Ok(false),
        Some(1) => Ok(true),
        _ => Err(Error::DatabaseError),
    }
}

pub fn event_bookmark_add(event_id: u64, user_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO event_bookmarks (event_id, user_id)
        VALUES (:event_id, :user_id);",
    )?;
    let params = params! {
        "event_id" => &event_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn event_bookmark_remove(event_id: u64, user_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE FROM event_bookmarks
        WHERE event_id = :event_id AND user_id = :user_id;",
    )?;

    let params = params! {
        "event_id" => &event_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

/* STATISTICS */

pub fn event_statistic_packlist(
    event_id: u64,
    category1: Option<u32>,
    category2: Option<u32>,
    category3: Option<u32>,
) -> Result<Vec<(User, u32, u32, u32)>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname, u.nickname,
            COUNT(CASE WHEN i.category_id = :category1 THEN 1 END) AS count1,
            COUNT(CASE WHEN i.category_id = :category2 THEN 1 END) AS count2,
            COUNT(CASE WHEN i.category_id = :category3 THEN 1 END) AS count3
        FROM event_participant_presences ep
        JOIN users u ON ep.user_id = u.user_id
        LEFT JOIN user_possessions up ON up.user_id = ep.user_id
        LEFT JOIN items i ON up.item_id = i.item_id
        WHERE ep.event_id = :event_id
        GROUP BY u.user_id;",
    )?;
    let params = params! {
        "event_id" => event_id,
        "category1" => category1,
        "category2" => category2,
        "category3" => category3,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname, count1, count2, count3)| {
        (
            { User::from_info(user_id, user_key, firstname, lastname, nickname) },
            count1,
            count2,
            count3,
        )
    };

    let stats = conn.exec_map(&stmt, &params, &map)?;
    Ok(stats)
}

pub fn event_statistic_division(event_id: u64) -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname, u.nickname, u.federationnumber, u.birthday, u.gender
        FROM event_participant_presences ep
        JOIN users u ON ep.user_id = u.user_id
        WHERE ep.event_id = :event_id;",
    )?;
    let params = params! {
        "event_id" => event_id,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname, federationnumber, birthday, gender)| {
        let mut user = User::from_info(user_id, user_key, firstname, lastname, nickname);
        user.birthday = birthday;
        user.federationnumber = federationnumber;
        user.gender = gender;
        user
    };

    let stats = conn.exec_map(&stmt, &params, &map)?;
    Ok(stats)
}

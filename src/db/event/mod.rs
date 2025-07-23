use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Acceptance, Affiliation, Course, Event, Location, Occurrence, User};
use crate::error::ErrorKind;

pub mod attendance;
pub mod moderator;
pub mod owner;

/*
 * METHODS
 */

pub fn event_info(conn: &mut PooledConn, event_id: u64) -> Result<Event, ErrorKind> {
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

    let mut row: mysql::Row = conn.exec_first(&stmt, &params)?.ok_or(ErrorKind::EventMissing)?;

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
    conn: &mut PooledConn,
    begin: Option<chrono::NaiveDateTime>,
    end: Option<chrono::NaiveDateTime>,
    location_id: Option<u64>,
    occurrence: Option<Occurrence>,
    acceptance: Option<Acceptance>,
    course_true: Option<bool>,
    course_id: Option<u32>,
    owner_id: Option<u64>,
) -> Result<Vec<Event>, ErrorKind> {
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

pub fn event_create(
    conn: &mut PooledConn,
    event: &Event,
    acceptance: &Acceptance,
    course_id: Option<u32>,
) -> Result<u64, ErrorKind> {
    if event.key.len() < 3 || event.key.len() > 12 {
        return Err(ErrorKind::EventKeyInvalid);
    }

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

    Ok(conn.last_insert_id())
}

pub fn event_edit(conn: &mut PooledConn, event_id: u64, event: &Event) -> Result<(), ErrorKind> {
    if event.key.is_empty() {
        return Err(ErrorKind::EventKeyInvalid);
    }

    let stmt = conn.prep(
        "UPDATE events
        SET
            event_key = :event_key,
            title = :title,
            begin = :begin,
            end = :end,
            location_id = :location_id,
            occurrence = :occurrence,
            public = :public,
            scrutable = :scrutable,
            note = :note
        WHERE event_id = :event_id",
    )?;

    let params = params! {
        "event_id" => &event_id,
        "event_key" => &event.key,
        "title" => &event.title,
        "begin" => &event.begin,
        "end" => &event.end,
        "location_id" => &event.location.as_ref().map(|location| location.id),
        "occurrence" => &event.occurrence,
        "public" => &event.public,
        "scrutable" => &event.scrutable,
        "note" => &event.note,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn event_acceptance_edit(conn: &mut PooledConn, event_id: u64, acceptance: &Acceptance) -> Result<(), ErrorKind> {
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

pub fn event_password_edit(conn: &mut PooledConn, event_id: u64, password: String) -> Result<(), ErrorKind> {
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

pub fn event_note_edit(conn: &mut PooledConn, event_id: u64, note: &str) -> Result<(), ErrorKind> {
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

pub fn event_delete(conn: &mut PooledConn, event_id: u64) -> Result<(), ErrorKind> {
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

pub fn event_free_true(conn: &mut PooledConn, event: &Event) -> Result<bool, ErrorKind> {
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
        None => Err(ErrorKind::DatabaseError),
        Some(count) => Ok(count == 0),
    }
}

/* COURSE RELATED */

pub fn event_course_info(conn: &mut PooledConn, event_id: u64) -> Result<Option<Course>, ErrorKind> {
    let stmt = conn.prep(
        "SELECT c.course_id, c.course_key, c.title as course_title, c.active as course_active, c.public as course_public
        FROM events e
        JOIN courses c ON c.course_id = e.course_id
        WHERE e.event_id = :event_id",
    )?;
    let params = params! {
        "event_id" => event_id,
    };

    let row = conn.exec_first(&stmt, &params)?;
    Ok(row.and_then(|mut row| Course::from_row(&mut row)))
}

pub fn event_course_edit(conn: &mut PooledConn, event_id: u64, course_id: Option<u32>) -> Result<(), ErrorKind> {
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

pub fn event_moderator_true(conn: &mut PooledConn, event_id: u64, user_id: u64) -> Result<bool, ErrorKind> {
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
        _ => Err(ErrorKind::DatabaseError),
    }
}

/* BOOKMARKS */

pub fn event_bookmark_true(conn: &mut PooledConn, event_id: u64, user_id: u64) -> Result<bool, ErrorKind> {
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
        _ => Err(ErrorKind::DatabaseError),
    }
}

pub fn event_bookmark_add(conn: &mut PooledConn, event_id: u64, user_id: u64) -> Result<(), ErrorKind> {
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

pub fn event_bookmark_remove(conn: &mut PooledConn, event_id: u64, user_id: u64) -> Result<(), ErrorKind> {
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
    conn: &mut PooledConn,
    event_id: u64,
    category1: Option<u32>,
    category2: Option<u32>,
    category3: Option<u32>,
) -> Result<Vec<(User, u32, u32, u32)>, ErrorKind> {
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

pub fn event_statistic_organisation(
    conn: &mut PooledConn,
    event_id: u64,
    organisation_id: u64,
) -> Result<Vec<Affiliation>, ErrorKind> {
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname AS user_firstname, u.lastname AS user_lastname, u.nickname AS user_nickname,
            u.birth_date AS user_birth_date, u.gender AS user_gender, u.height AS user_height, u.weight AS user_weight,
            o.organisation_id, o.abbreviation AS organisation_abbreviation, o.name AS organisation_name,
            oa.member_identifier, oa.permission_solo_date, oa.permission_team_date, oa.residency_move_date
        FROM event_participant_presences ep
        JOIN users u ON ep.user_id = u.user_id
        LEFT JOIN organisation_affiliations oa ON oa.user_id = u.user_id AND oa.organisation_id = :organisation_id
        LEFT JOIN organisations o ON o.organisation_id = oa.organisation_id
        WHERE ep.event_id = :event_id;",
    )?;
    let params = params! {
        "event_id" => event_id,
        "organisation_id" => organisation_id,
    };

    let rows: Vec<mysql::Row> = conn.exec(&stmt, &params)?;

    let mut affiliations: Vec<Affiliation> = Vec::new();

    for mut row in rows {
        affiliations.push(Affiliation::from_row(&mut row));
    }

    Ok(affiliations)
}

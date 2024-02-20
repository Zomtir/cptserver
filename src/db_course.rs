use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Course, Team, User};
use crate::db::get_pool_conn;
use crate::error::Error;

/*
 * METHODS
 */

pub fn course_list(mod_id: Option<i64>, active: Option<bool>, public: Option<bool>) -> Result<Vec<Course>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT c.course_id, c.course_key, c.title, c.active, c.public
        FROM courses c
        LEFT JOIN course_moderators m ON c.course_id = m.course_id
        WHERE (:mod_id IS NULL OR m.user_id = :mod_id)
        AND (:active IS NULL OR c.active = :active)
        AND (:public IS NULL OR c.public = :public)
        GROUP BY c.course_id",
    )?;

    let params = params! {
        "mod_id" => mod_id,
        "active" => active,
        "public" => public,
    };

    let map =
        |(course_id, course_key, course_title, active, public)| {
            Course {
                id: course_id,
                key: course_key,
                title: course_title,
                active,
                public,
            }
        };

    match conn.exec_map(&stmt, &params, &map)? {
        courses => Ok(courses),
    }
}

pub fn course_available(user_id: i64) -> Result<Vec<Course>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT DISTINCT c.course_id, c.course_key, c.title, c.active, c.public
        FROM courses c
        INNER JOIN course_requirements cr ON c.course_id = cr.course_id
        LEFT JOIN user_rankings ur ON ur.branch_id = cr.branch_id AND ur.rank >= cr.rank AND ur.user_id = :user_id
        WHERE ur.user_id IS NOT NULL;",
    )?;

    let params = params! {
        "user_id" => user_id,
    };

    let map =
        |(course_id, course_key, course_title, active, public)| {
            Course {
                id: course_id,
                key: course_key,
                title: course_title,
                active,
                public,
            }
        };

    match conn.exec_map(&stmt, &params, &map)? {
        courses => Ok(courses),
    }
}

pub fn course_create(course: &Course) -> Result<u32, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO courses (course_key, title, active, public)
        VALUES (:course_key, :title, :active, :public)",
    )?;
    let params = params! {
        "course_key" => crate::common::random_string(10),
        "title" => &course.title,
        "active" => &course.active,
        "public" => &course.public,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(conn.last_insert_id() as u32)
}

pub fn course_edit(course_id: i64, course: &Course) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE courses SET
            course_key = :course_key,
            title = :title,
            active = :active,
            public = :public
            WHERE course_id = :course_id")?;

    let params = params! {
        "course_id" => &course_id,
        "course_key" => &course.key,
        "title" => &course.title,
        "active" => &course.active,
        "public" => &course.public,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(())
}

pub fn course_delete(course_id: i64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE c FROM courses c
        WHERE c.course_id = :course_id")?;

    let params = params! {
        "course_id" => &course_id,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(())
}

pub fn course_moderator_list(course_id: i64) -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname
        FROM users u
        JOIN course_moderators m ON m.user_id = u.user_id
        WHERE m.course_id = :course_id",
    )?;

    let params = params! {
        "course_id" => course_id,
    };
    let map = |(user_id, user_key, firstname, lastname)| User::from_info(user_id, user_key, firstname, lastname);

    match conn.exec_map(&stmt, &params, &map)? {
        members => Ok(members),
    }
}

pub fn course_moderator_true(course_id: i64, user_id: i64) -> Result<bool, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT COUNT(1)
        FROM course_moderators
        WHERE course_id = :course_id AND user_id = :user_id",
    )?;

    let params = params! {
        "course_id" => course_id,
        "user_id" => user_id,
    };

    match conn.exec_first::<u32, _, _>(&stmt, &params)? {
        None => Ok(false),
        Some(count) => Ok(count == 1),
    }
}

pub fn course_moderator_add(course_id: i64, user_id: i64) -> Option<()> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO course_moderators (course_id, user_id)
                          SELECT :course_id, :user_id",
    );
    let params = params! {
        "course_id" => &course_id,
        "user_id" => &user_id,
    };

    match conn.exec_drop(&stmt.unwrap(), &params) {
        Err(..) => None,
        Ok(..) => Some(()),
    }
}

pub fn course_moderator_remove(course_id: i64, user_id: i64) -> Option<()> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn
        .prep(
            "DELETE e FROM course_moderators e
            WHERE course_id = :course_id AND user_id = :user_id",
        )
        .unwrap();
    let params = params! {
        "course_id" => &course_id,
        "user_id" => &user_id,
    };

    match conn.exec_drop(&stmt, &params) {
        Err(..) => None,
        Ok(..) => Some(()),
    }
}

pub fn course_teaminvite_list(course_id: i64) -> Result<Vec<Team>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT t.team_id, t.name, t.description
        FROM course_teaminvites c
        LEFT JOIN teams t ON c.team_id = t.team_id
        WHERE course_id = :course_id;",
    )?;
    let params = params! {
        "course_id" => course_id,
    };
    let map = |(team_id, name, description)| Team {
        id: team_id,
        name,
        description,
        right: None,
    };

    let teams = conn.exec_map(&stmt, &params, &map)?;
    Ok(teams)
}

pub fn course_teaminvite_add(course_id: i64, team_id: i64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO course_teaminvites (course_id, team_id)
        VALUES (:course_id, :team_id);",
    )?;
    let params = params! {
        "course_id" => &course_id,
        "team_id" => &team_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn course_teaminvite_remove(course_id: i64, team_id: i64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE FROM course_teaminvites
        WHERE course_id = :course_id AND team_id = :team_id;",
    )?;

    let params = params! {
        "course_id" => &course_id,
        "team_id" => &team_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

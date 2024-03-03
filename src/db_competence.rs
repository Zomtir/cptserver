use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Competence, Skill, User};
use crate::db::get_pool_conn;
use crate::error::Error;

pub fn competence_list(
    user_id: Option<i64>,
    skill_id: Option<i64>,
    rank_min: i16,
    rank_max: i16,
) -> Result<Vec<Competence>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT uc.skill_id,
            u.user_id, u.user_key, u.firstname, u.lastname,
            b.skill_id, b.branch_key, b.title,
            uc.rank, uc.date,
            j.user_id, j.user_key, j.firstname, j.lastname
        FROM user_competences uc
        JOIN branches b ON (uc.skill_id = b.skill_id)
        JOIN users u ON (uc.user_id = u.user_id)
        JOIN users j ON (uc.judge_id = j.user_id)
        WHERE (:user_id IS NULL OR uc.user_id = :user_id)
        AND ((:skill_id IS NULL) OR (uc.skill_id = :skill_id AND uc.rank >= :rank_min AND uc.rank <= :rank_max))",
    )?;

    let params = params! {
        "user_id" => user_id,
        "skill_id" => skill_id,
        "rank_min" => rank_min,
        "rank_max" => rank_max,
    };

    let rows: Vec<mysql::Row> = conn.exec(&stmt, &params)?;

    let mut competences: Vec<Competence> = Vec::new();

    for mut row in rows {
        let uc = Competence {
            id: row.take("skill_id").unwrap(),
            user: User::from_info(
                row.take(1).unwrap(),
                row.take(2).unwrap(),
                row.take(3).unwrap(),
                row.take(4).unwrap(),
            ),
            skill: Skill {
                id: row.take(5).unwrap(),
                key: row.take(6).unwrap(),
                title: row.take(7).unwrap(),
            },
            rank: row.take("rank").unwrap(),
            date: row.take("date").unwrap(),
            judge: User::from_info(
                row.take(10).unwrap(),
                row.take(11).unwrap(),
                row.take(12).unwrap(),
                row.take(13).unwrap(),
            ),
        };
        competences.push(uc);
    }

    Ok(competences)
}

pub fn competence_create(competence: &Competence) -> Result<u32, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO user_competences (user_id, skill_id, `rank`, date, judge_id)
        SELECT :user_id, :skill_id, :rank, :date, :judge_id",
    );
    let params = params! {
        "user_id" => &competence.user.id,
        "skill_id" => &competence.skill.id,
        "rank" => &competence.rank,
        "date" => &competence.date,
        "judge_id" => &competence.judge.id,
    };

    conn.exec_drop(&stmt.unwrap(), &params)?;

    Ok(conn.last_insert_id() as u32)
}

pub fn competence_edit(skill_id: i64, competence: &Competence) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE user_competences
        SET
            user_id  = :user_id,
            skill_id = :skill_id,
            `rank` = :rank,
            date = :date,
            judge_id = :judge_id
        WHERE skill_id = :skill_id",
    );

    let params = params! {
        "skill_id" => &skill_id,
        "user_id" => &competence.user.id,
        "skill_id" => &competence.skill.id,
        "rank" => &competence.rank,
        "date" => &competence.date,
        "judge_id" => &competence.judge.id,
    };

    match conn.exec_drop(&stmt.unwrap(), &params) {
        Err(..) => Err(Error::DatabaseError),
        Ok(..) => Ok(()),
    }
}

pub fn competence_delete(skill_id: i64) -> Option<()> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE uc FROM user_competences uc WHERE uc.skill_id = :skill_id");

    let params = params! {
        "skill_id" => skill_id
    };

    match conn.exec_drop(&stmt.unwrap(), &params) {
        Err(..) => None,
        Ok(..) => Some(()),
    }
}

pub fn competence_summary(user_id: i64) -> Result<Vec<(Skill, i16)>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT b.skill_id, b.branch_key, b.title, MAX(uc.rank)
        FROM user_competences uc
        JOIN branches b ON (uc.skill_id = b.skill_id)
        JOIN users j ON (uc.judge_id = j.user_id)
        WHERE uc.user_id = :user_id
        GROUP BY b.skill_id;",
    )?;

    let params = params! {
        "user_id" => user_id,
    };

    let map = |(skill_id, branch_key, branch_title, maxrank)| {
        (
            Skill {
                id: skill_id,
                key: branch_key,
                title: branch_title,
            },
            maxrank,
        )
    };

    Ok(conn.exec_map(&stmt, &params, &map)?)
}

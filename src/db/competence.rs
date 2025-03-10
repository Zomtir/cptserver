use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Competence, Skill, User};
use crate::error::Error;

pub fn competence_list(
    conn: &mut PooledConn,
    user_id: Option<u64>,
    skill_id: Option<u64>,
    rank_min: i16,
    rank_max: i16,
) -> Result<Vec<Competence>, Error> {
    let stmt = conn.prep(
        "SELECT uc.competence_id,
            u.user_id, u.user_key, u.firstname, u.lastname, u.nickname,
            s.skill_id, s.skill_key, s.title as skill_title, s.min as skill_min, s.max as skill_max,
            uc.rank, uc.date,
            j.user_id as judge_id, j.user_key as judge_key, j.firstname as judge_firstname, j.lastname as judge_lastname, j.nickname as judge_nickname
        FROM user_competences uc
        JOIN skills s ON (uc.skill_id = s.skill_id)
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
            id: row.take("competence_id").unwrap(),
            user: User::from_info(
                row.take("user_id").unwrap(),
                row.take("user_key").unwrap(),
                row.take("firstname").unwrap(),
                row.take("lastname").unwrap(),
                row.take("nickname").unwrap(),
            ),
            skill: Skill {
                id: row.take("skill_id").unwrap(),
                key: row.take("skill_key").unwrap(),
                title: row.take("skill_title").unwrap(),
                min: row.take("skill_min").unwrap(),
                max: row.take("skill_max").unwrap(),
            },
            rank: row.take("rank").unwrap(),
            date: row.take("date").unwrap(),
            judge: User::from_info(
                row.take("judge_id").unwrap(),
                row.take("judge_key").unwrap(),
                row.take("judge_firstname").unwrap(),
                row.take("judge_lastname").unwrap(),
                row.take("judge_nickname").unwrap(),
            ),
        };
        competences.push(uc);
    }

    Ok(competences)
}

pub fn competence_create(conn: &mut PooledConn, competence: &Competence) -> Result<u32, Error> {
    let stmt = conn.prep(
        "INSERT INTO user_competences (user_id, skill_id, `rank`, date, judge_id)
        SELECT :user_id, :skill_id, :rank, :date, :judge_id",
    )?;
    let params = params! {
        "user_id" => &competence.user.id,
        "skill_id" => &competence.skill.id,
        "rank" => &competence.rank,
        "date" => &competence.date,
        "judge_id" => &competence.judge.id,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(conn.last_insert_id() as u32)
}

pub fn competence_edit(conn: &mut PooledConn, competence_id: u64, competence: &Competence) -> Result<(), Error> {
    let stmt = conn.prep(
        "UPDATE user_competences
        SET
            user_id  = :user_id,
            skill_id = :skill_id,
            `rank` = :rank,
            date = :date,
            judge_id = :judge_id
        WHERE competence_id = :competence_id;",
    )?;

    let params = params! {
        "competence_id" => &competence_id,
        "user_id" => &competence.user.id,
        "skill_id" => &competence.skill.id,
        "rank" => &competence.rank,
        "date" => &competence.date,
        "judge_id" => &competence.judge.id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn competence_delete(conn: &mut PooledConn, competence_id: u64) -> Result<(), Error> {
    let stmt = conn.prep("DELETE uc FROM user_competences uc WHERE uc.competence_id = :competence_id")?;

    let params = params! {
        "competence_id" => competence_id
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn competence_summary(conn: &mut PooledConn, user_id: u64) -> Result<Vec<(Skill, i16)>, Error> {
    let stmt = conn.prep(
        "SELECT s.skill_id, s.skill_key, s.title, s.min, s.max, MAX(uc.rank)
        FROM user_competences uc
        JOIN skills s ON (uc.skill_id = s.skill_id)
        JOIN users j ON (uc.judge_id = j.user_id)
        WHERE uc.user_id = :user_id
        GROUP BY s.skill_id;",
    )?;

    let params = params! {
        "user_id" => user_id,
    };

    let map = |(skill_id, skill_key, skill_title, skill_min, skill_max, rank)| {
        (
            Skill {
                id: skill_id,
                key: skill_key,
                title: skill_title,
                min: skill_min,
                max: skill_max,
            },
            rank,
        )
    };

    Ok(conn.exec_map(&stmt, &params, &map)?)
}

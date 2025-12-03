use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Competence, Skill};
use crate::error::ErrorKind;

pub fn competence_list(
    conn: &mut PooledConn,
    user_id: Option<u64>,
    skill_id: Option<u64>,
    rank_min: i16,
    rank_max: i16,
) -> Result<Vec<Competence>, ErrorKind> {
    let stmt = conn.prep(
        "SELECT uc.competence_id,
            u.user_id, u.user_key, u.firstname as user_firstname, u.lastname as user_lastname, u.nickname as user_nickname,
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
        let uc = Competence::from_row(&mut row);
        competences.push(uc);
    }

    Ok(competences)
}

pub fn competence_info(conn: &mut PooledConn, competence_id: Option<u64>) -> Result<Option<Competence>, ErrorKind> {
    let stmt = conn.prep(
        "SELECT uc.competence_id,
            u.user_id, u.user_key, u.firstname as user_firstname, u.lastname as user_lastname, u.nickname as user_nickname,
            s.skill_id, s.skill_key, s.title as skill_title, s.min as skill_min, s.max as skill_max,
            uc.rank, uc.date,
            j.user_id as judge_id, j.user_key as judge_key, j.firstname as judge_firstname, j.lastname as judge_lastname, j.nickname as judge_nickname
        FROM user_competences uc
        JOIN skills s ON (uc.skill_id = s.skill_id)
        JOIN users u ON (uc.user_id = u.user_id)
        JOIN users j ON (uc.judge_id = j.user_id)
        WHERE uc.competence_id = :competence_id;",
    )?;

    let params = params! {
        "competence_id" => competence_id,
    };

    let row = conn.exec_first(&stmt, &params)?;
    Ok(row.map(|mut r| Competence::from_row(&mut r)))
}

pub fn competence_create(conn: &mut PooledConn, competence: &Competence) -> Result<u32, ErrorKind> {
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

pub fn competence_edit(conn: &mut PooledConn, competence_id: u64, competence: &Competence) -> Result<(), ErrorKind> {
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

pub fn competence_delete(conn: &mut PooledConn, competence_id: u64) -> Result<(), ErrorKind> {
    let stmt = conn.prep("DELETE uc FROM user_competences uc WHERE uc.competence_id = :competence_id")?;

    let params = params! {
        "competence_id" => competence_id
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn competence_summary(conn: &mut PooledConn, user_id: u64) -> Result<Vec<(Skill, i16)>, ErrorKind> {
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

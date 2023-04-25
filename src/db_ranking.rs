use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Branch, Ranking, User};
use crate::db::get_pool_conn;
use crate::error::Error;

pub fn list_rankings(
    user_id: Option<i64>,
    branch_id: Option<i64>,
    rank_min: i16,
    rank_max: i16,
) -> Option<Vec<Ranking>> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT r.ranking_id,
            u.user_id, u.user_key, u.firstname, u.lastname,
            b.branch_id, b.branch_key, b.title,
            r.rank, r.date,
            j.user_id, j.user_key, j.firstname, j.lastname
        FROM rankings r
        JOIN branches b ON (r.branch_id = b.branch_id)
        JOIN users u ON (r.user_id = u.user_id)
        JOIN users j ON (r.judge_id = j.user_id)
        WHERE (:user_id IS NULL OR r.user_id = :user_id)
        AND ((:branch_id IS NULL) OR (r.branch_id = :branch_id AND r.rank >= :rank_min AND r.rank <= :rank_max))",
    );

    let params = params! {
        "user_id" => user_id,
        "branch_id" => branch_id,
        "rank_min" => rank_min,
        "rank_max" => rank_max,
    };

    let rows: Vec<mysql::Row> = match conn.exec(&stmt.unwrap(), &params) {
        Err(..) => return None,
        Ok(rows) => rows,
    };

    let mut rankings: Vec<Ranking> = Vec::new();

    for mut row in rows {
        let r = Ranking {
            id: row.take("ranking_id").unwrap(),
            user: User::from_info(
                row.take(1).unwrap(),
                row.take(2).unwrap(),
                row.take(3).unwrap(),
                row.take(4).unwrap(),
            ),
            branch: Branch {
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
        rankings.push(r);
    }

    return Some(rankings);
}

pub fn create_ranking(ranking: &Ranking) -> Result<u32, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO rankings (user_id, branch_id, `rank`, date, judge_id)
        SELECT :user_id, :branch_id, :rank, :date, :judge_id",
    );
    let params = params! {
        "user_id" => &ranking.user.id,
        "branch_id" => &ranking.branch.id,
        "rank" => &ranking.rank,
        "date" => &ranking.date,
        "judge_id" => &ranking.judge.id,
    };

    conn.exec_drop(&stmt.unwrap(), &params)?;

    crate::db::get_last_id(&mut conn)
}

pub fn edit_ranking(ranking_id: i64, ranking: &Ranking) -> Option<()> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE rankings
        SET
            user_id  = :user_id,
            branch_id = :branch_id,
            `rank` = :rank,
            date = :date,
            judge_id = :judge_id
        WHERE ranking_id = :ranking_id",
    );

    let params = params! {
        "ranking_id" => &ranking_id,
        "user_id" => &ranking.user.id,
        "branch_id" => &ranking.branch.id,
        "rank" => &ranking.rank,
        "date" => &ranking.date,
        "judge_id" => &ranking.judge.id,
    };

    match conn.exec_drop(&stmt.unwrap(), &params) {
        Err(..) => None,
        Ok(..) => Some(()),
    }
}

pub fn delete_ranking(ranking_id: i64) -> Option<()> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE r FROM rankings r WHERE r.ranking_id = :ranking_id");

    let params = params! {
        "ranking_id" => ranking_id
    };

    match conn.exec_drop(&stmt.unwrap(), &params) {
        Err(..) => None,
        Ok(..) => Some(()),
    }
}

pub fn summarize_rankings(user_id: i64) -> Option<Vec<(Branch, i16)>> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT b.branch_id, b.branch_key, b.title, MAX(r.rank)
        FROM rankings r
        JOIN branches b ON (r.branch_id = b.branch_id)
        JOIN users j ON (r.judge_id = j.user_id)
        WHERE r.user_id = :user_id
        GROUP BY b.branch_id;",
    );

    let params = params! {
        "user_id" => user_id,
    };

    let map = |(branch_id, branch_key, branch_title, maxrank)| {
        (
            Branch {
                id: branch_id,
                key: branch_key,
                title: branch_title,
            },
            maxrank,
        )
    };

    match conn.exec_map(&stmt.unwrap(), &params, &map) {
        Err(..) => None,
        Ok(summary) => Some(summary),
    }
}

use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use rocket::http::Status;
use rocket::serde::json::Json;

use crate::db::get_pool_conn;
use crate::session::{UserSession};
use crate::common::{Ranking, Member, Branch};

#[rocket::get("/ranking_list?<user_id>&<branch_id>&<min>&<max>")]
pub fn ranking_list(user_id: u16, branch_id: u16, min: u8, max: u8, session: UserSession) -> Option<Json<Vec<Ranking>>> {
    if !session.user.admin_rankings {return None};

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT r.ranking_id,
                                 u.user_id, u.user_key, u.firstname, u.lastname,
                                 b.branch_id, b.branch_key, b.title,
                                 r.rank, r.date,
                                 j.user_id, j.user_key, j.firstname, j.lastname
                          FROM rankings r
                          JOIN branches b ON (r.branch_id = b.branch_id)
                          JOIN users u ON (r.user_id = u.user_id)
                          JOIN users j ON (r.judge_id = j.user_id)
                          WHERE ((:user_id = '0') OR (r.user_id = :user_id))
                          AND ((:branch_id = '0') OR (r.branch_id = :branch_id AND r.rank >= :rank_min AND r.rank <= :rank_max))").unwrap();

    let params = params! {
        "user_id" => user_id,
        "branch_id" => branch_id,
        "rank_min" => min,
        "rank_max" => max,
    };

    let rows : Vec<mysql::Row> = conn.exec(&stmt,&params).unwrap();

    let mut rankings : Vec<Ranking> = Vec::new();

    for mut row in rows {
        let r = Ranking {
            id : row.take("ranking_id").unwrap(),
            user: Member {
                id: row.take(1).unwrap(),
                key: row.take(2).unwrap(),
                firstname: row.take(3).unwrap(),
                lastname: row.take(4).unwrap()
            },
            branch : Branch {
                id: row.take(5).unwrap(),
                key: row.take(6).unwrap(),
                title: row.take(7).unwrap(),
            },
            rank: row.take("rank").unwrap(),
            date: row.take("date").unwrap(),
            judge : Member {
                id: row.take(10).unwrap(),
                key: row.take(11).unwrap(),
                firstname: row.take(12).unwrap(),
                lastname: row.take(13).unwrap(),
            },
        };
        rankings.push(r);
    }

    return Some(Json(rankings));
}

#[rocket::post("/ranking_create", format = "application/json", data = "<ranking>")]
pub fn ranking_create(ranking: Json<Ranking>, session: UserSession) {
    if !session.user.admin_rankings {return};

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("INSERT INTO rankings (user_id, branch_id, `rank`, date, judge_id)
                          SELECT :user_id, :branch_id, :rank, :date, :judge_id").unwrap();

    conn.exec::<String,_,_>(
        &stmt,
        params! {
            "user_id" => &ranking.user.id,
            "branch_id" => &ranking.branch.id,
            "rank" => &ranking.rank,
            "date" => &ranking.date,
            "judge_id" => &ranking.judge.id,
        },
    ).unwrap();
}

#[rocket::post("/ranking_edit", format = "application/json", data = "<ranking>")]
pub fn ranking_edit(ranking: Json<Ranking>, session: UserSession) {
    if !session.user.admin_rankings {return};

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("UPDATE rankings SET
                            user_id  = :user_id,
                            branch_id = :branch_id,
                            `rank` = :rank,
                            date = :date,
                            judge_id = :judge_id
                          WHERE rankin_id = :ranking_id").unwrap();

    conn.exec::<String,_,_>(
        &stmt,
        params! {
            "ranking_id" => &ranking.id,
            "user_id" => &ranking.user.id,
            "branch_id" => &ranking.branch.id,
            "rank" => &ranking.rank,
            "date" => &ranking.date,
            "judge_id" => &ranking.judge.id,
        },
    ).unwrap();
}

#[rocket::head("/ranking_delete?<ranking_id>", format = "application/json")]
pub fn ranking_delete(ranking_id: u32, session: UserSession) -> Status {
    if !session.user.admin_rankings {return Status::Forbidden};

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE r FROM rankings r WHERE r.ranking_id = :ranking_id").unwrap();

    let params = params! {"ranking_id" => ranking_id};

    match conn.exec::<String,_,_>(&stmt,&params){
        Err(..) => return Status::Conflict,
        Ok(..) => return Status::Ok,
    };
}

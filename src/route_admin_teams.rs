use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use rocket::http::Status;
use rocket::serde::json::Json;

use crate::db::get_pool_conn;
use crate::session::{UserSession};
use crate::common::{Team};

/* ROUTES */

#[rocket::get("/team_list")]
pub fn team_list(session: UserSession) -> Result<Json<Vec<Team>>, Status> {
    if !session.user.admin_users {return Err(Status::Unauthorized)};

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT team_id, name, description, admin_users, admin_rankings, admin_reservations, admin_courses FROM teams").unwrap();
    let map = |(team_id, name, description, admin_users, admin_rankings, admin_reservations, admin_courses)| {
        Team {id: team_id, name, description, admin_users, admin_rankings, admin_reservations, admin_courses}
    };

    match conn.exec_map(&stmt,params::Params::Empty,&map) {
        Err(..) => Err(Status::InternalServerError),
        Ok(teams) => Ok(Json(teams)),
    }
}

#[rocket::post("/team_create", format = "application/json", data = "<team>")]
pub fn team_create(session: UserSession, team: Json<Team>) -> Result<String, Status>{
    if !session.user.admin_users {return Err(Status::Unauthorized)};

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("INSERT INTO teams (name, description, admin_users, admin_rankings, admin_reservations, admin_courses)
                          VALUES (:name, :description, :admin_users, :admin_rankings, :admin_reservations, :admin_courses)").unwrap();
    let params = params! {
        "name" => &team.name,
        "description" => &team.description,
        "admin_users" => &team.admin_users,
        "admin_rankings" => &team.admin_rankings,
        "admin_reservations" => &team.admin_reservations,
        "admin_courses" => &team.admin_courses,
    };

    match conn.exec::<String,_,_>(&stmt,&params) {
        Err(..) => return Err(Status::BadRequest),
        Ok(..) => (),
    };

    let stmt_id = conn.prep("SELECT LAST_INSERT_ID()").unwrap();

    match conn.exec_first::<u32,_,_>(&stmt_id,params::Params::Empty) {
        Err(..) | Ok(None) => return Err(Status::Conflict),
        Ok(Some(team_id)) => return Ok(team_id.to_string()),
    };
}

#[rocket::post("/team_edit", format = "application/json", data = "<team>")]
pub fn team_edit(session: UserSession, team: Json<Team>) -> Status {
    if !session.user.admin_users {return Status::Unauthorized};

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("UPDATE users SET
        name = :name,
        description = :description,
        admin_users = :admin_users,
        admin_rankings = :admin_rankings,
        admin_reservations = :admin_reservations,
        admin_courses = :admin_courses
        WHERE team_id = :team_id").unwrap();
    let params = params! {
        "team_id" => &team.id,
        "name" => &team.name,
        "description" => &team.description,
        "admin_users" => &team.admin_users,
        "admin_rankings" => &team.admin_rankings,
        "admin_reservations" => &team.admin_reservations,
        "admin_courses" => &team.admin_courses,
    };

    match conn.exec::<String,_,_>(&stmt,&params) {
        Err(..) => Status::InternalServerError,
        Ok(..) => Status::Ok
    }
}

#[rocket::head("/team_delete?<team_id>")]
pub fn team_delete(session: UserSession, team_id: u32) -> Status {
    if !session.user.admin_users {return Status::Unauthorized};

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE t FROM teams t WHERE t.team_id = :team_id").unwrap();
    let params = params! {"team_id" => team_id};

    match conn.exec::<String,_,_>(&stmt,&params) {
        Err(..) => Status::InternalServerError,
        Ok(..) => Status::Ok,
    }
}

#[rocket::head("/team_enrol?<team_id>&<user_id>")]
pub fn team_enrol(session: UserSession, team_id: u32, user_id: u32) -> Status {
    if !session.user.admin_users {return Status::Unauthorized};

    // TODO fix DB call to drop/extend permissions
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("INSERT INTO team_members (team_id, user_id)
                          SELECT :team_id, :user_id").unwrap();
    let params = params! {
        "team_id" => &team_id,
        "user_id" => &user_id,
    };

    match conn.exec::<String,_,_>(&stmt,&params) {
        Err(..) => Status::InternalServerError,
        Ok(..) => Status::Ok,
    }
}

#[rocket::head("/team_dismiss?<team_id>&<user_id>")]
pub fn team_dismiss(session: UserSession, team_id: u32, user_id: u32) -> Status {
    if !session.user.admin_users {return Status::Unauthorized};

    // TODO fix DB call to drop/extend permissions
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE FROM team_members
                          WHERE team_id = :team_id AND e.user_id = :user_id").unwrap();
    let params = params! {
        "team_id" => &team_id,
        "user_id" => &user_id,
    };

    match conn.exec::<String,_,_>(&stmt,&params) {
        Err(..) => Status::InternalServerError,
        Ok(..) => Status::Ok,
    }
}
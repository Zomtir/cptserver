use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use rocket::http::Status;
use rocket::serde::json::Json;

use crate::db::get_pool_conn;
use crate::session::{UserSession};
use crate::common::{Team, Right};

/* ROUTES */

#[rocket::get("/admin/team_list")]
pub fn team_list(session: UserSession) -> Result<Json<Vec<Team>>, Status> {
    if !session.right.admin_teams {return Err(Status::Unauthorized)};

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("
    SELECT team_id, name, description, admin_courses, admin_event, admin_inventory, admin_rankings, admin_teams, admin_users
    FROM teams").unwrap();
    let map =
    |(team_id, name, description, admin_courses, admin_event, admin_inventory, admin_rankings, admin_teams, admin_users)| {
        Team {id: team_id, name, description, 
            right: Right {
                admin_courses,
                admin_event,
                admin_inventory,
                admin_rankings,
                admin_teams,
                admin_users,
            }
        }
    };

    match conn.exec_map(&stmt,params::Params::Empty,&map) {
        Err(..) => Err(Status::InternalServerError),
        Ok(teams) => Ok(Json(teams)),
    }
}

#[rocket::post("/admin/team_create", format = "application/json", data = "<team>")]
pub fn team_create(session: UserSession, team: Json<Team>) -> Result<String, Status>{
    if !session.right.admin_teams {return Err(Status::Unauthorized)};

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("
        INSERT INTO teams (name, description, admin_courses, admin_event, admin_inventory, admin_rankings, admin_teams, admin_users)
        VALUES (:name, :description, :admin_courses, :admin_event, :admin_inventory, :admin_rankings, :admin_teams, :admin_users)").unwrap();
    let params = params! {
        "name" => &team.name,
        "description" => &team.description,
        "admin_courses" => &team.right.admin_courses,
        "admin_rankings" => &team.right.admin_rankings,
        "admin_event" => &team.right.admin_event,
        "admin_teams" => &team.right.admin_teams,
        "admin_users" => &team.right.admin_users,
    };

    match conn.exec_drop(&stmt,&params) {
        Err(..) => return Err(Status::BadRequest),
        Ok(..) => (),
    };

    let stmt_id = conn.prep("SELECT LAST_INSERT_ID()").unwrap();

    match conn.exec_first::<u32,_,_>(&stmt_id,params::Params::Empty) {
        Err(..) | Ok(None) => return Err(Status::Conflict),
        Ok(Some(team_id)) => return Ok(team_id.to_string()),
    };
}

#[rocket::post("/admin/team_edit", format = "application/json", data = "<team>")]
pub fn team_edit(session: UserSession, team: Json<Team>) -> Status {
    if !session.right.admin_teams {return Status::Unauthorized};

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("UPDATE teams SET
        name = :name,
        description = :description,
        admin_courses = :admin_courses,
        admin_rankings = :admin_rankings,
        admin_event = :admin_event,
        admin_teams = :admin_teams,
        admin_users = :admin_users
        WHERE team_id = :team_id").unwrap();
    let params = params! {
        "team_id" => &team.id,
        "name" => &team.name,
        "description" => &team.description,
        "admin_courses" => &team.right.admin_courses,
        "admin_rankings" => &team.right.admin_rankings,
        "admin_event" => &team.right.admin_event,
        "admin_teams" => &team.right.admin_teams,
        "admin_users" => &team.right.admin_users,
    };

    match conn.exec_drop(&stmt,&params) {
        Err(..) => Status::InternalServerError,
        Ok(..) => Status::Ok
    }
}

#[rocket::head("/admin/team_delete?<team_id>")]
pub fn team_delete(session: UserSession, team_id: u32) -> Status {
    if !session.right.admin_teams {return Status::Unauthorized};

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE t FROM teams t WHERE t.team_id = :team_id").unwrap();
    let params = params! {"team_id" => team_id};

    match conn.exec_drop(&stmt,&params) {
        Err(..) => Status::InternalServerError,
        Ok(..) => Status::Ok,
    }
}

#[rocket::head("/admin/team_enrol?<team_id>&<user_id>")]
pub fn team_enrol(session: UserSession, team_id: u32, user_id: u32) -> Status {
    if !session.right.admin_teams {return Status::Unauthorized};

    // TODO: remove/add permissions of currently logged-in users
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("INSERT INTO team_members (team_id, user_id)
                          SELECT :team_id, :user_id").unwrap();
    let params = params! {
        "team_id" => &team_id,
        "user_id" => &user_id,
    };

    match conn.exec_drop(&stmt,&params) {
        Err(..) => Status::InternalServerError,
        Ok(..) => Status::Ok,
    }
}

#[rocket::head("/admin/team_dismiss?<team_id>&<user_id>")]
pub fn team_dismiss(session: UserSession, team_id: u32, user_id: u32) -> Status {
    if !session.right.admin_teams {return Status::Unauthorized};

    // TODO: remove/add permissions of currently logged-in users
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE FROM team_members
                          WHERE team_id = :team_id AND e.user_id = :user_id").unwrap();
    let params = params! {
        "team_id" => &team_id,
        "user_id" => &user_id,
    };

    match conn.exec_drop(&stmt,&params) {
        Err(..) => Status::InternalServerError,
        Ok(..) => Status::Ok,
    }
}

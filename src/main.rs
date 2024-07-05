//#![feature(try_blocks)]

use rocket_cors::{AllowedHeaders, AllowedOrigins};

use std::collections::HashSet;

extern crate mysql_common;

mod config;
mod db;
mod db_club;
mod db_competence;
mod db_inventory;
mod db_location;
mod db_skill;
mod db_team;
mod db_term;
mod db_user;

mod common;
mod error;
mod route_login;
mod session;

mod route;

mod route_anon;

mod route_user_admin;
mod route_user_regular;

mod route_club_admin;

mod route_competence_admin;
mod route_competence_regular;

mod route_location_admin;

mod route_inventory_admin;
mod route_inventory_regular;

mod route_skill_admin;

mod route_team_admin;
mod route_team_regular;

mod route_term_admin;

#[rocket::get("/")]
fn index() -> &'static str {
    "Welcome to the CPT server."
}

#[rocket::launch]
fn rocket() -> _ {
    config::readConfig();

    if db::connect_db().is_err() {
        panic!("Database connection failed")
    };

    // Promote an admin user, if demanded by the config, and make him session admin
    if let Some(admin) = crate::config::ADMIN_USER() {
        // Create the user, if not existing
        let elevate: bool = match crate::db_user::user_created_true(&admin) {
            // Database query failed, do not elevate
            Err(_) => false,
            // User is missing, create him
            Ok(false) => {
                let mut user = crate::common::User::from_info(
                    0,
                    admin.clone(),
                    "Placeholder".to_string(),
                    "Placeholder".to_string(),
                    None,
                );
                // Elevate unless database query failed
                crate::db_user::user_create(&mut user).is_ok()
            }
            // User is existing, elevate him
            Ok(true) => true,
        };

        if elevate {
            *crate::session::ADMINSESSION.lock().unwrap() = Some(admin.clone())
        };
    }

    let rocket_config = crate::config::ROCKET_CONFIG();

    // CORS
    let allowed_origins = AllowedOrigins::all();
    let allowed_methods = vec![
        rocket::http::Method::Head,
        rocket::http::Method::Get,
        rocket::http::Method::Post,
        rocket::http::Method::Delete,
    ]
    .into_iter()
    .map(From::from)
    .collect();
    let allowed_headers = AllowedHeaders::some(&["Token", "Accept", "Content-Type"]);
    let expose_headers = HashSet::from(["Error-URI".to_string(), "Error-MSG".to_string()]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods,
        allowed_headers,
        allow_credentials: true,
        expose_headers,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::custom(&rocket_config)
        //.register(catchers![catchers::user_not_found])
        .mount(
            "/",
            rocket::routes![
                index,
                route_anon::status,
                route_anon::location_list,
                route_anon::skill_list,
                route_anon::club_list,
                route_anon::course_list,
                route_anon::user_salt,
                route_login::user_login,
                route_login::event_login,
                route_login::course_login,
                route_login::location_login,
                route_user_admin::user_list,
                route_user_admin::user_detailed,
                route_user_admin::user_create,
                route_user_admin::user_edit,
                route_user_admin::user_edit_password,
                route_user_admin::user_delete,
                route_user_regular::user_info,
                route_user_regular::user_right,
                route_user_regular::user_password,
                route_user_regular::user_list,
                route_club_admin::club_list,
                route_club_admin::club_create,
                route_club_admin::club_edit,
                route_club_admin::club_delete,
                route_club_admin::club_statistic_terms,
                route_club_admin::club_statistic_members,
                route_club_admin::club_statistic_team,
                route::admin::course::course_list,
                route::admin::course::course_create,
                route::admin::course::course_edit,
                route::admin::course::course_delete,
                route::admin::course::course_event_list,
                route::admin::course::course_requirement_list,
                route::admin::course::course_requirement_add,
                route::admin::course::course_requirement_remove,
                route::admin::course::course_statistic_class,
                route::admin::course::course_statistic_leader,
                route::admin::course::course_statistic_leader1,
                route::admin::course::course_statistic_supporter,
                route::admin::course::course_statistic_supporter1,
                route::admin::course::course_statistic_participant,
                route::admin::course::course_statistic_participant1,
                route::admin::course::moderator::course_moderator_list,
                route::admin::course::moderator::course_moderator_add,
                route::admin::course::moderator::course_moderator_remove,
                route::admin::course::leader::sieve_list,
                route::admin::course::leader::sieve_edit,
                route::admin::course::leader::sieve_remove,
                route::admin::course::supporter::sieve_list,
                route::admin::course::supporter::sieve_edit,
                route::admin::course::supporter::sieve_remove,
                route::admin::course::participant::sieve_list,
                route::admin::course::participant::sieve_edit,
                route::admin::course::participant::sieve_remove,
                route::regular::course::course_availability,
                route::moderator::course::course_responsibility,
                route::moderator::course::course_moderator_list,
                route::moderator::course::course_moderator_add,
                route::moderator::course::course_moderator_remove,
                route::admin::event::event_list,
                route::admin::event::event_info,
                route::admin::event::event_credential,
                route::admin::event::event_create,
                route::admin::event::event_edit,
                route::admin::event::event_password_edit,
                route::admin::event::event_course_info,
                route::admin::event::event_course_edit,
                route::admin::event::event_delete,
                route::admin::event::event_accept,
                route::admin::event::event_reject,
                route::admin::event::event_suspend,
                route::admin::event::event_withdraw,
                route::admin::event::statistic_packlist,
                route::admin::event::statistic_division,
                route::admin::event::owner::owner_list,
                route::admin::event::owner::owner_add,
                route::admin::event::owner::owner_remove,
                route::admin::event::leader::registration_list,
                route::admin::event::leader::filter_list,
                route::admin::event::leader::filter_edit,
                route::admin::event::leader::filter_remove,
                route::admin::event::leader::presence_pool,
                route::admin::event::leader::presence_list,
                route::admin::event::leader::presence_add,
                route::admin::event::leader::presence_remove,
                route::admin::event::supporter::registration_list,
                route::admin::event::supporter::filter_list,
                route::admin::event::supporter::filter_edit,
                route::admin::event::supporter::filter_remove,
                route::admin::event::supporter::presence_pool,
                route::admin::event::supporter::presence_list,
                route::admin::event::supporter::presence_add,
                route::admin::event::supporter::presence_remove,
                route::admin::event::participant::registration_list,
                route::admin::event::participant::filter_list,
                route::admin::event::participant::filter_edit,
                route::admin::event::participant::filter_remove,
                route::admin::event::participant::presence_pool,
                route::admin::event::participant::presence_list,
                route::admin::event::participant::presence_add,
                route::admin::event::participant::presence_remove,
                route::moderator::event::event_list,
                route::moderator::event::event_create,
                route::moderator::event::event_edit,
                route::moderator::event::event_edit_password,
                route::moderator::event::event_delete,
                route::regular::event::event_list,
                route::regular::event::event_create,
                route::regular::event::event_owner_true,
                route::regular::event::event_leader_registration_info,
                route::regular::event::event_leader_registration_edit,
                route::regular::event::event_leader_presence_true,
                route::regular::event::event_leader_presence_add,
                route::regular::event::event_leader_presence_remove,
                route::regular::event::event_supporter_registration_info,
                route::regular::event::event_supporter_registration_edit,
                route::regular::event::event_supporter_presence_true,
                route::regular::event::event_supporter_presence_add,
                route::regular::event::event_supporter_presence_remove,
                route::regular::event::event_participant_registration_info,
                route::regular::event::event_participant_registration_edit,
                route::regular::event::event_participant_presence_true,
                route::regular::event::event_participant_presence_add,
                route::regular::event::event_participant_presence_remove,
                route::regular::event::event_bookmark_true,
                route::regular::event::event_bookmark_edit,
                route::owner::event::event_list,
                route::owner::event::event_info,
                route::owner::event::event_edit,
                route::owner::event::event_password_edit,
                route::owner::event::event_delete,
                route::owner::event::event_submit,
                route::owner::event::event_withdraw,
                route::owner::event::event_course_info,
                route::owner::event::event_course_edit,
                route::owner::event::owner::event_owner_list,
                route::owner::event::owner::event_owner_add,
                route::owner::event::owner::event_owner_remove,
                route::owner::event::leader::registration_list,
                route::owner::event::leader::filter_list,
                route::owner::event::leader::filter_edit,
                route::owner::event::leader::filter_remove,
                route::owner::event::leader::presence_pool,
                route::owner::event::leader::presence_list,
                route::owner::event::leader::presence_add,
                route::owner::event::leader::presence_remove,
                route::owner::event::supporter::registration_list,
                route::owner::event::supporter::filter_list,
                route::owner::event::supporter::filter_edit,
                route::owner::event::supporter::filter_remove,
                route::owner::event::supporter::presence_pool,
                route::owner::event::supporter::presence_list,
                route::owner::event::supporter::presence_add,
                route::owner::event::supporter::presence_remove,
                route::owner::event::participant::registration_list,
                route::owner::event::participant::filter_list,
                route::owner::event::participant::filter_edit,
                route::owner::event::participant::filter_remove,
                route::owner::event::participant::presence_pool,
                route::owner::event::participant::presence_list,
                route::owner::event::participant::presence_add,
                route::owner::event::participant::presence_remove,
                route_location_admin::location_list,
                route_location_admin::location_create,
                route_location_admin::location_edit,
                route_location_admin::location_delete,
                route_inventory_regular::possession_list,
                route_inventory_regular::itemcat_list,
                route_inventory_admin::item_list,
                route_inventory_admin::item_create,
                route_inventory_admin::item_edit,
                route_inventory_admin::item_delete,
                route_inventory_admin::itemcat_list,
                route_inventory_admin::itemcat_create,
                route_inventory_admin::itemcat_edit,
                route_inventory_admin::itemcat_delete,
                route_inventory_admin::stock_list,
                route_inventory_admin::stock_edit,
                route_inventory_admin::item_loan,
                route_inventory_admin::item_return,
                route_inventory_admin::item_handout,
                route_inventory_admin::item_handback,
                route_inventory_admin::possession_list,
                route_inventory_admin::possession_create,
                route_inventory_admin::possession_delete,
                route_skill_admin::skill_list,
                route_skill_admin::skill_create,
                route_skill_admin::skill_edit,
                route_skill_admin::skill_delete,
                route_team_admin::team_list,
                route_team_admin::team_create,
                route_team_admin::team_edit,
                route_team_admin::team_right_edit,
                route_team_admin::team_delete,
                route_team_admin::team_member_list,
                route_team_admin::team_member_add,
                route_team_admin::team_member_remove,
                route_team_regular::team_list,
                route_term_admin::term_list,
                route_term_admin::term_create,
                route_term_admin::term_edit,
                route_term_admin::term_delete,
                route_competence_admin::competence_list,
                route_competence_admin::competence_create,
                route_competence_admin::competence_edit,
                route_competence_admin::competence_delete,
                route_competence_regular::competence_list,
                route_competence_regular::competence_summary,
                route::service::event::event_info,
                route::service::event::event_note_edit,
                route::service::event::event_leader_presence_pool,
                route::service::event::event_leader_presence_list,
                route::service::event::event_leader_presence_add,
                route::service::event::event_leader_presence_remove,
                route::service::event::event_supporter_presence_pool,
                route::service::event::event_supporter_presence_list,
                route::service::event::event_supporter_presence_add,
                route::service::event::event_supporter_presence_remove,
                route::service::event::event_participant_presence_pool,
                route::service::event::event_participant_presence_list,
                route::service::event::event_participant_presence_add,
                route::service::event::event_participant_presence_remove,
            ],
        )
        .attach(cors)
}

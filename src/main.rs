//#![feature(try_blocks)]

use rocket_cors::{AllowedHeaders, AllowedOrigins};

use std::collections::HashSet;

extern crate mysql_common;

mod config;
mod db;
mod db_club;
mod db_competence;
mod db_course;
mod db_event;
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

mod route_anon;

mod route_user_admin;
mod route_user_regular;

mod route_club_admin;

mod route_course_admin;
mod route_course_moderator;
mod route_course_regular;

mod route_event_admin;
mod route_event_moderator;
mod route_event_owner;
mod route_event_regular;
mod route_event_service;

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
    let server_config = config::readConfig();

    if db::connect_db(&server_config).is_err() {
        panic!("Database connection failed")
    };

    // Promote an admin user, if demanded by the config, and make him session admin
    if let Some(admin) = server_config.cpt_admin {
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
            *crate::session::ADMINSESSION.lock().unwrap() = Some(admin)
        };
    }

    let rocket_config = rocket::Config {
        address: server_config
            .rocket_address
            .unwrap_or("127.0.0.1".into())
            .parse()
            .unwrap(),
        port: server_config.rocket_port.unwrap_or(8000),
        log_level: server_config
            .rocket_log_level
            .unwrap_or("Normal".into())
            .parse()
            .unwrap(),
        ..rocket::Config::default()
    };

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
                route_course_admin::course_list,
                route_course_admin::course_create,
                route_course_admin::course_edit,
                route_course_admin::course_delete,
                route_course_admin::course_event_list,
                route_course_admin::course_moderator_list,
                route_course_admin::course_moderator_add,
                route_course_admin::course_moderator_remove,
                route_course_admin::course_owner_summon_list,
                route_course_admin::course_owner_summon_add,
                route_course_admin::course_owner_summon_remove,
                route_course_admin::course_owner_unsummon_list,
                route_course_admin::course_owner_unsummon_add,
                route_course_admin::course_owner_unsummon_remove,
                route_course_admin::course_participant_summon_list,
                route_course_admin::course_participant_summon_add,
                route_course_admin::course_participant_summon_remove,
                route_course_admin::course_participant_unsummon_list,
                route_course_admin::course_participant_unsummon_add,
                route_course_admin::course_participant_unsummon_remove,
                route_course_admin::course_requirement_list,
                route_course_admin::course_requirement_add,
                route_course_admin::course_requirement_remove,
                route_course_admin::course_statistic_class,
                route_course_admin::course_statistic_participant,
                route_course_admin::course_statistic_participant1,
                route_course_admin::course_statistic_owner,
                route_course_admin::course_statistic_owner1,
                route_course_regular::course_availability,
                route_course_moderator::course_responsibility,
                route_course_moderator::course_moderator_list,
                route_course_moderator::course_moderator_add,
                route_course_moderator::course_moderator_remove,
                route_event_admin::event_list,
                route_event_admin::event_info,
                route_event_admin::event_create,
                route_event_admin::event_edit,
                route_event_admin::event_password_edit,
                route_event_admin::event_course_info,
                route_event_admin::event_course_edit,
                route_event_admin::event_delete,
                route_event_admin::event_accept,
                route_event_admin::event_deny,
                route_event_admin::event_cancel,
                route_event_admin::event_suspend,
                route_event_admin::event_owner_pool,
                route_event_admin::event_owner_list,
                route_event_admin::event_owner_add,
                route_event_admin::event_owner_remove,
                route_event_admin::event_owner_invite_list,
                route_event_admin::event_owner_invite_add,
                route_event_admin::event_owner_invite_remove,
                route_event_admin::event_owner_uninvite_list,
                route_event_admin::event_owner_uninvite_add,
                route_event_admin::event_owner_uninvite_remove,
                route_event_admin::event_owner_registration_list,
                route_event_admin::event_participant_pool,
                route_event_admin::event_participant_list,
                route_event_admin::event_participant_add,
                route_event_admin::event_participant_remove,
                route_event_admin::event_participant_invite_list,
                route_event_admin::event_participant_invite_add,
                route_event_admin::event_participant_invite_remove,
                route_event_admin::event_participant_uninvite_list,
                route_event_admin::event_participant_uninvite_add,
                route_event_admin::event_participant_uninvite_remove,
                route_event_admin::event_participant_registration_list,
                route_event_admin::event_statistic_preparation,
                route_event_moderator::event_list,
                route_event_moderator::event_create,
                route_event_moderator::event_edit,
                route_event_moderator::event_edit_password,
                route_event_moderator::event_delete,
                route_event_regular::event_list,
                route_event_regular::event_create,
                route_event_regular::event_owner_true,
                route_event_regular::event_participant_true,
                route_event_regular::event_participant_add,
                route_event_regular::event_participant_remove,
                route_event_regular::event_bookmark_true,
                route_event_regular::event_bookmark_edit,
                route_event_regular::event_owner_registration_status,
                route_event_regular::event_owner_registration_edit,
                route_event_regular::event_participant_registration_status,
                route_event_regular::event_participant_registration_edit,
                route_event_owner::event_list,
                route_event_owner::event_info,
                route_event_owner::event_edit,
                route_event_owner::event_password_edit,
                route_event_owner::event_delete,
                route_event_owner::event_submit,
                route_event_owner::event_withdraw,
                route_event_owner::event_cancel,
                route_event_owner::event_recycle,
                route_event_owner::event_owner_list,
                route_event_owner::event_owner_add,
                route_event_owner::event_owner_remove,
                route_event_owner::event_participant_list,
                route_event_owner::event_participant_add,
                route_event_owner::event_participant_remove,
                route_location_admin::location_list,
                route_location_admin::location_create,
                route_location_admin::location_edit,
                route_location_admin::location_delete,
                route_inventory_regular::possession_list,
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
                route_event_service::event_info,
                route_event_service::event_note_edit,
                route_event_service::event_participant_pool,
                route_event_service::event_participant_list,
                route_event_service::event_participant_add,
                route_event_service::event_participant_remove,
                route_event_service::event_owner_pool,
                route_event_service::event_owner_list,
                route_event_service::event_owner_add,
                route_event_service::event_owner_remove,
            ],
        )
        .attach(cors)
}

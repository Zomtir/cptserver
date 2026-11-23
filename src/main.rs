#![allow(clippy::too_many_arguments)]

use axum::http::header::{self, HeaderName};
use axum::http::Method;
use axum::routing::{delete, get, post};
use axum::Router;
use tower_http::cors::{Any, CorsLayer};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

extern crate mysql_common;

mod common;
mod config;
mod db;
mod error;
mod fs;
mod permission;
mod route;
mod session;
mod utils;

#[derive(Clone)]
struct AppState {
    db: mysql::Pool,
    admin_session: Arc<Mutex<Option<String>>>,
    user_sessions: Arc<Mutex<HashMap<String, session::UserSession>>>,
    event_sessions: Arc<Mutex<HashMap<String, session::EventSession>>>,
}

#[tokio::main]
async fn main() {
    let path_home_exe: Option<std::path::PathBuf> = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()));
    let path_home_env = std::env::var("CPT_PATH_HOME").ok().map(std::path::PathBuf::from);
    let path_home = path_home_env.or(path_home_exe);

    fs::init_paths(path_home);

    config::read_config();

    let url = crate::config::DB_URL();
    let opts = mysql::Opts::from_url(&url).expect("Invalid database URL");
    let pool = mysql::Pool::new(opts).expect("Failed to create database pool");

    let mut conn = pool.get_conn().expect("Database connection failed");
    db::migrate_scheme(&mut conn, &crate::config::DB_NAME()).expect("Database update failed");

    let session_admin = permission::promote_user_to_admin(&mut conn).expect("Admin elevation failed");

    // Setup AppState
    let app_state = AppState {
        db: pool,
        admin_session: Arc::new(Mutex::new(session_admin)),
        user_sessions: Arc::new(Mutex::new(HashMap::new())),
        event_sessions: Arc::new(Mutex::new(HashMap::new())),
    };

    // CORS
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::HEAD])
        .allow_headers([
            header::ACCEPT,
            header::CONTENT_TYPE,
            // TODO replace by Authorization header
            HeaderName::from_static("token"),
        ])
        .expose_headers([
            HeaderName::from_static("error-uri"),
            HeaderName::from_static("error-msg"),
        ])
        // TODO: Enable credentials when token is retired in Favour of Authorization header
        .allow_credentials(false);

    // Router
    let app = Router::new()
        .route("/", get(route::generic::index))
        .route("/status", get(route::generic::status))
        .route("/version", get(route::generic::version))
        .route("/user_login", post(route::login::user_login))
        .route("/event_login", post(route::login::event_login))
        .route("/course_login/{course_key}", get(route::login::course_login))
        .route("/location_login/{location_key}", get(route::login::location_login))
        .route("/user/salt/{user_key}", get(route::user::user_salt))
        .route("/user/all", get(route::user::user_all))
        .route("/user/list", get(route::user::user_list))
        .route("/user/create", post(route::user::user_create))
        .route("/user/{user_id}/right", get(route::user::user_right))
        .route("/user/{user_id}/info", get(route::user::user_info))
        .route("/user/{user_id}/detailed", get(route::user::user_detailed))
        .route("/user/{user_id}/edit", post(route::user::user_edit))
        .route("/user/{user_id}/delete", delete(route::user::user_delete))
        .route("/user/{user_id}/password/info", get(route::user::user_password_info))
        .route(
            "/user/{user_id}/password/create",
            post(route::user::user_password_create),
        )
        .route("/user/{user_id}/password/edit", post(route::user::user_password_edit))
        .route(
            "/user/{user_id}/password/delete",
            delete(route::user::user_password_delete),
        )
        .route("/user/{user_id}/image", get(route::user::user_image))
        .route(
            "/user/{user_id}/bank_account/create",
            post(route::user::user_bank_account_create),
        )
        .route(
            "/user/{user_id}/bank_account/edit",
            post(route::user::user_bank_account_edit),
        )
        .route(
            "/user/{user_id}/bank_account/delete",
            delete(route::user::user_bank_account_delete),
        )
        .route(
            "/user/{user_id}/license/main/create",
            post(route::user::user_license_main_create),
        )
        .route(
            "/user/{user_id}/license/main/edit",
            post(route::user::user_license_main_edit),
        )
        .route(
            "/user/{user_id}/license/main/delete",
            delete(route::user::user_license_main_delete),
        )
        .route(
            "/user/{user_id}/license/extra/create",
            post(route::user::user_license_extra_create),
        )
        .route(
            "/user/{user_id}/license/extra/edit",
            post(route::user::user_license_extra_edit),
        )
        .route(
            "/user/{user_id}/license/extra/delete",
            delete(route::user::user_license_extra_delete),
        )
        .route(
            "/user/{user_id}/competence/list",
            get(route::user::competence::competence_list),
        )
        .route(
            "/user/{user_id}/competence/info",
            get(route::user::competence::competence_info),
        )
        .route(
            "/user/{user_id}/competence/summary",
            get(route::user::competence::competence_summary),
        )
        .route(
            "/user/{user_id}/competence/create",
            post(route::user::competence::competence_create),
        )
        .route(
            "/user/{user_id}/competence/edit/{competence_id}",
            post(route::user::competence::competence_edit),
        )
        .route(
            "/user/{user_id}/competence/delete/{competence_id}",
            delete(route::user::competence::competence_delete),
        )
        .route("/organisation/list", get(route::organisation::organisation_list))
        .route(
            "/organisation/{organisation_id}/info",
            get(route::organisation::organisation_info),
        )
        .route("/organisation/create", post(route::organisation::organisation_create))
        .route(
            "/organisation/{organisation_id}/edit",
            post(route::organisation::organisation_edit),
        )
        .route(
            "/organisation/{organisation_id}/delete",
            delete(route::organisation::organisation_delete),
        )
        .route(
            "/organisation/{organisation_id}/affiliation/list",
            get(route::organisation::affiliation_list),
        )
        .route(
            "/organisation/{organisation_id}/affiliation/user/{user_id}/info",
            get(route::organisation::affiliation_info),
        )
        .route(
            "/organisation/{organisation_id}/affiliation/user/{user_id}/create",
            post(route::organisation::affiliation_create),
        )
        .route(
            "/organisation/{organisation_id}/affiliation/user/{user_id}/edit",
            post(route::organisation::affiliation_edit),
        )
        .route(
            "/organisation/{organisation_id}/affiliation/user/{user_id}/delete",
            delete(route::organisation::affiliation_delete),
        )
        .route("/location/list", get(route::location::location_list))
        .route("/location/create", post(route::location::location_create))
        .route("/location/{location_id}/edit", post(route::location::location_edit))
        .route(
            "/location/{location_id}/delete",
            delete(route::location::location_delete),
        )
        .route("/discipline/list", get(route::discipline::discipline_list))
        .route("/discipline/create", post(route::discipline::discipline_create))
        .route(
            "/discipline/{discipline_id}/edit",
            post(route::discipline::discipline_edit),
        )
        .route(
            "/discipline/{discipline_id}/delete",
            delete(route::discipline::discipline_delete),
        )
        .route("/skill/list", get(route::skill::skill_list))
        .route("/skill/create", post(route::skill::skill_create))
        .route("/skill/{skill_id}/edit", post(route::skill::skill_edit))
        .route("/skill/{skill_id}/delete", delete(route::skill::skill_delete))
        .route("/item/list", get(route::inventory::item_list))
        .route("/item/{item_id}/info", get(route::inventory::item_info))
        .route("/item/create", post(route::inventory::item_create))
        .route("/item/{item_id}/edit", post(route::inventory::item_edit))
        .route("/item/{item_id}/delete", delete(route::inventory::item_delete))
        .route("/itemcat/list", get(route::inventory::itemcat_list))
        .route("/itemcat/create", post(route::inventory::itemcat_create))
        .route("/itemcat/{category_id}/edit", post(route::inventory::itemcat_edit))
        .route(
            "/itemcat/{category_id}/delete",
            delete(route::inventory::itemcat_delete),
        )
        .route(
            "/equipment/list",
            get(route::inventory::user_equipment::user_equipment_list),
        )
        .route(
            "/equipment/{equipment_id}/info",
            get(route::inventory::user_equipment::user_equipment_info),
        )
        .route(
            "/equipment/create",
            post(route::inventory::user_equipment::user_equipment_create),
        )
        .route(
            "/equipment/{equipment_id}/edit",
            post(route::inventory::user_equipment::user_equipment_edit),
        )
        .route(
            "/equipment/{equipment_id}/delete",
            delete(route::inventory::user_equipment::user_equipment_delete),
        )
        .route("/stock/list", get(route::inventory::club_stock::stock_list))
        .route("/stock/create", post(route::inventory::club_stock::stock_create))
        .route("/stock/{stock_id}/edit", post(route::inventory::club_stock::stock_edit))
        .route(
            "/stock/{stock_id}/delete",
            delete(route::inventory::club_stock::stock_delete),
        )
        .route("/stock/{stock_id}/loan", post(route::inventory::club_stock::item_loan))
        .route(
            "/possession/{possession_id}/return",
            delete(route::inventory::club_stock::item_return),
        )
        .route(
            "/possession/{possession_id}/handout",
            post(route::inventory::club_stock::item_handout),
        )
        .route(
            "/possession/{possession_id}/stock/{stock_id}/restock",
            post(route::inventory::club_stock::item_restock),
        )
        .route(
            "/possession/list",
            get(route::inventory::user_possession::possession_list),
        )
        .route(
            "/possession/create",
            post(route::inventory::user_possession::possession_create),
        )
        .route(
            "/possession/{possession_id}/delete",
            delete(route::inventory::user_possession::possession_delete),
        )
        .route("/club/list", get(route::club::club_list))
        .route("/club/{club_id}/image", get(route::club::club_image))
        .route("/club/{club_id}/banner", get(route::club::club_banner))
        .route("/club/{club_id}/info", get(route::club::club_info))
        .route("/club/create", post(route::club::club_create))
        .route("/club/{club_id}/edit", post(route::club::club_edit))
        .route("/club/{club_id}/delete", delete(route::club::club_delete))
        .route("/club/{club_id}/statistic/terms", get(route::club::statistic_terms))
        .route("/club/{club_id}/statistic/members", get(route::club::statistic_members))
        .route("/club/{club_id}/statistic/team", get(route::club::statistic_team))
        .route(
            "/club/{club_id}/statistic/organisation",
            get(route::club::statistic_organisation),
        )
        .route(
            "/club/{club_id}/statistic/attendance",
            get(route::club::statistic_attendance),
        )
        .route("/club/term/list", get(route::club::term_list))
        .route("/club/term/{term_id}/info", get(route::club::term_info))
        .route("/club/term/create", post(route::club::term_create))
        .route("/club/term/{term_id}/edit", post(route::club::term_edit))
        .route("/club/term/{term_id}/delete", delete(route::club::term_delete))
        .route(
            "/club/term/{term_id}/discipline/create",
            post(route::club::term_discipline_create),
        )
        .route(
            "/club/term/discipline/{term_discipline_id}/edit",
            post(route::club::term_discipline_edit),
        )
        .route(
            "/club/term/discipline/{term_discipline_id}/delete",
            delete(route::club::term_discipline_delete),
        )
        .route("/course/all", get(route::course::course_all))
        .route("/course/list", get(route::course::course_list))
        .route("/course/create", post(route::course::course_create))
        .route("/course/{course_id}/edit", post(route::course::course_edit))
        .route("/course/{course_id}/delete", delete(route::course::course_delete))
        .route("/course/{course_id}/event/list", get(route::course::course_event_list))
        .route("/course/{course_id}/club/info", get(route::course::course_club_info))
        .route("/course/{course_id}/club/edit", post(route::course::course_club_edit))
        .route(
            "/course/{course_id}/moderators",
            get(route::course::moderator::course_moderator_list),
        )
        .route(
            "/course/{course_id}/moderator/id/{user_id}/add",
            post(route::course::moderator::course_moderator_add),
        )
        .route(
            "/course/{course_id}/moderator/id/{user_id}/remove",
            delete(route::course::moderator::course_moderator_remove),
        )
        .route(
            "/course/{course_id}/moderator/id/{user_id}/true",
            get(route::course::moderator::course_moderator_true),
        )
        .route(
            "/course/{course_id}/attendance/sieve/list",
            get(route::course::attendance::sieve_list),
        )
        .route(
            "/course/{course_id}/attendance/sieve/edit",
            post(route::course::attendance::sieve_edit),
        )
        .route(
            "/course/{course_id}/attendance/sieve/remove",
            delete(route::course::attendance::sieve_remove),
        )
        .route(
            "/course/{course_id}/requirement/list",
            get(route::course::requirement::course_requirement_list),
        )
        .route(
            "/course/{course_id}/requirement/add",
            post(route::course::requirement::course_requirement_add),
        )
        .route(
            "/course/{course_id}/requirement/remove",
            delete(route::course::requirement::course_requirement_remove),
        )
        .route(
            "/course/user/{user_id}/responsibility",
            get(route::course::course_responsibility),
        )
        .route(
            "/course/user/{user_id}/availability",
            get(route::course::course_availability),
        )
        .route(
            "/course/{course_id}/statistic/class",
            get(route::course::statistic::course_statistic_class),
        )
        .route(
            "/course/{course_id}/statistic/attendance",
            get(route::course::statistic::course_statistic_attendance),
        )
        .route(
            "/course/{course_id}/statistic/attendance/user",
            get(route::course::statistic::course_statistic_attendance_user),
        )
        .route("/event/list", get(route::event::event_list))
        .route("/event/{event_id}/info", get(route::event::event_info))
        .route("/event/create", post(route::event::event_create))
        .route("/event/draft", post(route::event::event_draft))
        .route("/event/{event_id}/edit", post(route::event::event_edit))
        .route("/event/{event_id}/delete", delete(route::event::event_delete))
        .route(
            "/event/{event_id}/acceptance/edit",
            post(route::event::event_acceptance_edit),
        )
        .route(
            "/event/{event_id}/credential/info",
            get(route::event::event_credential_info),
        )
        .route(
            "/event/{event_id}/password/edit",
            post(route::event::event_password_edit),
        )
        .route(
            "/event/{event_id}/bookmark/true",
            get(route::event::event_bookmark_true),
        )
        .route(
            "/event/{event_id}/bookmark/edit",
            post(route::event::event_bookmark_edit),
        )
        .route("/event/{event_id}/course/info", get(route::event::course::course_info))
        .route("/event/{event_id}/course/edit", post(route::event::course::course_edit))
        .route(
            "/event/{event_id}/moderator/true/{user_id}",
            get(route::event::course::moderator_true),
        )
        .route(
            "/event/{event_id}/statistic/packlist",
            get(route::event::statistic::statistic_packlist),
        )
        .route(
            "/event/{event_id}/statistic/organisation",
            get(route::event::statistic::statistic_organisation),
        )
        .route(
            "/event/{event_id}/attendance/registration/list",
            get(route::event::attendance::registration_list),
        )
        .route(
            "/event/{event_id}/attendance/registration/info",
            get(route::event::attendance::registration_info),
        )
        .route(
            "/event/{event_id}/attendance/registration/edit",
            post(route::event::attendance::registration_edit),
        )
        .route(
            "/event/{event_id}/attendance/presence/true/{user_id}/{role}",
            get(route::event::attendance::presence_true),
        )
        .route(
            "/event/{event_id}/attendance/filter/list",
            get(route::event::attendance::filter_list),
        )
        .route(
            "/event/{event_id}/attendance/filter/edit",
            post(route::event::attendance::filter_edit),
        )
        .route(
            "/event/{event_id}/attendance/filter/delete",
            delete(route::event::attendance::filter_remove),
        )
        .route(
            "/event/{event_id}/attendance/presence/pool",
            get(route::event::attendance::presence_pool),
        )
        .route(
            "/event/{event_id}/attendance/presence/list",
            get(route::event::attendance::presence_list),
        )
        .route(
            "/event/{event_id}/attendance/presence/add",
            post(route::event::attendance::presence_add),
        )
        .route(
            "/event/{event_id}/attendance/presence/remove",
            delete(route::event::attendance::presence_remove),
        )
        .route(
            "/event/{event_id}/attendance/presence/true",
            get(route::event::attendance::presence_true),
        )
        .route("/event/{event_id}/submit", post(route::event::event_submit))
        .route("/event/{event_id}/withdraw", post(route::event::event_withdraw))
        .route("/event/{event_id}/owner/list", get(route::event::owner::owner_list))
        .route("/event/{event_id}/owner/true", get(route::event::owner::owner_true))
        .route(
            "/event/{event_id}/owner/add/{user_id}",
            post(route::event::owner::owner_add),
        )
        .route(
            "/event/{event_id}/owner/remove/{user_id}",
            delete(route::event::owner::owner_remove),
        )
        .route("/event/service/info", get(route::event::service::event_info))
        .route("/event/service/note/edit", post(route::event::service::event_note_edit))
        .route(
            "/event/service/attendance/presence/pool",
            get(route::event::service::attendance_presence_pool),
        )
        .route(
            "/event/service/attendance/presence/list",
            get(route::event::service::attendance_presence_list),
        )
        .route(
            "/event/service/attendance/presence/add",
            post(route::event::service::attendance_presence_add),
        )
        .route(
            "/event/service/attendance/presence/remove",
            delete(route::event::service::attendance_presence_remove),
        )
        .route("/team/all", get(route::team::team_all))
        .route("/team/list", get(route::team::team_list))
        .route("/team/info/{team_id}", get(route::team::team_info))
        .route("/team/create", post(route::team::team_create))
        .route("/team/{team_id}/edit", post(route::team::team_edit))
        .route("/team/{team_id}/right/edit", post(route::team::team_right_edit))
        .route("/team/{team_id}/delete", delete(route::team::team_delete))
        .route("/team/{team_id}/member/list", get(route::team::team_member_list))
        .route(
            "/team/{team_id}/member/add/{user_id}",
            post(route::team::team_member_add),
        )
        .route(
            "/team/{team_id}/member/remove/{user_id}",
            delete(route::team::team_member_remove),
        )
        .with_state(app_state)
        .layer(cors_layer);

    // Start server
    let addr = crate::config::SERVER_URL();
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to create Tokio listener");
    axum::serve(listener, app).await.expect("Failed to serve application");
}

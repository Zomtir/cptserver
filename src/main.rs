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
async fn main() -> () {
    let path_home_exe: Option<std::path::PathBuf> = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()));
    let path_home_env = std::env::var("CPT_PATH_HOME").ok().map(|p| std::path::PathBuf::from(p));
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
        .route("/user_login", post(route::login::user_login))
        .route("/event_login", post(route::login::event_login))
        .route("/course_login/{course_key}", get(route::login::course_login))
        .route("/location_login/{location_key}", get(route::login::location_login))
        .route("/user/{user_key}/salt", get(route::user::user_salt))
        .route("/user/list", get(route::user::user_list))
        .route("/user/create", post(route::user::user_create))
        .route("/user/id/{user_id}/right", get(route::user::user_right))
        .route("/user/id/{user_id}/info", get(route::user::user_info))
        .route("/user/id/{user_id}/detailed", get(route::user::user_detailed))
        .route("/user/id/{user_id}/edit", post(route::user::user_edit))
        .route("/user/id/{user_id}/delete", delete(route::user::user_delete))
        .route("/user/id/{user_id}/password/info", get(route::user::user_password_info))
        .route(
            "/user/id/{user_id}/password/create",
            post(route::user::user_password_create),
        )
        .route(
            "/user/id/{user_id}/password/edit",
            post(route::user::user_password_edit),
        )
        .route(
            "/user/id/{user_id}/password/delete",
            delete(route::user::user_password_delete),
        )
        .route("/user/id/{user_id}/image", get(route::user::user_image))
        .route("/organisation/list", get(route::organisation::organisation_list))
        .route(
            "/organisation/id/{organisation_id}/info",
            get(route::organisation::organisation_info),
        )
        .route("/organisation/create", post(route::organisation::organisation_create))
        .route(
            "/organisation/id/{organisation_id}/edit",
            post(route::organisation::organisation_edit),
        )
        .route(
            "/organisation/id/{organisation_id}/delete",
            delete(route::organisation::organisation_delete),
        )
        .route(
            "/organisation/affiliation/list",
            get(route::organisation::affiliation_list),
        )
        .route(
            "/organisation/{organisation_id}/affiliation/user/id/{user_id}/info",
            get(route::organisation::affiliation_info),
        )
        .route(
            "/organisation/{organisation_id}/affiliation/user/id/{user_id}/create",
            post(route::organisation::affiliation_create),
        )
        .route(
            "/organisation/{organisation_id}/affiliation/user/id/{user_id}/edit",
            post(route::organisation::affiliation_edit),
        )
        .route(
            "/organisation/{organisation_id}/affiliation/user/id/{user_id}/delete",
            delete(route::organisation::affiliation_delete),
        )
        .route("/location/list", get(route::location::location_list))
        .route("/location/create", post(route::location::location_create))
        .route("/location/id/{location_id}/edit", post(route::location::location_edit))
        .route(
            "/location/id/{location_id}/delete",
            delete(route::location::location_delete),
        )
        .route("/discipline/list", get(route::discipline::discipline_list))
        .route("/discipline/create", post(route::discipline::discipline_create))
        .route(
            "/discipline/id/{discipline_id}/edit",
            post(route::discipline::discipline_edit),
        )
        .route(
            "/discipline/id/{discipline_id}/delete",
            delete(route::discipline::discipline_delete),
        )
        .route("/skill/list", get(route::skill::skill_list))
        .route("/skill/create", post(route::skill::skill_create))
        .route("/skill/id/{skill_id}/edit", post(route::skill::skill_edit))
        .route(
            "/skill/id/{skill_id}/delete",
            delete(route::skill::skill_delete),
        )
        .route("/item/list", get(route::inventory::item_list))
        .route("/item/id/{item_id}/info", get(route::inventory::item_info))
        .route("/item/create", post(route::inventory::item_create))
        .route("/item/id/{item_id}/edit", post(route::inventory::item_edit))
        .route("/item/id/{item_id}/delete", delete(route::inventory::item_delete))
        .route("/itemcat/list", get(route::inventory::itemcat_list))
        .route("/itemcat/create", post(route::inventory::itemcat_create))
        .route(
            "/itemcat/id/{category_id}/edit",
            post(route::inventory::itemcat_edit),
        )
        .route(
            "/itemcat/id/{category_id}/delete",
            delete(route::inventory::itemcat_delete),
        )
        .route("/equipment/list", get(route::inventory::user_equipment::user_equipment_list))
        .route(
            "/equipment/id/{equipment_id}/info",
            get(route::inventory::user_equipment::user_equipment_info),
        )
        .route(
            "/equipment/create",
            post(route::inventory::user_equipment::user_equipment_create),
        )
        .route(
            "/equipment/id/{equipment_id}/edit",
            post(route::inventory::user_equipment::user_equipment_edit),
        )
        .route(
            "/equipment/id/{equipment_id}/delete",
            delete(route::inventory::user_equipment::user_equipment_delete),
        )
        .route("/stock/list", get(route::inventory::club_stock::stock_list))
        .route("/stock/create", post(route::inventory::club_stock::stock_create))
        .route(
            "/stock/id/{stock_id}/edit",
            post(route::inventory::club_stock::stock_edit),
        )
        .route(
            "/stock/id/{stock_id}/delete",
            delete(route::inventory::club_stock::stock_delete),
        )
        .route(
            "/stock/id/{stock_id}/loan",
            post(route::inventory::club_stock::item_loan),
        )
        .route(
            "/possession/id/{possession_id}/return",
            delete(route::inventory::club_stock::item_return),
        )
        .route(
            "/possession/id/{possession_id}/handout",
            post(route::inventory::club_stock::item_handout),
        )
        .route(
            "/possession/id/{possession_id}/stock/id/{stock_id}/restock",
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
            "/possession/id/{possession_id}/delete",
            delete(route::inventory::user_possession::possession_delete),
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
            "/possession/id/{possession_id}/delete",
            delete(route::inventory::user_possession::possession_delete),
        )
        .route("/club/list", get(route::club::club_list))
        .route("/club/id/{club_id}/image", get(route::club::club_image))
        .route("/club/id/{club_id}/banner", get(route::club::club_banner))
        .route("/club/id/{club_id}/info", get(route::club::club_info))
        .route("/club/create", post(route::club::club_create))
        .route("/club/id/{club_id}/edit", post(route::club::club_edit))
        .route("/club/id/{club_id}/delete", delete(route::club::club_delete))
        .route("/club/id/{club_id}/statistic/terms", get(route::club::statistic_terms))
        .route(
            "/club/id/{club_id}/statistic/members",
            get(route::club::statistic_members),
        )
        .route("/club/id/{club_id}/statistic/team", get(route::club::statistic_team))
        .route(
            "/club/id/{club_id}/statistic/organisation",
            get(route::club::statistic_organisation),
        )
        .route(
            "/club/id/{club_id}/statistic/attendance",
            get(route::club::statistic_attendance),
        )
        .route("/club/term/list", get(route::club::term_list))
        .route("/club/term/id/{term_id}/info", get(route::club::term_info))
        .route("/club/term/create", post(route::club::term_create))
        .route("/club/term/id/{term_id}/edit", post(route::club::term_edit))
        .route("/club/term/id/{term_id}/delete", delete(route::club::term_delete))
        .route(
            "/club/term/id/{term_id}/discipline/create",
            post(route::club::term_discipline_create),
        )
        .route(
            "/club/term/discipline/id/{term_discipline_id}/edit",
            post(route::club::term_discipline_edit),
        )
        .route(
            "/club/term/discipline/id/{term_discipline_id}/delete",
            delete(route::club::term_discipline_delete),
        )
        .route(
            "/course/id/{course_id}/moderators",
            get(route::course::moderator::course_moderator_list),
        )
        .route(
            "/course/id/{course_id}/moderator/id/{user_id}/add",
            post(route::course::moderator::course_moderator_add),
        )
        .route(
            "/course/id/{course_id}/moderator/id/{user_id}/remove",
            delete(route::course::moderator::course_moderator_remove),
        )
        .route(
            "/course/user/id/{user_id}/responsibility",
            get(route::course::moderator::course_responsibility),
        )
        .route(
            "/course/user/id/{user_id}/availability",
            get(route::course::course_availability),
        )
        .route("/event/list", get(route::event::event_list))
        .route("/event/id/{event_id}/info", get(route::event::event_info))
        .route("/event/create", post(route::event::event_create))
        .route("/event/id/{event_id}/edit", post(route::event::event_edit))
        .route("/event/id/{event_id}/delete", delete(route::event::event_delete))
        .route(
            "/event/id/{event_id}/acceptance/edit",
            post(route::event::event_acceptance_edit),
        )
        .route(
            "/event/id/{event_id}/credential/info",
            get(route::event::event_credential_info),
        )
        .route(
            "/event/id/{event_id}/password/edit",
            post(route::event::event_password_edit),
        )
        .route("/event/id/{event_id}/course/info", get(route::event::event_course_info))
        .route(
            "/event/id/{event_id}/course/edit",
            post(route::event::event_course_edit),
        )
        .route(
            "/event/id/{event_id}/statistic/packlist",
            get(route::event::statistic_packlist),
        )
        .route(
            "/event/id/{event_id}/statistic/organisation",
            get(route::event::statistic_organisation),
        )
        .route(
            "/event/id/{event_id}/attendance/registration/list",
            get(route::event::attendance::registration_list),
        )
        .route(
            "/event/id/{event_id}/attendance/filter/list",
            get(route::event::attendance::filter_list),
        )
        .route(
            "/event/id/{event_id}/attendance/filter/edit",
            post(route::event::attendance::filter_edit),
        )
        .route(
            "/event/id/{event_id}/attendance/filter/delete",
            delete(route::event::attendance::filter_remove),
        )
        .route(
            "/event/id/{event_id}/attendance/presence/pool",
            get(route::event::attendance::presence_pool),
        )
        .route(
            "/event/id/{event_id}/attendance/presence/list",
            get(route::event::attendance::presence_list),
        )
        .route(
            "/event/id/{event_id}/attendance/presence/add",
            post(route::event::attendance::presence_add),
        )
        .route(
            "/event/id/{event_id}/attendance/presence/remove",
            delete(route::event::attendance::presence_remove),
        )
        .route("/event/id/{event_id}/submit", post(route::event::event_submit))
        .route("/event/id/{event_id}/withdraw", post(route::event::event_withdraw))
        .route("/event/id/{event_id}/owners", get(route::event::owner::owner_list))
        .route(
            "/event/id/{event_id}/owner/id/{user_id}/add",
            post(route::event::owner::owner_add),
        )
        .route(
            "/event/id/{event_id}/owner/id/{user_id}/remove",
            delete(route::event::owner::owner_remove),
        )
        .route("/event/service/info", get(route::event::service::event_info))
        .route(
            "/event/service/note/edit",
            post(route::event::service::event_note_edit),
        )
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
        /*
        route::anon::organisation_list,
        route::anon::club_list,
        route::anon::club_image,
        route::anon::club_banner,
        route::anon::course_list,
        //#[rocket::get("/anon/organisation_list")]
        //#[rocket::get("/anon/skill_list")]
        //#[rocket::get("/anon/club_list")]
        //#[rocket::get("/anon/club_image?<club_id>")]
        //#[rocket::get("/anon/club_banner?<club_id>")]
        //#[rocket::get("/anon/course_list")]
        .route("/admin/event_owner_list", get(route::admin::event::owner::owner_list))
        .route("/admin/event_owner_add", post(route::admin::event::owner::owner_add))
        .route(
            "/admin/event_owner_remove",
            delete(route::admin::event::owner::owner_remove),
        )
            #[rocket::get("/admin/discipline_list")]
            #[rocket::post("/admin/discipline_create", format = "application/json", data = "<discipline>")]
            #[rocket::post(
                "/admin/discipline_edit?<discipline_id>",
                format = "application/json",
                data = "<discipline>"
            )]
            #[rocket::head("/admin/discipline_delete?<discipline_id>")]
            #[rocket::get("/admin/competence_list?<user_id>&<skill_id>&<min>&<max>")]
            #[rocket::get("/admin/competence_info?<competence_id>")]
            #[rocket::post("/admin/competence_create", format = "application/json", data = "<competence>")]
            #[rocket::post(
                "/admin/competence_edit?<competence_id>",
                format = "application/json",
                data = "<competence>"
            )]
            #[rocket::head("/admin/competence_delete?<competence_id>")]
            #[rocket::get("/admin/skill_list")]
            #[rocket::post("/admin/skill_create", format = "application/json", data = "<skill>")]
            #[rocket::post("/admin/skill_edit?<skill_id>", format = "application/json", data = "<skill>")]
            #[rocket::head("/admin/skill_delete?<skill_id>")]
            //#[rocket::get("/admin/event_owner_list?<event_id>")]
            //#[rocket::head("/admin/event_owner_add?<event_id>&<user_id>")]
            //#[rocket::head("/admin/event_owner_remove?<event_id>&<user_id>")]
            #[rocket::get("/service/event_info")]
            #[rocket::post("/service/event_note_edit", format = "text/plain", data = "<note>")]
            #[rocket::get("/service/event_attendance_presence_pool?<role>")]
            #[rocket::get("/service/event_attendance_presence_list?<role>")]
            #[rocket::head("/service/event_attendance_presence_add?<user_id>&<role>")]
            #[rocket::head("/service/event_attendance_presence_remove?<user_id>&<role>")]
            #[rocket::get("/regular/user_info")]
            #[rocket::get("/regular/user_right")]
            #[rocket::get("/regular/user_password_info")]
            #[rocket::post("/regular/user_password_edit", format = "application/json", data = "<credit>")]
            #[rocket::get("/regular/user_image?<user_id>")]
            #[rocket::get("/regular/user_list")]
            #[rocket::get("/regular/team_list")]
            #[rocket::get("/regular/itemcat_list")]
            #[rocket::get("/regular/possession_list?<owned>&<club_id>")]
            #[rocket::get("/admin/user_list?<active>")]
            #[rocket::get("/admin/user_detailed?<user_id>")]
            #[rocket::post("/admin/user_create", format = "application/json", data = "<user>")]
            #[rocket::post("/admin/user_edit?<user_id>", format = "application/json", data = "<user>")]
            #[rocket::head("/admin/user_delete?<user_id>")]
            #[rocket::get("/admin/user_password_info?<user_id>")]
            #[rocket::post(
                "/admin/user_password_create?<user_id>",
                format = "application/json",
                data = "<credit>"
            )]
            #[rocket::post(
                "/admin/user_password_edit?<user_id>",
                format = "application/json",
                data = "<credit>"
            )]
            #[rocket::head("/admin/user_password_delete?<user_id>")]
            #[rocket::get("/owner/event_list?<begin>&<end>&<location_id>&<occurrence>&<acceptance>")]
            #[rocket::get("/owner/event_info?<event_id>")]
            #[rocket::post("/owner/event_edit?<event_id>", format = "application/json", data = "<event>")]
            #[rocket::post("/owner/event_password_edit?<event_id>", format = "text/plain", data = "<password>")]
            #[rocket::get("/owner/event_course_info?<event_id>")]
            #[rocket::head("/owner/event_course_edit?<event_id>&<course_id>")]
            #[rocket::head("/owner/event_submit?<event_id>")]
            #[rocket::head("/owner/event_withdraw?<event_id>")]
            #[rocket::head("/owner/event_delete?<event_id>")]
            #[rocket::get("/admin/team_list")]
            #[rocket::get("/admin/team_info?<team_id>")]
            #[rocket::post("/admin/team_create", format = "application/json", data = "<team>")]
            #[rocket::post("/admin/team_edit?<team_id>", format = "application/json", data = "<team>")]
            #[rocket::post("/admin/team_right_edit?<team_id>", format = "application/json", data = "<right>")]
            #[rocket::head("/admin/team_delete?<team_id>")]
            #[rocket::get("/admin/team_member_list?<team_id>")]
            #[rocket::head("/admin/team_member_add?<team_id>&<user_id>")]
            #[rocket::head("/admin/team_member_remove?<team_id>&<user_id>")]
            #[rocket::get("/admin/organisation_list")]
            #[rocket::get("/admin/organisation_info?<organisation_id>")]
            #[rocket::post("/admin/organisation_create", format = "application/json", data = "<organisation>")]
            #[rocket::post(
                "/admin/organisation_edit?<organisation_id>",
                format = "application/json",
                data = "<organisation>"
            )]
            #[rocket::head("/admin/organisation_delete?<organisation_id>")]
            #[rocket::get("/admin/stock_list?<club_id>&<item_id>")]
            #[rocket::post("/admin/stock_create", format = "application/json", data = "<stock>")]
            #[rocket::post("/admin/stock_edit?<stock_id>", format = "application/json", data = "<stock>")]
            #[rocket::head("/admin/stock_delete?<stock_id>")]
            #[rocket::head("/admin/item_loan?<stock_id>&<user_id>")]
            #[rocket::head("/admin/item_return?<possession_id>")]
            #[rocket::head("/admin/item_handout?<possession_id>")]
            #[rocket::head("/admin/item_restock?<possession_id>&<stock_id>")]
            #[rocket::get("/admin/possession_list?<user_id>&<item_id>&<owned>&<club_id>")]
            #[rocket::head("/admin/possession_create?<user_id>&<item_id>")]
            #[rocket::head("/admin/possession_delete?<possession_id>")]
            #[rocket::get("/admin/course_attendance_sieve_list?<course_id>&<role>")]
            #[rocket::head("/admin/course_attendance_sieve_edit?<course_id>&<team_id>&<role>&<access>")]
            #[rocket::head("/admin/course_attendance_sieve_remove?<course_id>&<team_id>&<role>")]
            #[rocket::get("/admin/course_list?<mod_id>&<active>&<public>")]
            #[rocket::post("/admin/course_create", format = "application/json", data = "<course>")]
            #[rocket::post("/admin/course_edit?<course_id>", format = "application/json", data = "<course>")]
            #[rocket::head("/admin/course_delete?<course_id>")]
            #[rocket::get("/admin/course_event_list?<course_id>")]
            #[rocket::get("/admin/course_requirement_list?<course_id>")]
            #[rocket::head("/admin/course_requirement_add?<course_id>&<skill_id>&<rank>")]
            #[rocket::head("/admin/course_requirement_remove?<requirement_id>")]
            #[rocket::get("/admin/course_club_info?<course_id>")]
            #[rocket::head("/admin/course_club_edit?<course_id>&<club_id>")]
            #[rocket::get("/admin/course_statistic_class?<course_id>")]
            #[rocket::get("/admin/course_statistic_attendance?<course_id>&<role>")]
            #[rocket::get("/admin/course_statistic_attendance1?<course_id>&<user_id>&<role>")]
            #[rocket::get("/admin/course_moderator_list?<course_id>")]
            #[rocket::head("/admin/course_moderator_add?<course_id>&<user_id>")]
            #[rocket::head("/admin/course_moderator_remove?<course_id>&<user_id>")]
            #[rocket::get("/admin/event_attendance_registration_list?<event_id>&<role>")]
            #[rocket::get("/admin/event_attendance_filter_list?<event_id>&<role>")]
            #[rocket::head("/admin/event_attendance_filter_edit?<event_id>&<user_id>&<role>&<access>")]
            #[rocket::head("/admin/event_attendance_filter_remove?<event_id>&<user_id>&<role>")]
            #[rocket::get("/admin/event_attendance_presence_pool?<event_id>&<role>")]
            #[rocket::get("/admin/event_attendance_presence_list?<event_id>&<role>")]
            #[rocket::head("/admin/event_attendance_presence_add?<event_id>&<user_id>&<role>")]
            #[rocket::head("/admin/event_attendance_presence_remove?<event_id>&<user_id>&<role>")]
            #[rocket::get(
                "/admin/event_list?<begin>&<end>&<location_id>&<occurrence>&<acceptance>&<course_true>&<course_id>&<owner_id>"
            )]
            #[rocket::get("/admin/event_info?<event_id>")]
            #[rocket::get("/admin/event_credential?<event_id>")]
            #[rocket::post("/admin/event_create?<course_id>", format = "application/json", data = "<event>")]
            #[rocket::post("/admin/event_edit?<event_id>", format = "application/json", data = "<event>")]
            #[rocket::post("/admin/event_password_edit?<event_id>", format = "text/plain", data = "<password>")]
            #[rocket::get("/admin/event_course_info?<event_id>")]
            #[rocket::head("/admin/event_course_edit?<event_id>&<course_id>")]
            #[rocket::head("/admin/event_delete?<event_id>")]
            #[rocket::get("/admin/event_statistic_packlist?<event_id>&<skill_id>")]
            #[rocket::get("/admin/event_statistic_organisation?<event_id>&<organisation_id>")]
            #[rocket::get("/admin/user_equipment_list?<user_id>&<skill_id>&<item_id>")]
            #[rocket::get("/admin/user_equipment_info?<equipment_id>")]
            #[rocket::head("/admin/user_equipment_create?<user_id>&<skill_id>&<item_id>&<count>")]
            #[rocket::head("/admin/user_equipment_edit?<equipment_id>&<count>")]
            #[rocket::head("/admin/user_equipment_delete?<equipment_id>")]
            #[rocket::get("/admin/affiliation_list?<user_id>&<organisation_id>")]
            #[rocket::get("/admin/affiliation_info?<user_id>&<organisation_id>")]
            #[rocket::head("/admin/affiliation_create?<user_id>&<organisation_id>")]
            #[rocket::post(
                "/admin/affiliation_edit?<user_id>&<organisation_id>",
                format = "application/json",
                data = "<affiliation>"
            )]
            #[rocket::head("/admin/affiliation_delete?<user_id>&<organisation_id>")]
            #[rocket::post(
                "/admin/user_bank_account_create?<user_id>",
                format = "application/json",
                data = "<bank_account>"
            )]
            #[rocket::post(
                "/admin/user_bank_account_edit?<user_id>",
                format = "application/json",
                data = "<bank_account>"
            )]
            #[rocket::head("/admin/user_bank_account_delete?<user_id>")]
            #[rocket::post(
                "/admin/user_license_main_create?<user_id>",
                format = "application/json",
                data = "<license>"
            )]
            #[rocket::post(
                "/admin/user_license_extra_create?<user_id>",
                format = "application/json",
                data = "<license>"
            )]
            #[rocket::post(
                "/admin/user_license_main_edit?<user_id>",
                format = "application/json",
                data = "<license>"
            )]
            #[rocket::post(
                "/admin/user_license_extra_edit?<user_id>",
                format = "application/json",
                data = "<license>"
            )]
            #[rocket::head("/admin/user_license_main_delete?<user_id>")]
            #[rocket::head("/admin/user_license_extra_delete?<user_id>")]
            #[rocket::get("/mod/course_responsibility?<active>&<public>")]
            #[rocket::get("/mod/course_moderator_list?<course_id>")]
            #[rocket::head("/mod/course_moderator_add?<course_id>&<user_id>")]
            #[rocket::head("/mod/course_moderator_remove?<course_id>&<user_id>")]
            #[rocket::get("/mod/event_list?<course_id>")]
            #[rocket::post("/mod/event_create?<course_id>", format = "application/json", data = "<event>")]
            #[rocket::post("/mod/event_edit?<event_id>", format = "application/json", data = "<event>")]
            #[rocket::post("/mod/event_edit_password?<event_id>", format = "text/plain", data = "<password>")]
            #[rocket::head("/mod/event_delete?<event_id>")]
            #[rocket::get("/owner/event_attendance_registration_list?<event_id>&<role>")]
            #[rocket::get("/owner/event_attendance_filter_list?<event_id>&<role>")]
            #[rocket::head("/owner/event_attendance_filter_edit?<event_id>&<user_id>&<role>&<access>")]
            #[rocket::head("/owner/event_attendance_filter_remove?<event_id>&<user_id>&<role>")]
            #[rocket::get("/owner/event_attendance_presence_pool?<event_id>&<role>")]
            #[rocket::get("/owner/event_attendance_presence_list?<event_id>&<role>")]
            #[rocket::head("/owner/event_attendance_presence_add?<event_id>&<user_id>&<role>")]
            #[rocket::head("/owner/event_attendance_presence_remove?<event_id>&<user_id>&<role>")]
            #[rocket::get("/owner/event_owner_list?<event_id>")]
            #[rocket::head("/owner/event_owner_add?<event_id>&<user_id>")]
            #[rocket::head("/owner/event_owner_remove?<event_id>&<user_id>")]
            #[rocket::get("/regular/event_list?<begin>&<end>&<location_id>&<occurrence>&<acceptance>&<course_true>&<course_id>")]
            #[rocket::post("/regular/event_create", format = "application/json", data = "<event>")]
            #[rocket::get("/regular/event_owner_true?<event_id>")]
            #[rocket::get("/regular/event_moderator_true?<event_id>")]
            #[rocket::get("/regular/event_attendance_presence_true?<event_id>&<role>")]
            #[rocket::head("/regular/event_attendance_presence_add?<event_id>&<role>")]
            #[rocket::head("/regular/event_attendance_presence_remove?<event_id>&<role>")]
            #[rocket::get("/regular/event_bookmark_true?<event_id>")]
            #[rocket::head("/regular/event_bookmark_edit?<event_id>&<bookmark>")]
            #[rocket::get("/regular/event_attendance_registration_info?<event_id>&<role>")]
            #[rocket::head("/regular/event_attendance_registration_edit?<event_id>&<role>&<status>")]
            #[rocket::get("/regular/competence_list")]
            #[rocket::get("/regular/competence_summary")]
        */
        .with_state(app_state)
        .layer(cors_layer);

    // Start server
    let addr = crate::config::SERVER_URL();
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to create Tokio listener");
    axum::serve(listener, app).await.expect("Failed to serve application");

    /*
    rocket::routes![
        route::admin::user::user_list,
        route::admin::user::user_detailed,
        route::admin::user::user_create,
        route::admin::user::user_edit,
        route::admin::user::user_delete,
        route::admin::user::user_password_info,
        route::admin::user::user_password_create,
        route::admin::user::user_password_edit,
        route::admin::user::user_password_delete,
        route::admin::user::user_bank_account_create,
        route::admin::user::user_bank_account_edit,
        route::admin::user::user_bank_account_delete,
        route::admin::user::user_license_main_create,
        route::admin::user::user_license_extra_create,
        route::admin::user::user_license_main_edit,
        route::admin::user::user_license_extra_edit,
        route::admin::user::user_license_main_delete,
        route::admin::user::user_license_extra_delete,
        route::regular::user::user_info,
        route::regular::user::user_right,
        route::regular::user::user_password_info,
        route::regular::user::user_password_set,
        route::regular::user::user_list,
        route::regular::user::user_image,
        route::admin::club::club_list,
        route::admin::club::club_info,
        route::admin::club::club_create,
        route::admin::club::club_edit,
        route::admin::club::club_delete,
        route::admin::club::statistic_terms,
        route::admin::club::statistic_members,
        route::admin::club::statistic_team,
        route::admin::club::statistic_organisation,
        route::admin::club::statistic_attendance,
        route::admin::event::event_list,
        route::admin::event::event_info,
        route::admin::event::event_credential,
        route::event::event_create,
        route::event::event_edit,
        route::event::event_password_edit,
        route::event::event_course_info,
        route::event::event_course_edit,
        route::event::event_delete,
        route::event::statistic_packlist,
        route::event::statistic_organisation,
        route::admin::event::owner::owner_list,
        route::admin::event::owner::owner_add,
        route::admin::event::owner::owner_remove,
        route::admin::event::attendance::registration_list,
        route::admin::event::attendance::filter_list,
        route::admin::event::attendance::filter_edit,
        route::admin::event::attendance::filter_remove,
        route::admin::event::attendance::presence_pool,
        route::admin::event::attendance::presence_list,
        route::admin::event::attendance::presence_add,
        route::admin::event::attendance::presence_remove,
        route::regular::event::event_list,
        route::regular::event::event_create,
        route::regular::event::event_owner_true,
        route::regular::event::event_moderator_true,
        route::regular::event::event_attendance_registration_info,
        route::regular::event::event_attendance_registration_edit,
        route::regular::event::event_attendance_presence_true,
        route::regular::event::event_attendance_presence_add,
        route::regular::event::event_attendance_presence_remove,
        route::regular::event::event_bookmark_true,
        route::regular::event::event_bookmark_edit,
        route::admin::organisation::organisation_list,
        route::admin::organisation::organisation_info,
        route::admin::organisation::organisation_create,
        route::admin::organisation::organisation_edit,
        route::admin::organisation::organisation_delete,
        route::admin::organisation::affiliation_list,
        route::admin::organisation::affiliation_info,
        route::admin::organisation::affiliation_create,
        route::admin::organisation::affiliation_edit,
        route::admin::organisation::affiliation_delete,
        route::admin::inventory::stock_list,
        route::admin::inventory::stock_create,
        route::admin::inventory::stock_edit,
        route::admin::inventory::stock_delete,
        route::admin::inventory::item_loan,
        route::admin::inventory::item_return,
        route::admin::inventory::item_handout,
        route::admin::inventory::item_restock
        route::inventory::user_equipment::user_equipment_list,
        route::inventory::user_equipment::user_equipment_info,
        route::inventory::user_equipment::user_equipment_create,
        route::inventory::user_equipment::user_equipment_edit,
        route::inventory::user_equipment::user_equipment_delete,
        route::skill::skill_list,
        route::skill::skill_create,
        route::skill::skill_edit,
        route::skill::skill_delete,
        route::admin::team::team_list,
        route::admin::team::team_info,
        route::admin::team::team_create,
        route::admin::team::team_edit,
        route::admin::team::team_right_edit,
        route::admin::team::team_delete,
        route::admin::team::team_member_list,
        route::admin::team::team_member_add,
        route::admin::team::team_member_remove,
        route::regular::team::team_list,
        route::admin::club::term::term_list,
        route::admin::club::term::term_info,
        route::admin::club::term::term_create,
        route::admin::club::term::term_edit,
        route::admin::club::term::term_delete,
        route::admin::competence::competence_list,
        route::admin::competence::competence_info,
        route::admin::competence::competence_create,
        route::admin::competence::competence_edit,
        route::admin::competence::competence_delete,
        route::regular::competence::competence_list,
        route::regular::competence::competence_summary,
        route::event::service::event_info,
        route::event::service::event_note_edit,
        route::event::service::attendance_presence_pool,
        route::event::service::attendance_presence_list,
        route::event::service::attendance_presence_add,
        route::event::service::attendance_presence_remove,
    ],
     */
}

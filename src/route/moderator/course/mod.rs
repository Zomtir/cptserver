use axum::extract::State;
use axum::Json;

use crate::common::{Course, User, WebBool};
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

pub fn course_responsibility(
    State(state): State<AppState>,
    session: UserSession,
    active: Option<WebBool>,
    public: Option<WebBool>,
) -> Result<Json<Vec<Course>>> {
    let conn = &mut state.db.get_conn()?;
    let courses = crate::db::course::course_list(
        conn,
        Some(session.user.id),
        active.map(|b| b.to_bool()),
        public.map(|b| b.to_bool()),
    )?;
    Ok(Json(courses))
}

pub fn course_moderator_list(
    State(state): State<AppState>,
    session: UserSession,
    course_id: u32,
) -> Result<Json<Vec<User>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_course_moderator(conn, course_id, session.user.id)?;

    let moderators = crate::db::course::moderator::course_moderator_list(conn, course_id)?;
    Ok(Json(moderators))
}

pub fn course_moderator_add(
    State(state): State<AppState>,
    session: UserSession,
    course_id: u32,
    user_id: u64,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_course_moderator(conn, course_id, session.user.id)?;

    crate::db::course::moderator::course_moderator_add(conn, course_id, user_id)?;
    Ok(())
}

pub fn course_moderator_remove(
    State(state): State<AppState>,
    session: UserSession,
    course_id: u32,
    user_id: u64,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_course_moderator(conn, course_id, session.user.id)?;

    crate::db::course::moderator::course_moderator_remove(conn, course_id, user_id)?;
    Ok(())
}

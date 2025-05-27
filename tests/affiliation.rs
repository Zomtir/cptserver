use cptserver;
use cptserver::error::ErrorKind;

use cptserver::common::{Affiliation, Organisation, User};

mod common;

#[test]
fn affiliation() -> Result<(), ErrorKind> {
    let conn = &mut common::get_dbt_conn()?;

    let mut user = User::from_info(0, "key".into(), "first".into(), "last".into(), None);

    let user_id = cptserver::db::user::user_create(conn, &mut user)?;
    let _ = cptserver::db::user::user_list(conn, None)?;
    let _ = cptserver::db::user::user_info(conn, user_id)?;

    let organisation = Organisation {
        id: 0,
        abbreviation: Some("T".into()),
        name: Some("EST".into()),
    };

    let organisation_id = cptserver::db::organisation::organisation_create(conn, &organisation)?;
    let _ = cptserver::db::organisation::organisation_list(conn)?;
    let _ = cptserver::db::organisation::organisation_info(conn, organisation_id)?;

    let affiliation = Affiliation {
        user: None,
        organisation: None,
        member_identifier: None,
        permission_solo_date: None,
        permission_team_date: None,
        residency_move_date: None,
    };

    cptserver::db::organisation::affiliation_create(conn, user_id, organisation_id)?;
    cptserver::db::organisation::affiliation_edit(conn, user_id, organisation_id, &affiliation)?;
    cptserver::db::organisation::affiliation_delete(conn, user_id, organisation_id)?;

    cptserver::db::organisation::organisation_delete(conn, organisation_id)?;
    cptserver::db::user::user_delete(conn, user_id)?;

    Ok(())
}

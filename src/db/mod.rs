pub mod club;
pub mod competence;
pub mod course;
pub mod event;
pub mod inventory;
pub mod location;
pub mod organisation;
pub mod skill;
pub mod team;
pub mod user;

use crate::error::Error;
use mysql::{Pool, PooledConn};
use std::sync::OnceLock;

static POOL: OnceLock<Pool> = OnceLock::new();

pub fn connect_db() -> Result<(), Error> {
    let db_url = crate::config::DB_URL();

    let pool = mysql::Pool::new(mysql::Opts::from_url(&db_url)?)?;
    let _ = POOL.set(pool);

    Ok(())
}

pub fn get_pool_conn() -> PooledConn {
    match POOL.get() {
        None => panic!("No pool available to establish a database connection"),
        Some(pool) => match pool.get_conn() {
            Err(..) => panic!("Pool did not hand out a database connection"),
            Ok(conn) => conn,
        },
    }
}

/// Helper function to debug SQL queries
///
/// ```
/// let query = "INSERT INTO events (event_key, pwd, title, location_id, begin, end, status, public, scrutable, note, course_id)
/// SELECT :event_key, :pwd, :title, :location_id, :begin, :end, :status, :public, :scrutable, :note, :course_id;";
/// crate::db::print_sql(&query, &params);
/// ```
#[allow(dead_code)]
pub fn print_sql(query: &str, params: &mysql::Params) {
    let pnp = mysql_common::named_params::ParsedNamedParams::parse(query.as_bytes()).unwrap();
    let (real_query, real_params) = (pnp.query(), pnp.params());

    let replacement_map = match params {
        mysql::Params::Named(map) => map,
        _ => return,
    };

    let input_string = String::from_utf8(real_query.to_vec()).unwrap();
    let mut param_index = 0;
    let mut output_string = String::new();

    for char in input_string.chars() {
        match char {
            '?' => {
                let s = replacement_map.get(&*real_params[param_index]).unwrap().as_sql(true);
                param_index += 1;
                output_string.push_str(&s);
            }
            s => output_string.push_str(s.to_string().as_str()),
        }
    }

    println!("{}", output_string);
}

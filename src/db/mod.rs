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
use mysql::prelude::Queryable;
use mysql::{Pool, PooledConn};
use std::sync::OnceLock;

static POOL: OnceLock<Pool> = OnceLock::new();

static SCHEME_VERSION : u8 = 0; 

pub fn connect_db() -> Result<(), Error> {
    let db_url = crate::config::DB_URL();

    let pool = mysql::Pool::new(mysql::Opts::from_url(&db_url)?)?;
    let _ = POOL.set(pool);

    Ok(())
}

pub fn update_db() -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();

    let latest_version : u8 = SCHEME_VERSION;

    // Check if the database has tables
    let query_empty = "SELECT COUNT(*) FROM information_schema.tables WHERE TABLE_TYPE = 'BASE TABLE';";
    let is_empty : bool = conn.query_first::<u8,_>(query_empty)?.unwrap() < 1;

    // Case 1: The database is empty and we do a fresh install
    if is_empty {
        let query_schema = match std::fs::read_to_string(format!("sql/schema_{}.sql", latest_version)) {
            Err(_) => return Err(Error::DatabaseError),
            // Load the schema, and inser the latest version number
            Ok(schema) => format!("{}\nINSERT INTO _info (version) VALUES ({});", schema, latest_version),
        };
        conn.query_drop(&query_schema)?;
    }
    else {
        // Check if schema info exists
        let query_info = "SELECT COUNT(*) FROM information_schema.tables WHERE table_name = '_info';";
        let has_info : bool = conn.query_first::<u8,_>(query_info)?.unwrap() > 0;

        // Case 2: Schema info is missing which is taken as indicator of schema version 0
        if !has_info {
            // Run the script that makes the version 0 explicit
            let query_update0 = match std::fs::read_to_string("sql/update_0.sql") {
                Err(_) => return Err(Error::DatabaseError),
                Ok(update0) => update0,
            };
            conn.query_drop(&query_update0)?;
        }

        // Case 3: Schema info exists or was set in case 2
        let query_version = "SELECT version FROM _info;";
        let mut current_version : u8 = conn.query_first::<u8,_>(query_version)?.unwrap();

        // Do incremental upgrades
        while current_version < latest_version {
            let query_update = match std::fs::read_to_string(format!("sql/update_{}.sql", current_version+1)) {
                Err(_) => return Err(Error::DatabaseError),
                Ok(update) => update,
            };
            conn.query_drop(query_update)?;
            current_version += 1;
        }
    }

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

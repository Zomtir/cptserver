pub mod club;
pub mod course;
pub mod event;

use mysql::{Pool, PooledConn};

use crate::error::Error;

static mut POOL: Option<Pool> = None;

pub fn connect_db() -> Result<(), Error> {
    let db_url = crate::config::DB_URL();

    unsafe {
        let pool = mysql::Pool::new(mysql::Opts::from_url(&db_url)?)?;
        POOL = Some(pool);
    }
    Ok(())
}

pub fn get_pool_conn() -> PooledConn {
    unsafe {
        match &POOL {
            None => panic!("No pool available to establish a database connection"),
            Some(pool) => match pool.clone().get_conn() {
                Err(..) => panic!("Pool did not hand out a database connection"),
                Ok(conn) => conn,
            },
        }
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

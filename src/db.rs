use mysql::{Pool, PooledConn};

use crate::error::Error;

static mut POOL: Option<Pool> = None;

pub fn connect_db(server_conf: &crate::config::ServerConfig) -> Result<(), Error> {
    let db_url = format!(
        "mysql://{user}:{password}@{server}:{port}/{database}",
        server = server_conf.db_server.clone().unwrap_or("127.0.0.1".into()),
        port = server_conf.db_port.clone().unwrap_or(3306),
        database = server_conf.db_database.clone().unwrap_or("cptdb".into()),
        user = server_conf.db_user.clone().unwrap_or("cptdb-user".into()),
        password = server_conf.db_password.clone().unwrap_or_default(),
    );

    unsafe {
        POOL = match mysql::Pool::new(mysql::Opts::from_url(&db_url)?)? {
            pool => Some(pool),
        };
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

/*
let query = "INSERT INTO slots (slot_key, pwd, title, location_id, begin, end, status, public, obscured, note, course_id)
SELECT :slot_key, :pwd, :title, :location_id, :begin, :end, :status, :public, :obscured, :note, :course_id;";
println!("SQL: {}", crate::db::as_sql(&query, &params).unwrap()); 
 */
#[allow(dead_code)]
pub fn as_sql(query: &str, params: &mysql::Params) -> Result<String, Error> {
    let (placeholder, real_query) : (Vec<Vec<u8>>, Vec<u8>) = match mysql_common::named_params::parse_named_params(query.as_bytes()).unwrap() {
        (p,q) => (p.unwrap(), q.into_owned()), 
    };

    println!("Real Query: {}", String::from_utf8(real_query.clone()).unwrap());
    println!("Named Param Size: {}", placeholder.len());

    let replacement_map = match params {
        mysql::Params::Named(map) => map,
        _ => return Err(Error::Default),
    };

    let input_string = String::from_utf8(real_query.clone()).unwrap();
    let mut param_index = 0;
    let mut output_string = String::new();

    for char in input_string.chars() {
        match char {
            '?' => {
                let s = replacement_map.get(&placeholder[param_index]).unwrap().as_sql(true);
                param_index += 1;
                output_string.push_str(&s);
            },
            s => output_string.push_str(s.to_string().as_str()),
        }
    }

    output_string.push_str("#END");
    Ok(output_string)
}

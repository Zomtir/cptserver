use mysql::{Pool, PooledConn, params};
use mysql::prelude::Queryable;

static mut POOL : Option<Pool> = None;


pub fn connect_db(server_conf: &crate::config::ServerConfig) {
    let db_url = format!("mysql://{user}:{password}@{server}:{port}/{database}",
        server =   server_conf.db_server,
        port =     server_conf.db_port,
        database = server_conf.db_database,
        user =     server_conf.db_user,
        password = server_conf.db_password,

    );

    unsafe {
    POOL = match mysql::Pool::new(mysql::Opts::from_url(&db_url).unwrap()) {
        Err(error) => {panic!(" => DB connection failed: {:?}", error)},
        Ok(pool) => Some(pool),
    };
    }
}

pub fn get_pool_conn() -> PooledConn {
    unsafe {
    match &POOL {
        None => panic!("No pool available to establish a database connection"),
        Some(pool) => match pool.clone().get_conn() {
            Err(..) => panic!("Pool did not hand out a database connection"),
            Ok(conn) => conn,
        }
    }
    }
}

pub fn get_last_id<T: mysql::prelude::FromValue>(mut conn: PooledConn) -> Option<T> {
    let stmt = conn.prep("SELECT LAST_INSERT_ID()").unwrap();
    let params = params::Params::Empty;

    match conn.exec_first::<T, _, _>(&stmt, &params) {
        Err(..) => None,
        Ok(id) => id,
    }
}

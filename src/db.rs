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

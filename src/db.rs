use serde::{Serialize, Deserialize};
use mysql::{Pool, PooledConn, params};
use mysql::prelude::Queryable;

static mut POOL : Option<Pool> = None;

#[derive(Serialize, Deserialize)]
struct DatabaseConfig {
    server: String,
    port: u16,
    user: String,
    password: String,
    database: String,
}

// This let's us assume that a config is present even if there is none
impl ::std::default::Default for DatabaseConfig {
    fn default() -> Self { Self {
        server: "localhost".into(),
        port: 3306,
        user: "db-user".into(),
        password: "db-password".into(),
        database: "cpt".into(),
    } }
}

pub fn connect_db() {
    let mut confdir: String = match std::env::var("ROCKET_CONFIG") {
        Err(..) => ".".to_string(),
        Ok(dir) => dir,
    };

    if confdir.is_empty() {confdir = ".".to_string()}

    let confpath = format!("{}/{}", confdir, "Database.toml");

    let db_conf : DatabaseConfig = confy::load_path(confpath).unwrap();
    println!("\u{1F5C4}  Configured DB server.");
    println!("    => address: {}", db_conf.server);
    println!("    => port: {:?}", db_conf.port);

    let db_url = format!("mysql://{user}:{password}@{server}:{port}/{database}",
        server =   db_conf.server,
        port =     db_conf.port,
        user =     db_conf.user,
        password = db_conf.password,
        database = db_conf.database,
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

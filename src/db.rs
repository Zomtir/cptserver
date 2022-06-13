use serde::{Serialize, Deserialize};
use mysql::{Pool, PooledConn};

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
    let db_conf : DatabaseConfig = confy::load_path("./Database.toml").unwrap();
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

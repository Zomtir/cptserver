use cptserver::db;
use cptserver::error::ErrorKind;
use mysql::{Pool, PooledConn};
use std::sync::OnceLock;

static DBTPOOL: OnceLock<Pool> = OnceLock::new();

pub fn init_dbt_pool() -> Result<(), ErrorKind> {
    let url = dbt_url();
    let pool = mysql::Pool::new(mysql::Opts::from_url(&url)?)?;
    let _ = DBTPOOL.set(pool);

    Ok(())
}

pub fn get_dbt_conn() -> Result<PooledConn, ErrorKind> {
    let pool = DBTPOOL
        .get()
        .or_else(|| {
            init_dbt_pool().ok();
            println!("DB_TEST: Initialized database pool");
            let pool = DBTPOOL.get()?;
            println!("DB_TEST: Establshing database connection");
            let mut conn = pool.get_conn().ok()?;
            println!("DB_TEST: Migrating database scheme");
            db::migrate_scheme(&mut conn, &dbt_name()).ok()?;
            Some(pool)
        })
        .ok_or(ErrorKind::DatabasePool)?;

    pool.get_conn().map_err(|_| ErrorKind::DatabaseConnection)
}

pub fn dbt_name() -> String {
    std::env::var("CPTDB_TEST_DATABASE").unwrap_or("cptdbt".to_string())
}

pub fn dbt_url() -> String {
    format!(
        "mysql://{user}:{password}@{server}:{port}/{database}",
        server = std::env::var("CPTDB_TEST_SERVER").unwrap_or("localhost".to_string()),
        port = std::env::var("CPTDB_TEST_PORT").unwrap_or("3306".to_string()),
        database = std::env::var("CPTDB_TEST_DATABASE").unwrap_or("cptdbt".to_string()),
        user = std::env::var("CPTDB_TEST_USER").unwrap_or("cptdbt-user".to_string()),
        password = std::env::var("CPTDB_TEST_PASSWORD").unwrap_or("cptdbt-password".to_string()),
    )
}

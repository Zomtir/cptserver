use cptserver::db;
use cptserver::error::{Error, ErrorKind, Result};
use mysql::{Pool, PooledConn};
use std::path::PathBuf;
use std::sync::OnceLock;

static DBTPOOL: OnceLock<Pool> = OnceLock::new();

pub fn init_dbt_pool() -> Result<()> {
    let url = dbt_url();
    let pool = mysql::Pool::new(mysql::Opts::from_url(&url)?)?;
    let _ = DBTPOOL.set(pool);

    Ok(())
}

pub fn get_dbt_conn() -> Result<PooledConn> {
    let pool = DBTPOOL
        .get()
        .or_else(|| {
            let path_home = std::env::var("CPT_PATH_HOME").ok().map(|p| PathBuf::from(p));
            cptserver::fs::init_paths(path_home);
            init_dbt_pool().ok();
            println!("TEST_DB: Initialized database pool");
            let pool = DBTPOOL.get()?;
            println!("TEST_DB: Establshing database connection");
            let mut conn = pool.get_conn().ok()?;
            println!("TEST_DB: Migrating database scheme");
            db::migrate_scheme(&mut conn, &dbt_name()).ok()?;
            Some(pool)
        })
        .ok_or(Error::new(ErrorKind::Database, "Failed to get database pool"))?;

    pool.get_conn()
        .map_err(|_| Error::new(ErrorKind::Database, "Failed to get database connection"))
}

pub fn dbt_name() -> String {
    std::env::var("CPT_TEST_DB_DATABASE").unwrap_or("cptdbt".to_string())
}

pub fn dbt_url() -> String {
    format!(
        "mysql://{user}:{password}@{host}:{port}/{database}",
        host = std::env::var("CPT_TEST_DB_HOST").unwrap_or("localhost".to_string()),
        port = std::env::var("CPT_TEST_DB_PORT").unwrap_or("3306".to_string()),
        database = std::env::var("CPT_TEST_DB_DATABASE").unwrap_or("cptdbt".to_string()),
        user = std::env::var("CPT_TEST_DB_USER").unwrap_or("cptdbt-user".to_string()),
        password = std::env::var("CPT_TEST_DB_PASSWORD").unwrap_or("cptdbt-password".to_string()),
    )
}

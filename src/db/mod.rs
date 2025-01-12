pub mod club;
pub mod competence;
pub mod course;
pub mod event;
pub mod inventory;
pub mod location;
pub mod login;
pub mod organisation;
pub mod skill;
pub mod team;
pub mod user;

use crate::error::Error;
use mysql::prelude::Queryable;
use mysql::PooledConn;

static SCHEME_VERSION: u8 = 2;

pub fn get_version(conn: &mut PooledConn) -> Result<u8, Error> {
    let query_version = "SELECT version FROM _info;";
    let version: u8 = conn.query_first::<u8, _>(query_version)?.unwrap();
    Ok(version)
}

pub fn set_version(conn: &mut PooledConn, version: u8) -> Result<(), Error> {
    let query_version = format!("INSERT INTO _info (version) VALUES ({});", version);
    conn.query_drop(&query_version)?;
    Ok(())
}

pub fn migrate_scheme(conn: &mut PooledConn) -> Result<(), Error> {
    let latest_version: u8 = SCHEME_VERSION;

    // Check if the database has tables
    let query_empty = "SELECT COUNT(*) FROM information_schema.tables WHERE TABLE_TYPE = 'BASE TABLE';";
    let is_empty: bool = conn.query_first::<u8, _>(query_empty)?.unwrap() < 1;

    // Case 1: The database is empty and we do a fresh install
    if is_empty {
        let partial_path = format!("sql/schema_{}.sql", latest_version);
        let local_path = crate::common::fs::local_path(&partial_path)?;

        println!("DB: Fresh setup to version {}", latest_version);

        // Apply the schema
        let query_schema = std::fs::read_to_string(local_path).map_err(|_| Error::Default)?;
        conn.query_drop(&query_schema)?;

        // Insert the latest version number
        set_version(conn, latest_version)?;
    } else {
        // Check if schema info exists
        let query_info = "SELECT COUNT(*) FROM information_schema.tables WHERE table_name = '_info';";
        let has_info: bool = conn.query_first::<u8, _>(query_info)?.unwrap() > 0;

        // Case 2: Schema info is missing which is taken as indicator of schema version 0
        if !has_info {
            let partial_path = "sql/migrate_0.sql";
            let local_path = crate::common::fs::local_path(partial_path)?;

            // Run the script that makes the version 0 explicit
            let query_migrate0 = std::fs::read_to_string(local_path).map_err(|_| Error::Default)?;
            conn.query_drop(&query_migrate0)?;
        }

        // Case 3: Schema info exists or was set in case 2
        let mut current_version: u8 = get_version(conn)?;

        println!("DB: Migration from version {} to {}", current_version, latest_version);
        // Do incremental migrations
        while current_version < latest_version {
            let partial_path = format!("sql/migrate_{}.sql", current_version + 1);
            let local_path = crate::common::fs::local_path(&partial_path)?;

            // Apply the next migration script
            let query_migrate = std::fs::read_to_string(local_path).map_err(|_| Error::Default)?;
            conn.query_drop(query_migrate)?;
            current_version += 1;
        }
    }

    Ok(())
}

/// Helper function to debug SQL queries
///
/// ```
/// use mysql::params;
/// let query = "INSERT INTO spells (name, damage, mana, level) VALUES (:name, :damage, :mana, :level);";
/// let params = params! { "name" => "Fireball", "damage" => 12, "mana" => 5, "level" => 3 };
/// assert_eq!(cptserver::db::prep_sql(&query, &params).unwrap(), "INSERT INTO spells (name, damage, mana, level) VALUES ('Fireball', 12, 5, 3);".to_string());
/// ```
#[allow(dead_code)]
pub fn prep_sql(query: &str, params: &mysql::Params) -> Result<String, Error> {
    let pnp = mysql_common::named_params::ParsedNamedParams::parse(query.as_bytes()).unwrap();
    let (real_query, real_params) = (pnp.query(), pnp.params());

    let replacement_map = match params {
        mysql::Params::Named(map) => map,
        _ => return Err(Error::Default),
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

    Ok(output_string)
}

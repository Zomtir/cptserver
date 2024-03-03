PREPARATIONS
============

The server requires a database with the structure described `Database.sql`. The easiest way to setup is to setup a MariaDB database. Adapt the names to your liking.

```SQL
CREATE USER 'cptdb-user'@'localhost' IDENTIFIED BY 'cptdb-password';
CREATE DATABASE cptdb;
GRANT ALL PRIVILEGES ON cptdb.* TO 'cptdb-user'@'localhost';
```

Then import the `Database.sql` file via `phymyadmin` or use the CML. You will be promted to enter the 'cptdb-password'.

```BASH
mariadb -u cptdb-user -p cptdb < Database.sql
```

Finally, set up a config file called `CptServer.toml` for the server containing the database details. You can use `CptServer.template.toml` as template. `3306` is usually the default port.

db_server = 'localhost'
db_port = 3306
db_database = 'cptdb'
db_user = 'cptdb-user'
db_password = 'cptdb-password'


DEVELOPEMENT
============

```BASH
cargo build
cargo fmt
cargo clippy
cargo run
```

PRODUCTION
==========

```BASH
cargo build --release
```

The binary can be found at `./target/release/cptserver`.

TODO
====

- Change slots.status to "DRAFT,PENDING,ACCEPTED,REJECTED"  with another boolean slots.active
- Move server behaviour settings to the settings file
- Server action log file
- The response should contain what call stack did result in the repsonse? So that you can track what client action cause the response in shared code
    //Err(crate::Error::user_missing(origin.path()))
    use rocket::http::uri::Origin;
    pub fn user_login(origin: &Origin, credit: Json<Credential>)

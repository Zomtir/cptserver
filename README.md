About
=====

This is the server application for providing an API for the cptclient. CPT stands for Course Participation Tracker.

Deployment
==========

Database
--------

The server requires a suitable SQL database. The easiest way to setup is to setup a MariaDB database. Adapt the names to your liking. The program installs and updates the schema automatically.

```SQL
CREATE USER 'cptdb-user'@'localhost' IDENTIFIED BY 'cptdb-password';
CREATE DATABASE cptdb;
GRANT ALL PRIVILEGES ON cptdb.* TO 'cptdb-user'@'localhost';
```

Server config
-------------

You can configure various settings in the following priority (lowest to highest):

* Default setting
* Config file (.toml)
* Environment variables

If you plan to use `cptserver.toml` for the server settings, you can use `cptserver.template.toml` as template.

Best practice for developement is to add your environment variables your `~/.bashrc`. On the server they should be included in the `.service` system unit.

| ENVAR                    |      TOML           |  Default           |
|:---                      |:---                 |:---                |
| `CPT_PATH_HOME`          |  unavailable        | `$EXE`             |
| `CPT_PATH_CONFIG`        |  unavailable        | `$CPT_PATH_HOME/cptserver.toml` |
| `CPT_PATH_SQL`           |  unavailable        | `$CPT_PATH_HOME/sql/`           |
| `CPT_PATH_RESOURCES`     |  unavailable        | `$CPT_PATH_HOME/resources/`     |
| `CPT_PATH_DATA`          |  unavailable        | `$CPT_PATH_HOME/data/`          |
| `CPT_ROCKET_ADDRESS`     |  `rocket_address`   | `'127.0.0.1'`      |
| `CPT_ROCKET_PORT`        |  `rocket_port`      | `8000`             |
| `CPT_ROCKET_LOG_LEVEL`   |  `rocket_log_level` | `'Normal'`         |
| `CPT_DB_HOST`            |  `db_host`          | `'localhost'`      |
| `CPT_DB_PORT`            |  `db_port`          | `3306`             |
| `CPT_DB_DATABASE`        |  `db_database`      | `'cptdb'`          |
| `CPT_DB_USER`            |  `db_user`          | `'cptdb-user'`     |
| `CPT_DB_PASSWORD`        |  `db_password`      | `'cptdb-password'` |
| `CPT_APP_ADMIN`          |  `app_admin`        | unset              |


To create an initial admin user account, also include an `app_admin` in `cptserver.toml`. He has all right and requires no password. **Do remove this line as soon as you are done with the initial user/group setup.**

<details>
<summary>All settings</summary>

| ENVAR                                           | TOML                                        | Default            |
| :---------------------------------------------- | :------------------------------------------ | :----------------- |
| `CPT_APP_SESSION_DURATION_HOURS`                | `app_session_duration_hours`                | ?                  |
| `CPT_APP_EVENT_ACCEPTANCE_AUTO`                 | `app_event_acceptance_auto`                 | ?                  |
| `CPT_APP_EVENT_SEARCH_DATE_MIN_YEAR`            | `app_event_search_date_min_year`            | ?                  |
| `CPT_APP_EVENT_SEARCH_DATE_MAX_YEAR`            | `app_event_search_date_max_year`            | ?                  |
| `CPT_APP_EVENT_SEARCH_WINDOW_MIN_DAYS`          | `app_event_search_window_min_days`          | ?                  |
| `CPT_APP_EVENT_SEARCH_WINDOW_MAX_DAYS`          | `app_event_search_window_max_days`          | ?                  |
| `CPT_APP_EVENT_OCCURRENCE_DURATION_MIN_MINUTES` | `app_event_occurrence_duration_min_minutes` | ?                  |
| `CPT_APP_EVENT_OCCURRENCE_DURATION_MAX_DAYS`    | `app_event_occurrence_duration_max_days`    | ?                  |
| `CPT_APP_EVENT_OCCURRENCE_SNAP_MINUTES`         | `app_event_occurrence_snap_minutes`         | ?                  |
| `CPT_APP_EVENT_LOGIN_BUFFER_HOURS`              | `app_event_login_buffer_hours`              | ?                  |

</details>

Compiling and executing the application for developement is the usualy `cargo` workflow.

```bash
# Format your code if you made changes
cargo fmt
# Run a sanity check on your changes
cargo clippy
# Build the application
cargo build
# Run the application
cargo run
```

Testing
=======

If you want to `cargo test` the application, you have to set configure a few `ENVAR` beforehand. You also should use a dedicated test database and make the information available.

It is recommended to put your variables in `~/.bashrc`, a dedicated `TOML` test config is not available at this point.

<details>
<summary>All settings</summary>

| ENVAR                    |      TOML           |  Default           |
|:---                      |:---                 |:---                |
| `CPT_PATH_HOME`          | unavailable         | `$PWD`             |
| `CPT_TEST_DB_HOST`       | unavailable         | `'localhost'`      |
| `CPT_TEST_DB_PORT`       | unavailable         | `3306`             |
| `CPT_TEST_DB_DATABASE`   | unavailable         | `'cpttdb'`          |
| `CPT_TEST_DB_USER`       | unavailable         | `'cpttdb-user'`     |
| `CPT_TEST_DB_PASSWORD`   | unavailable         | `'cpttdb-password'` |

</details>

Releases
========

This is the targeted release procedure.

Collect the changes since the previous release and add them to the [CHANGELOG](CHANGELOG.md).
```
git log --format=%B v1.0.0..HEAD
```

The versioning scheme is `MAJOR.MINOR.PATCH`, increment the:
- MAJOR version when you make substantial API changes or core reworks
- MINOR version when you make any API changes
- PATCH version when you make backward compatible changes

Adapt the Cargo.toml file.

```
version = "1.1.1"
```

Update the database scheme version to version `X` in src/db.rs if neccessary, whereas `X` is the next increment from the previous one.

```
static SCHEME_VERSION : u8 = X;
```

Create an database update script under `sql/update_X.sql`.

Export the current schema from the database and save it as `sql/schema_X.sql`

Commit the changes and tag the commit.

```
git commit -m "Release v1.1.1"
git tag v1.1.1
```

Production
==========

To build a binary for production, choose a reliable version and build it with the `--release` flag.

```bash
cargo build --release
```

The binary can be found at `./target/release/cptserver`.

There is also a systemd unit file (`cptserver.service`) with the assumption that you have your binary
installed at `/opt/cptserver/`.

License
=======

The code is dedicated to the Public Domain as declared in the [License](LICENSE.md).

Contributing
============

Contributing to the project implies a copyright release according to the [Waiver](WAIVER.md) unless 
stated otherwise.

You are very welcome to explicitly state your approval with a simple statement such as
`Dedicated to Public Domain` in your patches. You can also sign the [Waiver](WAIVER.md) with GPG
while listing yourself as [Author](AUTHORS.md).

```bash
# Generate a GPG key
gpg --full-generate-key
# Optionally export your public key and add it to your Github account and/or a keyserver.
gpg --list-keys
gpg --armor --export <KEYID>
# Sign the waiver
gpg --detach-sig --armor WAIVER.md
# View the signature
cat WAIVER.md.asc
# Verify the signature
gpg --verify WAIVER.md.asc WAIVER.md
```


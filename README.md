About
=====

This is the server application for providing an API for the cptclient. CPT stands for Course Participation Tracker.

Deployment
==========

The server requires a suitable SQL database. The easiest way to setup is to setup a MariaDB database. Adapt the names to your liking. The program installs and updates the schema automatically.

```SQL
CREATE USER 'cptdb-user'@'localhost' IDENTIFIED BY 'cptdb-password';
CREATE DATABASE cptdb;
GRANT ALL PRIVILEGES ON cptdb.* TO 'cptdb-user'@'localhost';
```

Then set up a config file called `cptserver.toml` for the server containing the database details. You can use `cptserver.template.toml` as template. `3306` is usually the default port.

```
db_server = 'localhost'
db_port = 3306
db_database = 'cptdb'
db_user = 'cptdb-user'
db_password = 'cptdb-password'
```

To create an initial admin user account, also include an `cpt_admin` in `cptserver.toml`. He has all right and requires no password. **Do remove this line as soon as you are done with the initial user/group setup.**

```
cpt_admin = 'admin'
```

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

If you want to `cargo test` the application, you have to adapt the `cpt_test_env.sh` in the project root directory beforehand. It has to contain the correct path to just mentioned directory to get a hold of uncompiled files. Also it contains your test database information.

Either `source cpt_test_env.sh` to your current shell or put the content in your `~/.bashrc`.

```
export CPTSERVER_CONFIG=/home/user/development/cptserver

export CPTDB_TEST_SERVER="localhost"
export CPTDB_TEST_PORT=3306
export CPTDB_TEST_DATABASE="cptdbt"
export CPTDB_TEST_USER="cptdbt-user"
export CPTDB_TEST_PASSWORD="cptdbt-password"
```

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

Update the database scheme version to version `X` in src/db/mod.rs if neccessary, whereas `X` is the next increment from the previous one.

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


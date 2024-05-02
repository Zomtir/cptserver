ABOUT
=====

This is the server application for providing an API for the cptclient. CPT stands for Course Participation Tracker.

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

Finally, set up a config file called `cptserver.toml` for the server containing the database details. You can use `cptserver.template.toml` as template. `3306` is usually the default port.

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

```BASH
# Format your code if you made changes
cargo fmt
# Run a sanity check on your changes
cargo clippy
# Build the application
cargo build
# Run the application
cargo run
```

RELEASES
========

When you make a new release, this is the targeted procedure.

The versioning scheme is `MAJOR.MINOR.PATCH`, increment the:
- MAJOR version when you make substantial API changes or core reworks
- MINOR version when you make any API changes
- PATCH version when you make backward compatible changes

As long as the `MAJOR` release is `0.x`, it is considered a pre-release. The API and feature set
might be incomplete and unstable.

```
# Tag the commit with a release tag and a 'v' prefix.
# The `PATCH` version is omitted for the `.0` increment.
git tag v0.7

# Adapt the Cargo toml file
> version = "0.7.0"
```

PRODUCTION
==========

To build a binary for production, choose a reliable version and build it with the `--release` flag.

```BASH
cargo build --release
```

The binary can be found at `./target/release/cptserver`.

There is also a systemd unit file (`cptserver.service`) with the assumption that you have your binary
installed at `/opt/cptserver/`.

LICENSE
=======

The code is dedicated to the [Public Domain](LICENSE.md).

CONTRIBUTING
============

Contributing to the project implies a copyright release as stated in the [Waiver](WAIVER.md) unless 
stated otherwise.

You are very welcome to explicitly state your approval with a simple statement such as
`Dedicated to Public Domain` in your patches. You can also sign the [Waiver](WAIVER.md) with GPG
while listing yourself as [Author](AUTHORS.md).

```bash
# Generate a GPG key
gpg --full-generate-key
# Sign the waiver
gpg --no-version --armor --sign WAIVER.md
# Copy the signature
cat WAIVER.md.asc
# Optionally export your public key and add it to your Github account and/or a keyserver.
gpg --list-keys
gpg --armor --export <KEYID>
```

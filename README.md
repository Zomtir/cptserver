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

RELEASES
========

This is the targeted release procedure.

The versioning scheme is `MAJOR.MINOR.PATCH`, increment the:
- MAJOR version when you make substantial API changes or core reworks
- MINOR version when you make any API changes
- PATCH version when you make backward compatible changes

```
# Tag the commit with a release tag and a 'v' prefix.
git tag v1.1.1

# Adapt the Cargo toml file
> version = "1.1.1"
```

PRODUCTION
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


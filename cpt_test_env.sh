!#/bin/bash
# This script sets up the environment variables for the CPT test environment.
# Preferably put it in your ~/.bashrc

export CPTSERVER_CONFIG=/home/user/development/cptserver

export CPTDB_TEST_SERVER="localhost"
export CPTDB_TEST_PORT=3306
export CPTDB_TEST_DATABASE="cptdbt"
export CPTDB_TEST_USER="cptdbt-user"
export CPTDB_TEST_PASSWORD="cptdbt-password"

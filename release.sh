#! /bin/bash

RELEASE_DIR=./release

if [[ -z "$RELEASE_DIR" || "$RELEASE_DIR" == "/" ]]; then
    echo "Error: RELEASE_DIR is either empty or set to '/'. Exiting."
    exit 1
fi

cargo build --release

mkdir -p $RELEASE_DIR/SOURCES/
rm -r $RELEASE_DIR/SOURCES/*

cp {target/release/cptserver,cptserver.service,cptserver.template.toml} $RELEASE_DIR/SOURCES/
cp {README.md,LICENSE.md,CHANGELOG.md} $RELEASE_DIR/SOURCES/
cp -r resources $RELEASE_DIR/SOURCES/
cp -r sql $RELEASE_DIR/SOURCES/

cd $RELEASE_DIR/
tar -czvf cptserver.tar.gz ./SOURCES/

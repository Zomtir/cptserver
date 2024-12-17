#! /bin/bash

RELEASE_DIR=./release
RELEASE_VERSION="1.0.0"
RELEASE_ARCH="debian13-x86_64"

if [[ -z "$RELEASE_DIR" || "$RELEASE_DIR" == "/" ]]; then
    echo "Error: RELEASE_DIR is either empty or set to '/'. Exiting."
    exit 1
fi

RELEASE_NAME="cptserver-$RELEASE_VERSION-$RELEASE_ARCH"
mkdir -p $RELEASE_DIR/$RELEASE_NAME/

cargo build --release

cp {target/release/cptserver,cptserver.service,cptserver.template.toml} $RELEASE_DIR/$RELEASE_NAME/
cp {README.md,LICENSE.md,CHANGELOG.md} $RELEASE_DIR/$RELEASE_NAME/
cp -r resources $RELEASE_DIR/$RELEASE_NAME/
cp -r sql $RELEASE_DIR/$RELEASE_NAME/

cd $RELEASE_DIR/
tar -czvf cptserver-$RELEASE_VERSION-$RELEASE_ARCH.tar.gz $RELEASE_NAME/ 
